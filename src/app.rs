//! 统一执行入口：请求 -> 计算 -> 输出

use crate::arbitrage::{calculate_arbitrage, calculate_multi_arbitrage};
use crate::display::{
    print_result, print_result_arbitrage, print_result_arbitrage_json, print_result_json,
    print_result_multi_arbitrage, print_result_multi_arbitrage_json, print_result_polymarket,
    print_result_polymarket_json, print_result_portfolio, print_result_portfolio_json,
    print_result_stock, print_result_stock_json,
};
use crate::kelly::{build_stock_info, kelly_criterion, kelly_polymarket, kelly_stock};
use crate::portfolio::calculate_portfolio_kelly;
use crate::types::PortfolioLeg;

#[derive(Clone, Copy)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    pub fn is_json(self) -> bool {
        matches!(self, Self::Json)
    }
}

pub enum ModeRequest {
    Standard {
        odds: f64,
        win_rate: f64,
        capital: Option<f64>,
    },
    Polymarket {
        market_price: f64,
        your_probability: f64,
        capital: Option<f64>,
    },
    Stock {
        entry_price: f64,
        target_price: f64,
        stop_loss: f64,
        win_rate: f64,
        capital: Option<f64>,
    },
    Arbitrage {
        odds1: f64,
        odds2: f64,
        capital: Option<f64>,
    },
    MultiArbitrage {
        odds: Vec<f64>,
        capital: Option<f64>,
    },
    Portfolio {
        legs: Vec<PortfolioLeg>,
        capital: Option<f64>,
    },
}

pub fn execute_mode(mode: ModeRequest, output: OutputFormat) {
    match mode {
        ModeRequest::Standard {
            odds,
            win_rate,
            capital,
        } => {
            let result = kelly_criterion(odds, win_rate);
            if output.is_json() {
                print_result_json(odds, win_rate, &result, capital);
            } else {
                print_result(odds, win_rate, &result, capital);
            }
        }
        ModeRequest::Polymarket {
            market_price,
            your_probability,
            capital,
        } => {
            let result = kelly_polymarket(market_price, your_probability);
            if output.is_json() {
                print_result_polymarket_json(market_price, your_probability, &result, capital);
            } else {
                print_result_polymarket(market_price, your_probability, &result, capital);
            }
        }
        ModeRequest::Stock {
            entry_price,
            target_price,
            stop_loss,
            win_rate,
            capital,
        } => {
            let info = build_stock_info(entry_price, target_price, stop_loss);
            let result = kelly_stock(entry_price, target_price, stop_loss, win_rate);
            if output.is_json() {
                print_result_stock_json(&info, win_rate, &result, capital);
            } else {
                print_result_stock(&info, win_rate, &result, capital);
            }
        }
        ModeRequest::Arbitrage {
            odds1,
            odds2,
            capital,
        } => {
            let result = calculate_arbitrage(odds1, odds2);
            if output.is_json() {
                print_result_arbitrage_json(odds1, odds2, &result, capital);
            } else {
                print_result_arbitrage(odds1, odds2, &result, capital);
            }
        }
        ModeRequest::MultiArbitrage { odds, capital } => {
            let result = calculate_multi_arbitrage(&odds);
            if output.is_json() {
                print_result_multi_arbitrage_json(&odds, &result, capital);
            } else {
                print_result_multi_arbitrage(&odds, &result, capital);
            }
        }
        ModeRequest::Portfolio { legs, capital } => {
            let result = calculate_portfolio_kelly(&legs);
            if output.is_json() {
                print_result_portfolio_json(&legs, &result, capital);
            } else {
                print_result_portfolio(&legs, &result, capital);
            }
        }
    }
}
