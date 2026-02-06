//! 组合凯利（独立二项标的 / 相关情景）计算

use crate::types::{PortfolioKellyResult, PortfolioLeg, PortfolioScenario};

const MAX_TOTAL_ALLOCATION: f64 = 0.999_999;
const MAX_ITERATIONS: usize = 800;
const IMPROVEMENT_EPS: f64 = 1e-12;
const CONVERGENCE_OBJECTIVE_DELTA: f64 = 1e-10;
const STATE_PROB_EPS: f64 = 1e-15;

#[derive(Debug, Clone)]
struct OutcomeState {
    prob: f64,
    returns: Vec<f64>,
}

fn enumerate_independent_states(legs: &[PortfolioLeg]) -> Vec<OutcomeState> {
    let n = legs.len();
    let mut states = Vec::with_capacity(1 << n);

    for mask in 0..(1 << n) {
        let mut prob = 1.0;
        let mut returns = vec![0.0; n];
        for (i, leg) in legs.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                prob *= leg.win_prob;
                returns[i] = leg.win_return;
            } else {
                prob *= 1.0 - leg.win_prob;
                returns[i] = leg.loss_return;
            }
        }
        states.push(OutcomeState { prob, returns });
    }

    states
}

fn states_from_scenarios(leg_count: usize, scenarios: &[PortfolioScenario]) -> Vec<OutcomeState> {
    scenarios
        .iter()
        .map(|scenario| {
            assert!(
                scenario.probability.is_finite() && scenario.probability >= 0.0,
                "scenario probability must be finite and non-negative"
            );
            assert!(
                scenario.returns.len() == leg_count,
                "scenario returns length mismatch"
            );
            assert!(
                scenario.returns.iter().all(|r| r.is_finite()),
                "scenario return must be finite"
            );

            OutcomeState {
                prob: scenario.probability,
                returns: scenario.returns.clone(),
            }
        })
        .collect()
}

fn objective_and_gradient(allocations: &[f64], states: &[OutcomeState]) -> (f64, Vec<f64>) {
    let mut objective = 0.0;
    let mut gradient = vec![0.0; allocations.len()];

    for state in states {
        let wealth = 1.0
            + allocations
                .iter()
                .zip(state.returns.iter())
                .map(|(f, r)| f * r)
                .sum::<f64>();

        if wealth <= 0.0 {
            return (f64::NEG_INFINITY, gradient);
        }

        objective += state.prob * wealth.ln();
        for (i, ret) in state.returns.iter().enumerate() {
            gradient[i] += state.prob * ret / wealth;
        }
    }

    (objective, gradient)
}

fn state_wealth(allocations: &[f64], returns: &[f64]) -> f64 {
    1.0 + allocations
        .iter()
        .zip(returns.iter())
        .map(|(f, r)| f * r)
        .sum::<f64>()
}

fn expected_arithmetic_return(allocations: &[f64], states: &[OutcomeState]) -> f64 {
    states
        .iter()
        .map(|s| {
            s.prob
                * allocations
                    .iter()
                    .zip(&s.returns)
                    .map(|(f, r)| f * r)
                    .sum::<f64>()
        })
        .sum()
}

fn project_to_simplex(values: &[f64], cap: f64) -> Vec<f64> {
    let mut non_negative: Vec<f64> = values.iter().map(|v| v.max(0.0)).collect();
    let sum: f64 = non_negative.iter().sum();
    if sum <= cap {
        return non_negative;
    }

    let mut sorted = non_negative.clone();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let mut cumulative = 0.0;
    let mut theta = 0.0;
    for i in 0..sorted.len() {
        cumulative += sorted[i];
        let t = (cumulative - cap) / (i as f64 + 1.0);
        let next = sorted.get(i + 1).copied().unwrap_or(f64::NEG_INFINITY);
        if next <= t {
            theta = t;
            break;
        }
    }

    for v in &mut non_negative {
        *v = (*v - theta).max(0.0);
    }
    non_negative
}

fn single_leg_kelly_fraction(leg: &PortfolioLeg) -> f64 {
    let u = leg.win_return;
    let d = leg.loss_return;
    let p = leg.win_prob;
    let q = 1.0 - p;

    if !(u.is_finite() && d.is_finite() && p.is_finite()) {
        return 0.0;
    }
    if (u - d).abs() < 1e-12 {
        return if u > 0.0 { MAX_TOTAL_ALLOCATION } else { 0.0 };
    }
    if u.abs() < 1e-12 || d.abs() < 1e-12 {
        return 0.0;
    }

    let numerator = -(p * u + q * d);
    let denominator = u * d;
    let f = numerator / denominator;

    if !f.is_finite() || f <= 0.0 { 0.0 } else { f }
}

fn initial_allocations_independent(legs: &[PortfolioLeg]) -> Vec<f64> {
    let allocations: Vec<f64> = legs.iter().map(single_leg_kelly_fraction).collect();
    project_to_simplex(&allocations, MAX_TOTAL_ALLOCATION)
}

fn initial_allocations_correlated(leg_count: usize, states: &[OutcomeState]) -> Vec<f64> {
    let mut edges = vec![0.0; leg_count];
    for state in states {
        for (i, ret) in state.returns.iter().enumerate() {
            edges[i] += state.prob * ret;
        }
    }

    let non_negative: Vec<f64> = edges.into_iter().map(|e| e.max(0.0)).collect();
    project_to_simplex(&non_negative, MAX_TOTAL_ALLOCATION)
}

fn solve_with_states(
    leg_count: usize,
    states: &[OutcomeState],
    mut allocations: Vec<f64>,
) -> PortfolioKellyResult {
    if leg_count == 0 || states.is_empty() {
        return PortfolioKellyResult {
            allocations: vec![0.0; leg_count],
            total_allocation: 0.0,
            expected_log_growth: 0.0,
            expected_arithmetic_return: 0.0,
            worst_case_multiplier: 1.0,
            converged: true,
            iterations: 0,
        };
    }

    let mut step = 0.25;
    let mut iterations = 0usize;
    let mut converged = false;

    for _ in 0..MAX_ITERATIONS {
        iterations += 1;
        let (objective, gradient) = objective_and_gradient(&allocations, states);

        if !objective.is_finite() {
            break;
        }

        let mut improved = false;
        let mut accepted_improvement = 0.0;
        let mut local_step = step;

        for _ in 0..24 {
            let candidate: Vec<f64> = allocations
                .iter()
                .zip(gradient.iter())
                .map(|(f, g)| f + local_step * g)
                .collect();
            let projected = project_to_simplex(&candidate, MAX_TOTAL_ALLOCATION);
            let (next_objective, _) = objective_and_gradient(&projected, states);

            if next_objective > objective + IMPROVEMENT_EPS {
                accepted_improvement = next_objective - objective;
                allocations = projected;
                step = (local_step * 1.15).min(1.0);
                improved = true;
                break;
            }

            local_step *= 0.5;
            if local_step < 1e-10 {
                break;
            }
        }

        if !improved || accepted_improvement < CONVERGENCE_OBJECTIVE_DELTA {
            converged = true;
            break;
        }
    }

    let (expected_log_growth, _) = objective_and_gradient(&allocations, states);
    let total_allocation: f64 = allocations.iter().sum();
    let expected_arithmetic_return = expected_arithmetic_return(&allocations, states);

    let worst_case_multiplier = states
        .iter()
        .filter(|s| s.prob > STATE_PROB_EPS)
        .map(|s| state_wealth(&allocations, &s.returns))
        .fold(f64::INFINITY, f64::min);

    PortfolioKellyResult {
        allocations,
        total_allocation,
        expected_log_growth,
        expected_arithmetic_return,
        worst_case_multiplier: if worst_case_multiplier.is_finite() {
            worst_case_multiplier
        } else {
            0.0
        },
        converged,
        iterations,
    }
}

/// 计算独立二项标的的组合凯利仓位
pub fn calculate_portfolio_kelly(legs: &[PortfolioLeg]) -> PortfolioKellyResult {
    let states = enumerate_independent_states(legs);
    let allocations = initial_allocations_independent(legs);
    solve_with_states(legs.len(), &states, allocations)
}

/// 计算相关情景输入下的组合凯利仓位
pub fn calculate_portfolio_kelly_correlated(
    leg_count: usize,
    scenarios: &[PortfolioScenario],
) -> PortfolioKellyResult {
    let states = states_from_scenarios(leg_count, scenarios);
    let allocations = initial_allocations_correlated(leg_count, &states);
    solve_with_states(leg_count, &states, allocations)
}

#[cfg(test)]
mod tests {
    use super::{calculate_portfolio_kelly, calculate_portfolio_kelly_correlated};
    use crate::types::{PortfolioLeg, PortfolioLegSource, PortfolioScenario};

    fn leg(odds: f64, win_rate: f64) -> PortfolioLeg {
        PortfolioLeg {
            source: PortfolioLegSource::Standard,
            summary: format!("odds={odds},win={win_rate}"),
            win_prob: win_rate,
            win_return: odds - 1.0,
            loss_return: -1.0,
        }
    }

    #[test]
    fn symmetric_bets_have_symmetric_allocations() {
        let legs = vec![leg(2.0, 0.6), leg(2.0, 0.6)];
        let result = calculate_portfolio_kelly(&legs);
        let diff = (result.allocations[0] - result.allocations[1]).abs();
        assert!(diff < 1e-6);
        assert!(result.allocations[0] > 0.0);
    }

    #[test]
    fn negative_edge_bet_gets_near_zero_allocation() {
        let legs = vec![leg(2.0, 0.6), leg(2.0, 0.4)];
        let result = calculate_portfolio_kelly(&legs);
        assert!(result.allocations[0] > 0.0);
        assert!(result.allocations[1] < 1e-8);
    }

    #[test]
    fn total_allocation_respects_budget_constraint() {
        let legs = vec![leg(2.0, 0.6), leg(2.5, 0.5), leg(3.0, 0.4)];
        let result = calculate_portfolio_kelly(&legs);
        assert!(result.total_allocation < 1.0);
        assert!(result.worst_case_multiplier > 0.0);
    }

    #[test]
    fn stock_like_leg_is_supported() {
        let legs = vec![PortfolioLeg {
            source: PortfolioLegSource::Stock,
            summary: "entry=100,target=120,stop=90,win=60%".to_string(),
            win_prob: 0.6,
            win_return: 0.2,
            loss_return: -0.1,
        }];
        let result = calculate_portfolio_kelly(&legs);
        assert!(result.total_allocation > 0.0);
    }

    #[test]
    fn worst_case_multiplier_reflects_leg_loss_return() {
        let legs = vec![PortfolioLeg {
            source: PortfolioLegSource::Stock,
            summary: "entry=100,target=120,stop=90,win=60%".to_string(),
            win_prob: 0.6,
            win_return: 0.2,
            loss_return: -0.1,
        }];
        let result = calculate_portfolio_kelly(&legs);
        assert!(result.total_allocation > 0.95);
        assert!(result.worst_case_multiplier > 0.85);
        assert!(result.worst_case_multiplier <= 1.0);
    }

    #[test]
    fn deterministic_positive_leg_has_worst_case_above_one() {
        let legs = vec![PortfolioLeg {
            source: PortfolioLegSource::Arbitrage2,
            summary: "deterministic +5%".to_string(),
            win_prob: 1.0,
            win_return: 0.05,
            loss_return: 0.05,
        }];
        let result = calculate_portfolio_kelly(&legs);
        assert!(result.total_allocation > 0.95);
        assert!(result.worst_case_multiplier > 1.04);
    }

    #[test]
    fn worst_case_ignores_zero_probability_states() {
        let legs = vec![leg(2.0, 1.0), leg(2.0, 1.0)];
        let result = calculate_portfolio_kelly(&legs);
        assert!(result.worst_case_multiplier > 1.9);
    }

    #[test]
    fn correlated_joint_drawdown_reduces_total_allocation() {
        let scenarios = vec![
            PortfolioScenario {
                probability: 0.5,
                returns: vec![0.2, 0.2],
            },
            PortfolioScenario {
                probability: 0.5,
                returns: vec![-0.9, -0.9],
            },
        ];
        let result = calculate_portfolio_kelly_correlated(2, &scenarios);
        assert!(result.total_allocation < 0.5);
        assert!(result.allocations[0] >= 0.0);
        assert!(result.allocations[1] >= 0.0);
    }

    #[test]
    fn correlated_anti_correlation_allocates_to_both_legs() {
        let scenarios = vec![
            PortfolioScenario {
                probability: 0.5,
                returns: vec![0.2, -0.1],
            },
            PortfolioScenario {
                probability: 0.5,
                returns: vec![-0.1, 0.2],
            },
        ];
        let result = calculate_portfolio_kelly_correlated(2, &scenarios);
        assert!(result.allocations[0] > 0.2);
        assert!(result.allocations[1] > 0.2);
        let diff = (result.allocations[0] - result.allocations[1]).abs();
        assert!(diff < 1e-6);
    }

    #[test]
    fn correlated_zero_probability_scenario_is_ignored() {
        let scenarios = vec![
            PortfolioScenario {
                probability: 1.0,
                returns: vec![0.2],
            },
            PortfolioScenario {
                probability: 0.0,
                returns: vec![-0.9],
            },
        ];
        let result = calculate_portfolio_kelly_correlated(1, &scenarios);
        assert!(result.total_allocation > 0.95);
    }
}
