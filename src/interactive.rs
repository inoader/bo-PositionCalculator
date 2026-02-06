//! 交互式模式

use std::io::{self, Write};

use crate::arbitrage::{calculate_arbitrage, calculate_multi_arbitrage};
use crate::display::{
    print_result, print_result_arbitrage, print_result_multi_arbitrage, print_result_polymarket, print_result_stock,
    print_title, print_title_arbitrage, print_title_polymarket, print_title_stock, separator,
};
use crate::kelly::{build_stock_info, kelly_criterion, kelly_polymarket, kelly_stock};
use crate::validation::{parse_market_price, parse_odds, parse_percent, parse_positive};

/// 标准交互式模式
pub fn interactive() {
    print_title();

    loop {
        println!("请输入赔率 (如 2.0 表示 1赔1，输入 q 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut odds_input = String::new();
        io::stdin().read_line(&mut odds_input).unwrap();

        if odds_input.trim().to_lowercase() == "q" {
            println!("再见！");
            break;
        }

        let odds: f64 = match parse_odds(odds_input.trim(), "赔率") {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入胜率 (0-100，如 60 表示 60%):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut win_rate_input = String::new();
        io::stdin().read_line(&mut win_rate_input).unwrap();

        let win_rate = match parse_percent(win_rate_input.trim(), "胜率") {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入本金 (可选，直接回车跳过):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match parse_positive(capital_input.trim(), "本金") {
                Ok(n) => Some(n),
                Err(_) => {
                    println!("✗ 本金必须为正数，已跳过\n");
                    None
                }
            }
        };

        let result = kelly_criterion(odds, win_rate);
        print_result(odds, win_rate, &result, capital);
        println!();
    }
}

/// Polymarket 交互式
pub fn interactive_polymarket() {
    print_title_polymarket();

    loop {
        println!("请输入 Polymarket 市场价格 (0-100，如 60 表示 60c，输入 q 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut price_input = String::new();
        io::stdin().read_line(&mut price_input).unwrap();

        if price_input.trim().to_lowercase() == "q" {
            println!("再见！");
            break;
        }

        let market_price: f64 = match parse_market_price(price_input.trim()) {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入你估计的真实概率 (0-100):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut prob_input = String::new();
        io::stdin().read_line(&mut prob_input).unwrap();

        let your_probability: f64 = match parse_percent(prob_input.trim(), "概率") {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入本金 (可选，直接回车跳过):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match parse_positive(capital_input.trim(), "本金") {
                Ok(n) => Some(n),
                Err(_) => {
                    println!("✗ 本金必须为正数，已跳过\n");
                    None
                }
            }
        };

        let result = kelly_polymarket(market_price, your_probability);
        print_result_polymarket(market_price, your_probability, &result, capital);
        println!();
    }
}

/// 股票交互式
pub fn interactive_stock() {
    print_title_stock();

    loop {
        println!("请输入当前价 (输入 q 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut entry_input = String::new();
        io::stdin().read_line(&mut entry_input).unwrap();

        if entry_input.trim().to_lowercase() == "q" {
            println!("再见！");
            break;
        }

        let entry_price: f64 = match parse_positive(entry_input.trim(), "当前价") {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入止盈价:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut target_input = String::new();
        io::stdin().read_line(&mut target_input).unwrap();

        let target_price: f64 = match parse_positive(target_input.trim(), "止盈价") {
            Ok(n) if n > entry_price => n,
            Ok(_) => {
                println!("✗ 止盈价必须大于当前价\n");
                continue;
            }
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入止损价:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut stop_input = String::new();
        io::stdin().read_line(&mut stop_input).unwrap();

        let stop_loss: f64 = match parse_positive(stop_input.trim(), "止损价") {
            Ok(n) if n < entry_price => n,
            Ok(_) => {
                println!("✗ 止损价必须小于当前价\n");
                continue;
            }
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入胜率 (0-100):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut win_rate_input = String::new();
        io::stdin().read_line(&mut win_rate_input).unwrap();

        let win_rate = match parse_percent(win_rate_input.trim(), "胜率") {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入本金 (可选，直接回车跳过):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match parse_positive(capital_input.trim(), "本金") {
                Ok(n) => Some(n),
                Err(_) => {
                    println!("✗ 本金必须为正数，已跳过\n");
                    None
                }
            }
        };

        let info = build_stock_info(entry_price, target_price, stop_loss);
        let result = kelly_stock(entry_price, target_price, stop_loss, win_rate);
        print_result_stock(&info, win_rate, &result, capital);
        println!();
    }
}

/// 套利交互式
pub fn interactive_arbitrage() {
    print_title_arbitrage();

    loop {
        println!("请输入方案1的赔率 (输入 q 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut odds1_input = String::new();
        io::stdin().read_line(&mut odds1_input).unwrap();

        if odds1_input.trim().to_lowercase() == "q" {
            println!("再见！");
            break;
        }

        let odds1: f64 = match parse_odds(odds1_input.trim(), "赔率1") {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入方案2的赔率:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut odds2_input = String::new();
        io::stdin().read_line(&mut odds2_input).unwrap();

        let odds2: f64 = match parse_odds(odds2_input.trim(), "赔率2") {
            Ok(n) => n,
            Err(e) => {
                println!("✗ {}\n", e);
                continue;
            }
        };

        println!("请输入本金 (可选，直接回车跳过):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match parse_positive(capital_input.trim(), "本金") {
                Ok(n) => Some(n),
                Err(_) => {
                    println!("✗ 本金必须为正数，已跳过\n");
                    None
                }
            }
        };

        let result = calculate_arbitrage(odds1, odds2);
        print_result_arbitrage(odds1, odds2, &result, capital);
        println!();
    }
}

/// 多标的套利交互式
pub fn interactive_multi_arbitrage() {
    separator();
    println!("                        多标的套利计算器");
    separator();
    println!();

    loop {
        println!("请输入标的数量 (输入 q 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut count_input = String::new();
        io::stdin().read_line(&mut count_input).unwrap();

        if count_input.trim().to_lowercase() == "q" {
            println!("再见！");
            break;
        }

        let count: usize = match count_input.trim().parse() {
            Ok(n) if n >= 2 => n,
            Ok(_) => {
                println!("✗ 标的数量必须至少为 2\n");
                continue;
            }
            Err(_) => {
                println!("✗ 无效输入\n");
                continue;
            }
        };

        let mut odds = Vec::new();
        'outer: loop {
            for i in (odds.len() + 1)..=count {
                println!("请输入标的{}的赔率:", i);
                print!("> ");
                io::stdout().flush().unwrap();

                let mut odds_input = String::new();
                io::stdin().read_line(&mut odds_input).unwrap();

                let o: f64 = match parse_odds(odds_input.trim(), "赔率") {
                    Ok(n) => n,
                    Err(e) => {
                        println!("✗ {}\n", e);
                        continue 'outer;
                    }
                };
                odds.push(o);
            }
            break;
        }

        println!("请输入本金 (可选，直接回车跳过):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match parse_positive(capital_input.trim(), "本金") {
                Ok(n) => Some(n),
                Err(_) => {
                    println!("✗ 本金必须为正数，已跳过\n");
                    None
                }
            }
        };

        let result = calculate_multi_arbitrage(&odds);
        print_result_multi_arbitrage(&odds, &result, capital);
        println!();
    }
}
