//! 2x2 双人博弈纳什均衡计算

use crate::types::{NashMixedEquilibrium, NashPureEquilibrium, NashResult};

const EPS: f64 = 1e-10;

fn nearly_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPS
}

fn in_unit_interval(v: f64) -> bool {
    v >= -EPS && v <= 1.0 + EPS
}

fn clamp01(v: f64) -> f64 {
    if nearly_eq(v, 0.0) {
        0.0
    } else if nearly_eq(v, 1.0) {
        1.0
    } else {
        v.clamp(0.0, 1.0)
    }
}

fn expected_payoff(matrix: [[f64; 2]; 2], p: f64, q: f64) -> f64 {
    p * q * matrix[0][0]
        + p * (1.0 - q) * matrix[0][1]
        + (1.0 - p) * q * matrix[1][0]
        + (1.0 - p) * (1.0 - q) * matrix[1][1]
}

fn find_pure_equilibria(
    row_payoffs: [[f64; 2]; 2],
    col_payoffs: [[f64; 2]; 2],
) -> Vec<NashPureEquilibrium> {
    let mut pure = Vec::new();

    for i in 0..2 {
        for j in 0..2 {
            let row_best_response = row_payoffs[i][j] >= row_payoffs[1 - i][j] - EPS;
            let col_best_response = col_payoffs[i][j] >= col_payoffs[i][1 - j] - EPS;

            if row_best_response && col_best_response {
                pure.push(NashPureEquilibrium {
                    row_strategy: i,
                    col_strategy: j,
                    row_payoff: row_payoffs[i][j],
                    col_payoff: col_payoffs[i][j],
                });
            }
        }
    }

    pure
}

fn find_mixed_equilibrium(
    row_payoffs: [[f64; 2]; 2],
    col_payoffs: [[f64; 2]; 2],
) -> Option<NashMixedEquilibrium> {
    // 列玩家左策略概率 q，使行玩家上下无差异
    let row_denom = row_payoffs[0][0] - row_payoffs[0][1] - row_payoffs[1][0] + row_payoffs[1][1];
    // 行玩家上策略概率 p，使列玩家左右无差异
    let col_denom = col_payoffs[0][0] - col_payoffs[1][0] - col_payoffs[0][1] + col_payoffs[1][1];

    if row_denom.abs() <= EPS || col_denom.abs() <= EPS {
        return None;
    }

    let q = (row_payoffs[1][1] - row_payoffs[0][1]) / row_denom;
    let p = (col_payoffs[1][1] - col_payoffs[1][0]) / col_denom;

    if !in_unit_interval(p) || !in_unit_interval(q) {
        return None;
    }

    let p = clamp01(p);
    let q = clamp01(q);

    Some(NashMixedEquilibrium {
        row_top_prob: p,
        col_left_prob: q,
        row_expected_payoff: expected_payoff(row_payoffs, p, q),
        col_expected_payoff: expected_payoff(col_payoffs, p, q),
    })
}

/// 计算 2x2 双人博弈纳什均衡
pub fn calculate_nash_2x2(row_payoffs: [[f64; 2]; 2], col_payoffs: [[f64; 2]; 2]) -> NashResult {
    let pure_equilibria = find_pure_equilibria(row_payoffs, col_payoffs);
    let mixed_equilibrium = find_mixed_equilibrium(row_payoffs, col_payoffs);

    NashResult {
        pure_equilibria,
        mixed_equilibrium,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_nash_2x2;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-8
    }

    #[test]
    fn prison_dilemma_has_single_pure_equilibrium() {
        // (上=合作, 下=背叛; 左=合作, 右=背叛)
        let row = [[3.0, 0.0], [5.0, 1.0]];
        let col = [[3.0, 5.0], [0.0, 1.0]];

        let result = calculate_nash_2x2(row, col);
        assert_eq!(result.pure_equilibria.len(), 1);
        let eq = &result.pure_equilibria[0];
        assert_eq!(eq.row_strategy, 1);
        assert_eq!(eq.col_strategy, 1);
    }

    #[test]
    fn coordination_game_has_multiple_pure_equilibria() {
        let row = [[4.0, 0.0], [0.0, 2.0]];
        let col = [[4.0, 0.0], [0.0, 2.0]];

        let result = calculate_nash_2x2(row, col);
        assert_eq!(result.pure_equilibria.len(), 2);
    }

    #[test]
    fn matching_pennies_has_only_mixed_equilibrium() {
        let row = [[1.0, -1.0], [-1.0, 1.0]];
        let col = [[-1.0, 1.0], [1.0, -1.0]];

        let result = calculate_nash_2x2(row, col);
        assert!(result.pure_equilibria.is_empty());

        let mixed = result
            .mixed_equilibrium
            .expect("mixed equilibrium expected");
        assert!(approx(mixed.row_top_prob, 0.5));
        assert!(approx(mixed.col_left_prob, 0.5));
        assert!(approx(mixed.row_expected_payoff, 0.0));
        assert!(approx(mixed.col_expected_payoff, 0.0));
    }

    #[test]
    fn no_internal_mixed_when_probability_out_of_range() {
        let row = [[3.0, 2.0], [1.0, 0.0]];
        let col = [[1.0, 2.0], [3.0, 4.0]];

        let result = calculate_nash_2x2(row, col);
        assert!(result.mixed_equilibrium.is_none());
    }
}
