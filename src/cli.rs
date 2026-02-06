//! CLI 命令行模式

use crate::arbitrage::{calculate_arbitrage, calculate_multi_arbitrage};
use crate::display::{print_result, print_result_arbitrage, print_result_multi_arbitrage, print_result_polymarket, print_result_stock, print_usage};
use crate::kelly::{build_stock_info, kelly_criterion, kelly_polymarket, kelly_stock};
use crate::validation::{parse_market_price, parse_odds, parse_percent, parse_positive};

/// 标准凯利 CLI 模式
pub fn cli_mode(odds: f64, win_rate: f64, capital: Option<f64>) {
    let result = kelly_criterion(odds, win_rate);
    print_result(odds, win_rate, &result, capital);
}

/// Polymarket CLI 模式
pub fn cli_mode_polymarket(market_price: f64, your_probability: f64, capital: Option<f64>) {
    let result = kelly_polymarket(market_price, your_probability);
    print_result_polymarket(market_price, your_probability, &result, capital);
}

/// 股票 CLI 模式
pub fn cli_mode_stock(entry_price: f64, target_price: f64, stop_loss: f64, win_rate: f64, capital: Option<f64>) {
    let info = build_stock_info(entry_price, target_price, stop_loss);
    let result = kelly_stock(entry_price, target_price, stop_loss, win_rate);
    print_result_stock(&info, win_rate, &result, capital);
}

/// 套利 CLI 模式
pub fn cli_mode_arbitrage(odds1: f64, odds2: f64, capital: Option<f64>) {
    let result = calculate_arbitrage(odds1, odds2);
    print_result_arbitrage(odds1, odds2, &result, capital);
}

/// 多标的套利 CLI 模式
pub fn cli_mode_multi_arbitrage(odds: Vec<f64>, capital: Option<f64>) {
    let result = calculate_multi_arbitrage(&odds);
    print_result_multi_arbitrage(&odds, &result, capital);
}

/// 处理命令行参数
pub fn handle_args(args: Vec<String>) {
    let is_polymarket = args.iter().any(|a| a == "-p");
    let is_stock = args.iter().any(|a| a == "-s");
    let is_arbitrage = args.iter().any(|a| a == "-a");
    let is_multi_arbitrage = args.iter().any(|a| a == "-A");

    if is_multi_arbitrage {
        handle_multi_arbitrage(args);
    } else if is_arbitrage {
        handle_arbitrage(args);
    } else if is_stock {
        handle_stock(args);
    } else if is_polymarket {
        handle_polymarket(args);
    } else {
        handle_standard(args);
    }
}

fn handle_standard(args: Vec<String>) {
    match args.len() {
        2 => {
            if args[1] == "-h" || args[1] == "--help" {
                print_usage();
            } else {
                println!("✗ 参数不足");
                print_usage();
            }
        }
        3 => {
            let odds = match parse_odds(&args[1], "赔率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let win_rate = match parse_percent(&args[2], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            cli_mode(odds, win_rate, None);
        }
        4 => {
            let odds = match parse_odds(&args[1], "赔率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let win_rate = match parse_percent(&args[2], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let capital = match parse_positive(&args[3], "本金") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            cli_mode(odds, win_rate, Some(capital));
        }
        _ => {
            println!("✗ 参数过多");
            print_usage();
        }
    }
}

fn handle_polymarket(args: Vec<String>) {
    let pm_args: Vec<&String> = args.iter().filter(|&a| a != "-p").collect();

    match pm_args.len() {
        1 => {
            // 交互式模式由 main.rs 处理
        }
        3 => {
            let market_price = match parse_market_price(pm_args[1]) {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let your_prob = match parse_percent(pm_args[2], "你的概率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            cli_mode_polymarket(market_price, your_prob, None);
        }
        4 => {
            let market_price = match parse_market_price(pm_args[1]) {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let your_prob = match parse_percent(pm_args[2], "你的概率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let capital = match parse_positive(pm_args[3], "本金") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            cli_mode_polymarket(market_price, your_prob, Some(capital));
        }
        _ => {
            println!("✗ Polymarket 模式参数错误");
            println!();
            println!("用法: bo -p <市场价格> <你的概率> [本金]");
            println!("示例: bo -p 60 75    # 市场价格60c，你认为75%");
        }
    }
}

fn handle_stock(args: Vec<String>) {
    let s_args: Vec<&String> = args.iter().filter(|&a| a != "-s").collect();

    match s_args.len() {
        1 => {
            // 交互式模式由 main.rs 处理
        }
        5 => {
            let entry = match parse_positive(s_args[1], "当前价") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let target = match parse_positive(s_args[2], "止盈价") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let stop = match parse_positive(s_args[3], "止损价") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let win_rate = match parse_percent(s_args[4], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };

            if target <= entry || stop >= entry {
                println!("✗ 参数错误: 止盈价必须大于当前价，止损价必须小于当前价");
            } else {
                cli_mode_stock(entry, target, stop, win_rate, None);
            }
        }
        6 => {
            let entry = match parse_positive(s_args[1], "当前价") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let target = match parse_positive(s_args[2], "止盈价") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let stop = match parse_positive(s_args[3], "止损价") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let win_rate = match parse_percent(s_args[4], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let capital = match parse_positive(s_args[5], "本金") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };

            if target <= entry || stop >= entry {
                println!("✗ 参数错误: 止盈价必须大于当前价，止损价必须小于当前价");
            } else {
                cli_mode_stock(entry, target, stop, win_rate, Some(capital));
            }
        }
        _ => {
            println!("✗ 股票模式参数错误");
            println!();
            println!("用法: bo -s <当前价> <止盈价> <止损价> <胜率> [本金]");
            println!("示例: bo -s 100 120 90 60    # 当前价100，止盈120，止损90，胜率60%");
        }
    }
}

fn handle_arbitrage(args: Vec<String>) {
    let a_args: Vec<&String> = args.iter().filter(|&a| a != "-a").collect();

    match a_args.len() {
        1 => {
            // 交互式模式由 main.rs 处理
        }
        3 => {
            let odds1 = match parse_odds(a_args[1], "赔率1") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let odds2 = match parse_odds(a_args[2], "赔率2") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            cli_mode_arbitrage(odds1, odds2, None);
        }
        4 => {
            let odds1 = match parse_odds(a_args[1], "赔率1") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let odds2 = match parse_odds(a_args[2], "赔率2") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            let capital = match parse_positive(a_args[3], "本金") {
                Ok(v) => v,
                Err(e) => {
                    println!("✗ {}", e);
                    return;
                }
            };
            cli_mode_arbitrage(odds1, odds2, Some(capital));
        }
        _ => {
            println!("✗ 套利模式参数错误");
            println!();
            println!("用法: bo -a <赔率1> <赔率2> [本金]");
            println!("示例: bo -a 1.9 2.1    # 方案1赔率1.9，方案2赔率2.1");
        }
    }
}

fn handle_multi_arbitrage(args: Vec<String>) {
    let ma_args: Vec<&String> = args.iter().filter(|&a| a != "-A").collect();

    if ma_args.len() < 2 {
        println!("✗ 多标的套利模式参数不足");
        println!();
        println!("用法: bo -A <标的数量> <赔率1> ... <赔率N> [本金]");
        println!("示例: bo -A 3 2.0 3.5 4.0    # 3个标的，赔率分别为2.0, 3.5, 4.0");
        return;
    }

    let count: usize = match ma_args[1].parse() {
        Ok(n) if n >= 2 => n,
        Ok(_) => {
            println!("✗ 标的数量必须至少为 2");
            return;
        }
        Err(_) => {
            println!("✗ 标的数量必须是数字");
            return;
        }
    };

    let expected_min = count + 2;
    let has_capital = ma_args.len() == expected_min + 1;

    if ma_args.len() != expected_min && !has_capital {
        println!("✗ 参数数量不匹配，期望 {} 个赔率值，实际得到 {}", count, ma_args.len() - 2);
        println!();
        println!("用法: bo -A <标的数量> <赔率1> ... <赔率N> [本金]");
        println!("示例: bo -A 3 2.0 3.5 4.0    # 3个标的，赔率分别为2.0, 3.5, 4.0");
        return;
    }

    let mut odds = Vec::new();
    for i in 0..count {
        let o: f64 = match ma_args[2 + i].parse() {
            Ok(n) if n > 1.0 => n,
            Ok(_) => {
                println!("✗ 赔率必须大于 1.0");
                return;
            }
            Err(_) => {
                println!("✗ 赔率{}必须是数字", i + 1);
                return;
            }
        };
        odds.push(o);
    }

    let capital = if has_capital {
        let cap: f64 = match ma_args[ma_args.len() - 1].parse() {
            Ok(n) if n > 0.0 => n,
            _ => {
                println!("✗ 本金必须为正数");
                return;
            }
        };
        Some(cap)
    } else {
        None
    };

    cli_mode_multi_arbitrage(odds, capital);
}

/// 检查是否为交互式模式调用
pub fn is_interactive_call(args: &[String]) -> bool {
    if args.len() == 1 {
        return true;
    }

    let flags = ["-p", "-s", "-a", "-A"];
    for flag in &flags {
        if args.iter().any(|a| a == *flag) && args.len() == 2 {
            return true;
        }
    }

    false
}
