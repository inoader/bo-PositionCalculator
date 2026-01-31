//! 凯利公式计算
//! f* = (bp - q) / b
//! 其中 b 为赔率-1，p 为胜率，q = 1-p

use crate::types::{KellyResult, StockInfo};

/// 标准凯利公式计算
pub fn kelly_criterion(odds: f64, win_rate: f64) -> KellyResult {
    let b = odds - 1.0;
    let p = win_rate;
    let q = 1.0 - p;

    let optimal_fraction = (b * p - q) / b;
    let expected_value = p * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: expected_value > 0.0,
        expected_value,
    }
}

/// Polymarket 市场凯利公式计算
pub fn kelly_polymarket(market_price: f64, your_probability: f64) -> KellyResult {
    let p_market = market_price;
    let p_your = your_probability;

    let b = (1.0 - p_market) / p_market;
    let q = 1.0 - p_your;

    let optimal_fraction = (b * p_your - q) / b;
    let expected_value = p_your * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: expected_value > 0.0,
        expected_value,
    }
}

/// 股票交易凯利公式计算
pub fn kelly_stock(entry_price: f64, target_price: f64, stop_loss: f64, win_rate: f64) -> KellyResult {
    let profit = target_price - entry_price;
    let risk = entry_price - stop_loss;
    let b = profit / risk;

    let p = win_rate;
    let q = 1.0 - p;

    let optimal_fraction = (b * p - q) / b;
    let expected_value = p * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: expected_value > 0.0,
        expected_value,
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
