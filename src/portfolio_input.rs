//! 组合凯利输入转换（各模式 -> 统一组合腿）

use crate::arbitrage::{calculate_arbitrage, calculate_multi_arbitrage};
use crate::types::PortfolioLeg;
use crate::validation::{parse_market_price, parse_odds, parse_percent, parse_positive};

fn pct(v: f64) -> String {
    format!("{:.2}%", v * 100.0)
}

pub fn build_standard_leg(odds: f64, win_rate: f64) -> PortfolioLeg {
    PortfolioLeg {
        source: "standard".to_string(),
        summary: format!("赔率 {:.3} / 胜率 {}", odds, pct(win_rate)),
        win_prob: win_rate,
        win_return: odds - 1.0,
        loss_return: -1.0,
    }
}

pub fn build_polymarket_leg(market_price: f64, your_probability: f64) -> PortfolioLeg {
    let odds = 1.0 / market_price;
    PortfolioLeg {
        source: "polymarket".to_string(),
        summary: format!(
            "价格 {:.3}% / 概率 {}",
            market_price * 100.0,
            pct(your_probability)
        ),
        win_prob: your_probability,
        win_return: odds - 1.0,
        loss_return: -1.0,
    }
}

pub fn build_stock_leg(
    entry_price: f64,
    target_price: f64,
    stop_loss: f64,
    win_rate: f64,
) -> Result<PortfolioLeg, String> {
    if target_price <= entry_price || stop_loss >= entry_price {
        return Err("参数错误: 止盈价必须大于当前价，止损价必须小于当前价".to_string());
    }

    let win_return = (target_price - entry_price) / entry_price;
    let loss_return = -(entry_price - stop_loss) / entry_price;

    Ok(PortfolioLeg {
        source: "stock".to_string(),
        summary: format!(
            "入场 {:.2} / 止盈 {:.2} / 止损 {:.2} / 胜率 {}",
            entry_price,
            target_price,
            stop_loss,
            pct(win_rate)
        ),
        win_prob: win_rate,
        win_return,
        loss_return,
    })
}

pub fn build_arbitrage_two_leg(odds1: f64, odds2: f64) -> PortfolioLeg {
    let result = calculate_arbitrage(odds1, odds2);
    let r = if result.has_arbitrage {
        result.arbitrage_profit
    } else {
        -result.juice_rate
    };

    PortfolioLeg {
        source: "arbitrage2".to_string(),
        summary: format!(
            "双边赔率 {:.3}/{:.3} / {}",
            odds1,
            odds2,
            if result.has_arbitrage {
                format!("套利 {:.2}%", result.arbitrage_profit * 100.0)
            } else {
                format!("抽水 {:.2}%", result.juice_rate * 100.0)
            }
        ),
        win_prob: 1.0,
        win_return: r,
        loss_return: r,
    }
}

pub fn build_arbitrage_multi_leg(odds: &[f64]) -> PortfolioLeg {
    let result = calculate_multi_arbitrage(odds);
    let r = if result.has_arbitrage {
        result.arbitrage_profit
    } else {
        -result.juice_rate
    };

    PortfolioLeg {
        source: "arbitrageN".to_string(),
        summary: format!(
            "多边赔率 {} / {}",
            odds.iter()
                .map(|o| format!("{:.3}", o))
                .collect::<Vec<String>>()
                .join(","),
            if result.has_arbitrage {
                format!("套利 {:.2}%", result.arbitrage_profit * 100.0)
            } else {
                format!("抽水 {:.2}%", result.juice_rate * 100.0)
            }
        ),
        win_prob: 1.0,
        win_return: r,
        loss_return: r,
    }
}

/// 解析组合腿描述:
/// - `std:赔率:胜率`
/// - `pm:市场价格:你的概率`
/// - `stock:当前价:止盈价:止损价:胜率`
/// - `arb:赔率1:赔率2`
/// - `marb:赔率1,赔率2,...`
pub fn parse_portfolio_leg_descriptor(token: &str) -> Result<PortfolioLeg, String> {
    let parts: Vec<&str> = token.split(':').collect();
    if parts.is_empty() {
        return Err("组合标的描述不能为空".to_string());
    }

    match parts[0].to_lowercase().as_str() {
        "std" | "standard" => {
            if parts.len() != 3 {
                return Err("标准标的格式错误，应为 std:赔率:胜率".to_string());
            }
            let odds = parse_odds(parts[1], "赔率")?;
            let win_rate = parse_percent(parts[2], "胜率")?;
            Ok(build_standard_leg(odds, win_rate))
        }
        "pm" | "polymarket" => {
            if parts.len() != 3 {
                return Err("Polymarket 标的格式错误，应为 pm:市场价格:你的概率".to_string());
            }
            let market_price = parse_market_price(parts[1])?;
            let your_prob = parse_percent(parts[2], "你的概率")?;
            Ok(build_polymarket_leg(market_price, your_prob))
        }
        "stock" | "stk" => {
            if parts.len() != 5 {
                return Err("股票标的格式错误，应为 stock:当前价:止盈价:止损价:胜率".to_string());
            }
            let entry = parse_positive(parts[1], "当前价")?;
            let target = parse_positive(parts[2], "止盈价")?;
            let stop = parse_positive(parts[3], "止损价")?;
            let win_rate = parse_percent(parts[4], "胜率")?;
            build_stock_leg(entry, target, stop, win_rate)
        }
        "arb" => {
            if parts.len() != 3 {
                return Err("套利标的格式错误，应为 arb:赔率1:赔率2".to_string());
            }
            let odds1 = parse_odds(parts[1], "赔率1")?;
            let odds2 = parse_odds(parts[2], "赔率2")?;
            Ok(build_arbitrage_two_leg(odds1, odds2))
        }
        "marb" => {
            if parts.len() != 2 {
                return Err("多边套利标的格式错误，应为 marb:赔率1,赔率2,...".to_string());
            }
            let raw = parts[1];
            let mut odds = Vec::new();
            for (i, item) in raw.split(',').enumerate() {
                let o = parse_odds(item.trim(), &format!("赔率{}", i + 1))?;
                odds.push(o);
            }
            if odds.len() < 2 {
                return Err("marb 至少需要 2 个赔率".to_string());
            }
            Ok(build_arbitrage_multi_leg(&odds))
        }
        _ => Err("不支持的组合标的类型，支持 std/pm/stock/arb/marb".to_string()),
    }
}
