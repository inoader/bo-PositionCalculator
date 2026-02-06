//! 套利机会计算

use crate::types::{ArbitrageResult, MultiArbitrageResult};

/// 计算套利机会（两个标的）
/// 输入两边的赔率，返回套利方案
pub fn calculate_arbitrage(odds1: f64, odds2: f64) -> ArbitrageResult {
    let implied_prob1 = 1.0 / odds1;
    let implied_prob2 = 1.0 / odds2;
    let total_implied_prob = implied_prob1 + implied_prob2;

    let has_arbitrage = total_implied_prob < 1.0;

    if has_arbitrage {
        // 套利收益率 = (1 / 总隐含概率) - 1
        let arbitrage_profit = (1.0 / total_implied_prob) - 1.0;

        // 最优投注比例分配
        // 投注1 = 赔率2 / (赔率1 + 赔率2)
        // 投注2 = 赔率1 / (赔率1 + 赔率2)
        let total_odds = odds1 + odds2;
        let stake1_ratio = odds2 / total_odds;
        let stake2_ratio = odds1 / total_odds;

        ArbitrageResult {
            has_arbitrage: true,
            total_implied_prob,
            arbitrage_profit,
            juice_rate: 0.0,
            stake1_ratio,
            stake2_ratio,
        }
    } else {
        // 抽水率 = 总隐含概率 - 1
        let juice_rate = total_implied_prob - 1.0;

        ArbitrageResult {
            has_arbitrage: false,
            total_implied_prob,
            arbitrage_profit: 0.0,
            juice_rate,
            stake1_ratio: 0.0,
            stake2_ratio: 0.0,
        }
    }
}

/// 计算多标的套利机会
/// 输入多个赔率，返回套利方案
pub fn calculate_multi_arbitrage(odds: &[f64]) -> MultiArbitrageResult {
    let total_implied_prob: f64 = odds.iter().map(|&o| 1.0 / o).sum();
    let has_arbitrage = total_implied_prob < 1.0;

    if has_arbitrage {
        let arbitrage_profit = (1.0 / total_implied_prob) - 1.0;

        // 各标的投注比例 = (1 / 该标的赔率) / 总隐含概率
        let stake_ratios: Vec<f64> = odds.iter().map(|&o| (1.0 / o) / total_implied_prob).collect();

        MultiArbitrageResult {
            has_arbitrage: true,
            total_implied_prob,
            arbitrage_profit,
            juice_rate: 0.0,
            stake_ratios,
        }
    } else {
        // 抽水率 = 总隐含概率 - 1
        let juice_rate = total_implied_prob - 1.0;

        MultiArbitrageResult {
            has_arbitrage: false,
            total_implied_prob,
            arbitrage_profit: 0.0,
            juice_rate,
            stake_ratios: vec![0.0; odds.len()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{calculate_arbitrage, calculate_multi_arbitrage};

    const EPS: f64 = 1e-10;

    fn assert_almost_eq(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < EPS, "actual={actual}, expected={expected}");
    }

    #[test]
    fn two_way_arbitrage_detects_opportunity_and_splits_stake() {
        let result = calculate_arbitrage(2.1, 2.1);
        assert!(result.has_arbitrage);
        assert_almost_eq(result.total_implied_prob, 2.0 / 2.1);
        assert_almost_eq(result.arbitrage_profit, (1.0 / result.total_implied_prob) - 1.0);
        assert_almost_eq(result.stake1_ratio, 0.5);
        assert_almost_eq(result.stake2_ratio, 0.5);
    }

    #[test]
    fn two_way_arbitrage_detects_no_opportunity() {
        let result = calculate_arbitrage(1.9, 1.9);
        assert!(!result.has_arbitrage);
        assert!(result.total_implied_prob > 1.0);
        assert!(result.juice_rate > 0.0);
        assert_almost_eq(result.arbitrage_profit, 0.0);
    }

    #[test]
    fn multi_way_arbitrage_stake_ratios_sum_to_one() {
        let odds = [2.5, 3.6, 4.2];
        let result = calculate_multi_arbitrage(&odds);
        assert!(result.has_arbitrage);
        let sum: f64 = result.stake_ratios.iter().sum();
        assert_almost_eq(sum, 1.0);
    }

    #[test]
    fn multi_way_arbitrage_detects_juice() {
        let odds = [1.8, 2.0, 4.0];
        let result = calculate_multi_arbitrage(&odds);
        assert!(!result.has_arbitrage);
        assert!(result.total_implied_prob > 1.0);
        assert!(result.juice_rate > 0.0);
        assert!(result.stake_ratios.iter().all(|&r| r == 0.0));
    }
}
