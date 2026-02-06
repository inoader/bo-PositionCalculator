//! 交互式模式

use std::io::{self, Write};

use crate::app::{ModeRequest, OutputFormat, execute_mode};
use crate::display::{
    print_title, print_title_arbitrage, print_title_nash, print_title_polymarket,
    print_title_portfolio, print_title_stock, separator,
};
use crate::portfolio_input::parse_portfolio_leg_descriptor;
use crate::validation::{parse_f64, parse_market_price, parse_odds, parse_percent, parse_positive};

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

        execute_mode(
            ModeRequest::Standard {
                odds,
                win_rate,
                capital,
            },
            OutputFormat::Text,
        );
        println!();
    }
}

/// Polymarket 交互式
pub fn interactive_polymarket() {
    print_title_polymarket();

    loop {
        println!("请输入 Polymarket 市场价格 ((0,100)，如 60 表示 60c，输入 q 退出):");
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

        execute_mode(
            ModeRequest::Polymarket {
                market_price,
                your_probability,
                capital,
            },
            OutputFormat::Text,
        );
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

        execute_mode(
            ModeRequest::Stock {
                entry_price,
                target_price,
                stop_loss,
                win_rate,
                capital,
            },
            OutputFormat::Text,
        );
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

        execute_mode(
            ModeRequest::Arbitrage {
                odds1,
                odds2,
                capital,
            },
            OutputFormat::Text,
        );
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

        execute_mode(
            ModeRequest::MultiArbitrage { odds, capital },
            OutputFormat::Text,
        );
        println!();
    }
}

/// 纳什均衡交互式（2x2）
pub fn interactive_nash() {
    print_title_nash();

    loop {
        println!("请输入 8 个收益值: a11 a12 a21 a22 b11 b12 b21 b22 (输入 q 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();

        if trimmed.to_lowercase() == "q" {
            println!("再见！");
            break;
        }

        let fields: Vec<&str> = trimmed.split_whitespace().collect();
        if fields.len() != 8 {
            println!("✗ 请输入 8 个数字，示例: 3 0 5 1 3 5 0 1\n");
            continue;
        }

        let labels = ["a11", "a12", "a21", "a22", "b11", "b12", "b21", "b22"];
        let mut values = [0.0_f64; 8];
        let mut failed = false;

        for i in 0..8 {
            match parse_f64(fields[i], labels[i]) {
                Ok(v) => values[i] = v,
                Err(e) => {
                    println!("✗ {}\n", e);
                    failed = true;
                    break;
                }
            }
        }

        if failed {
            continue;
        }

        execute_mode(
            ModeRequest::Nash {
                row_payoffs: [[values[0], values[1]], [values[2], values[3]]],
                col_payoffs: [[values[4], values[5]], [values[6], values[7]]],
            },
            OutputFormat::Text,
        );
        println!();
    }
}

/// 组合凯利交互式
pub fn interactive_portfolio() {
    print_title_portfolio();

    loop {
        println!("请输入标的数量 (2-12，输入 q 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut count_input = String::new();
        io::stdin().read_line(&mut count_input).unwrap();

        if count_input.trim().to_lowercase() == "q" {
            println!("再见！");
            break;
        }

        let count: usize = match count_input.trim().parse() {
            Ok(n) if (2..=12).contains(&n) => n,
            Ok(_) => {
                println!("✗ 标的数量必须在 2-12 之间\n");
                continue;
            }
            Err(_) => {
                println!("✗ 无效输入\n");
                continue;
            }
        };

        let mut bets = Vec::with_capacity(count);
        'outer: loop {
            for i in (bets.len() + 1)..=count {
                println!(
                    "请输入标的{}描述 (std:2.0:60 / pm:60:75 / stock:100:120:90:60 / arb:2.1:2.2 / marb:2.5,4.0,5.0):",
                    i
                );
                print!("> ");
                io::stdout().flush().unwrap();

                let mut descriptor_input = String::new();
                io::stdin().read_line(&mut descriptor_input).unwrap();
                let bet = match parse_portfolio_leg_descriptor(descriptor_input.trim()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("✗ {}\n", e);
                        continue 'outer;
                    }
                };

                bets.push(bet);
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

        execute_mode(
            ModeRequest::Portfolio {
                legs: bets,
                capital,
            },
            OutputFormat::Text,
        );
        println!();
    }
}
