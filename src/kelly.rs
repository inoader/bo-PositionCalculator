//! 凯利公式计算
//! f* = (bp - q) / b
//! 其中 b 为赔率-1，p 为胜率，q = 1-p

use crate::types::{KellyGrowthRates, KellyResult, StockInfo};

fn geometric_expected_return(win_prob: f64, net_odds: f64, fraction: f64) -> f64 {
    if fraction <= 0.0 {
        return 0.0;
    }

    let loss_prob = 1.0 - win_prob;
    let win_multiplier = 1.0 + fraction * net_odds;
    let loss_multiplier = 1.0 - fraction;

    if win_multiplier <= 0.0 || loss_multiplier < 0.0 {
        return f64::NAN;
    }

    if loss_multiplier == 0.0 {
        return if loss_prob > 0.0 {
            -1.0
        } else {
            win_multiplier - 1.0
        };
    }

    (win_prob * win_multiplier.ln() + loss_prob * loss_multiplier.ln()).exp() - 1.0
}

fn growth_rates(win_prob: f64, net_odds: f64, optimal_fraction: f64) -> KellyGrowthRates {
    KellyGrowthRates {
        full_kelly: geometric_expected_return(win_prob, net_odds, optimal_fraction),
        half_kelly: geometric_expected_return(win_prob, net_odds, optimal_fraction * 0.5),
        quarter_kelly: geometric_expected_return(win_prob, net_odds, optimal_fraction * 0.25),
    }
}

/// 标准凯利公式计算
pub fn kelly_criterion(odds: f64, win_rate: f64) -> KellyResult {
    let b = odds - 1.0;
    let p = win_rate;
    let q = 1.0 - p;

    let optimal_fraction = (b * p - q) / b;
    let arithmetic_expected_return = p * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: arithmetic_expected_return > 0.0,
        arithmetic_expected_return,
        geometric_expected_return: growth_rates(p, b, optimal_fraction),
    }
}

/// Polymarket 市场凯利公式计算
pub fn kelly_polymarket(market_price: f64, your_probability: f64) -> KellyResult {
    let p_market = market_price;
    let p_your = your_probability;

    let b = (1.0 - p_market) / p_market;
    let q = 1.0 - p_your;

    let optimal_fraction = (b * p_your - q) / b;
    let arithmetic_expected_return = p_your * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: arithmetic_expected_return > 0.0,
        arithmetic_expected_return,
        geometric_expected_return: growth_rates(p_your, b, optimal_fraction),
    }
}

/// 股票交易凯利公式计算
pub fn kelly_stock(
    entry_price: f64,
    target_price: f64,
    stop_loss: f64,
    win_rate: f64,
) -> KellyResult {
    let profit = target_price - entry_price;
    let risk = entry_price - stop_loss;
    let b = profit / risk;

    let p = win_rate;
    let q = 1.0 - p;

    let optimal_fraction = (b * p - q) / b;
    let arithmetic_expected_return = p * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: arithmetic_expected_return > 0.0,
        arithmetic_expected_return,
        geometric_expected_return: growth_rates(p, b, optimal_fraction),
    }
}

/// 构建股票交易信息
pub fn build_stock_info(entry_price: f64, target_price: f64, stop_loss: f64) -> StockInfo {
    let profit = target_price - entry_price;
    let risk = entry_price - stop_loss;
    let ratio = profit / risk;

    StockInfo {
        entry_price,
        target_price,
        stop_loss,
        profit,
        risk,
        ratio,
    }
}

#[cfg(test)]
mod tests {
    use super::{build_stock_info, kelly_criterion, kelly_polymarket, kelly_stock};

    const EPS: f64 = 1e-10;

    fn assert_almost_eq(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < EPS,
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn standard_kelly_calculation_is_correct() {
        let result = kelly_criterion(2.0, 0.6);
        assert_almost_eq(result.optimal_fraction, 0.2);
        assert_almost_eq(result.arithmetic_expected_return, 0.2);
        assert_almost_eq(result.geometric_expected_return.full_kelly, 0.0203396005);
        assert_almost_eq(result.geometric_expected_return.half_kelly, 0.0151556004);
        assert_almost_eq(result.geometric_expected_return.quarter_kelly, 0.0087952335);
        assert!(result.positive_ev);
    }

    #[test]
    fn polymarket_kelly_calculation_is_correct() {
        let result = kelly_polymarket(0.6, 0.75);
        assert_almost_eq(result.optimal_fraction, 0.375);
        assert_almost_eq(result.arithmetic_expected_return, 0.25);
        assert!(result.positive_ev);
    }

    #[test]
    fn stock_kelly_calculation_is_correct() {
        let result = kelly_stock(100.0, 120.0, 90.0, 0.6);
        assert_almost_eq(result.optimal_fraction, 0.4);
        assert_almost_eq(result.arithmetic_expected_return, 0.8);
        assert!(result.positive_ev);
    }

    #[test]
    fn stock_info_ratio_is_correct() {
        let info = build_stock_info(100.0, 120.0, 90.0);
        assert_almost_eq(info.profit, 20.0);
        assert_almost_eq(info.risk, 10.0);
        assert_almost_eq(info.ratio, 2.0);
    }

    #[test]
    fn negative_ev_sets_non_positive_flag() {
        let result = kelly_criterion(2.0, 0.4);
        assert!(result.arithmetic_expected_return < 0.0);
        assert_almost_eq(result.geometric_expected_return.full_kelly, 0.0);
        assert!(!result.positive_ev);
        assert!(result.optimal_fraction <= 0.0);
    }
}
