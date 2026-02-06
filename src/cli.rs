//! CLI 命令行模式

use crate::app::{ModeRequest, OutputFormat, execute_mode};
use crate::display::{print_json_error, print_usage};
use crate::portfolio_input::{build_standard_leg, parse_portfolio_leg_descriptor};
use crate::types::PortfolioScenario;
use crate::validation::{parse_f64, parse_market_price, parse_odds, parse_percent, parse_positive};

fn is_help_flag(flag: &str) -> bool {
    matches!(flag, "-h" | "-help" | "--help")
}

fn is_version_flag(flag: &str) -> bool {
    matches!(flag, "-v" | "-version" | "--version")
}

fn print_version() {
    println!("bo {}", env!("CARGO_PKG_VERSION"));
}

fn parse_return_percent(input: &str, field_name: &str) -> Result<f64, String> {
    let value = parse_f64(input, field_name)? / 100.0;
    if value < -1.0 {
        Err(format!(
            "{field_name}不能小于 -100%（当前为 {:.2}%）",
            value * 100.0
        ))
    } else {
        Ok(value)
    }
}

fn probability_sum_tolerance(scenario_count: usize) -> f64 {
    // 允许按两位小数录入概率时的累计四舍五入误差
    (scenario_count as f64) * 0.00005 + 1e-9
}

fn emit_error(output: OutputFormat, message: &str) {
    if output.is_json() {
        print_json_error(message);
    } else {
        println!("✗ {}", message);
    }
}

/// 处理命令行参数
pub fn handle_args(args: Vec<String>) {
    let output = if args.iter().any(|a| a == "--json") {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    let args: Vec<String> = args.into_iter().filter(|a| a != "--json").collect();

    if args.len() == 2 && is_help_flag(&args[1]) {
        print_usage();
        return;
    }
    if args.len() == 2 && is_version_flag(&args[1]) {
        print_version();
        return;
    }

    if args.len() == 1 && output.is_json() {
        emit_error(output, "JSON 模式需要命令行参数，不支持交互式模式");
        return;
    }

    let is_polymarket = args.iter().any(|a| a == "-p");
    let is_stock = args.iter().any(|a| a == "-s");
    let is_arbitrage = args.iter().any(|a| a == "-a");
    let is_multi_arbitrage = args.iter().any(|a| a == "-A");
    let is_nash = args.iter().any(|a| a == "-n");
    let is_portfolio_correlated = args.iter().any(|a| a == "-K");
    let is_portfolio = args.iter().any(|a| a == "-k");

    if is_portfolio_correlated {
        handle_portfolio_correlated(args, output);
    } else if is_portfolio {
        handle_portfolio(args, output);
    } else if is_nash {
        handle_nash(args, output);
    } else if is_multi_arbitrage {
        handle_multi_arbitrage(args, output);
    } else if is_arbitrage {
        handle_arbitrage(args, output);
    } else if is_stock {
        handle_stock(args, output);
    } else if is_polymarket {
        handle_polymarket(args, output);
    } else {
        handle_standard(args, output);
    }
}

fn handle_standard(args: Vec<String>, output: OutputFormat) {
    match args.len() {
        2 => {
            if is_help_flag(&args[1]) {
                print_usage();
            } else if is_version_flag(&args[1]) {
                print_version();
            } else {
                emit_error(output, "参数不足");
                if !output.is_json() {
                    print_usage();
                }
            }
        }
        3 => {
            let odds = match parse_odds(&args[1], "赔率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let win_rate = match parse_percent(&args[2], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            execute_mode(
                ModeRequest::Standard {
                    odds,
                    win_rate,
                    capital: None,
                },
                output,
            );
        }
        4 => {
            let odds = match parse_odds(&args[1], "赔率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let win_rate = match parse_percent(&args[2], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let capital = match parse_positive(&args[3], "本金") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            execute_mode(
                ModeRequest::Standard {
                    odds,
                    win_rate,
                    capital: Some(capital),
                },
                output,
            );
        }
        _ => {
            emit_error(output, "参数错误");
            if !output.is_json() {
                print_usage();
            }
        }
    }
}

fn handle_polymarket(args: Vec<String>, output: OutputFormat) {
    let pm_args: Vec<&String> = args.iter().filter(|&a| a != "-p").collect();

    match pm_args.len() {
        1 => {
            emit_error(output, "Polymarket 模式参数不足");
        }
        3 => {
            let market_price = match parse_market_price(pm_args[1]) {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let your_prob = match parse_percent(pm_args[2], "你的概率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            execute_mode(
                ModeRequest::Polymarket {
                    market_price,
                    your_probability: your_prob,
                    capital: None,
                },
                output,
            );
        }
        4 => {
            let market_price = match parse_market_price(pm_args[1]) {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let your_prob = match parse_percent(pm_args[2], "你的概率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let capital = match parse_positive(pm_args[3], "本金") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            execute_mode(
                ModeRequest::Polymarket {
                    market_price,
                    your_probability: your_prob,
                    capital: Some(capital),
                },
                output,
            );
        }
        _ => {
            emit_error(output, "Polymarket 模式参数错误");
            if !output.is_json() {
                println!();
                println!("用法: bo -p <市场价格> <你的概率> [本金]");
                println!("示例: bo -p 60 75    # 市场价格60c，你认为75%");
            }
        }
    }
}

fn handle_stock(args: Vec<String>, output: OutputFormat) {
    let s_args: Vec<&String> = args.iter().filter(|&a| a != "-s").collect();

    match s_args.len() {
        1 => {
            emit_error(output, "股票模式参数不足");
        }
        5 => {
            let entry = match parse_positive(s_args[1], "当前价") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let target = match parse_positive(s_args[2], "止盈价") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let stop = match parse_positive(s_args[3], "止损价") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let win_rate = match parse_percent(s_args[4], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };

            if target <= entry || stop >= entry {
                emit_error(
                    output,
                    "参数错误: 止盈价必须大于当前价，止损价必须小于当前价",
                );
            } else {
                execute_mode(
                    ModeRequest::Stock {
                        entry_price: entry,
                        target_price: target,
                        stop_loss: stop,
                        win_rate,
                        capital: None,
                    },
                    output,
                );
            }
        }
        6 => {
            let entry = match parse_positive(s_args[1], "当前价") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let target = match parse_positive(s_args[2], "止盈价") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let stop = match parse_positive(s_args[3], "止损价") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let win_rate = match parse_percent(s_args[4], "胜率") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let capital = match parse_positive(s_args[5], "本金") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };

            if target <= entry || stop >= entry {
                emit_error(
                    output,
                    "参数错误: 止盈价必须大于当前价，止损价必须小于当前价",
                );
            } else {
                execute_mode(
                    ModeRequest::Stock {
                        entry_price: entry,
                        target_price: target,
                        stop_loss: stop,
                        win_rate,
                        capital: Some(capital),
                    },
                    output,
                );
            }
        }
        _ => {
            emit_error(output, "股票模式参数错误");
            if !output.is_json() {
                println!();
                println!("用法: bo -s <当前价> <止盈价> <止损价> <胜率> [本金]");
                println!("示例: bo -s 100 120 90 60    # 当前价100，止盈120，止损90，胜率60%");
            }
        }
    }
}

fn handle_arbitrage(args: Vec<String>, output: OutputFormat) {
    let a_args: Vec<&String> = args.iter().filter(|&a| a != "-a").collect();

    match a_args.len() {
        1 => {
            emit_error(output, "套利模式参数不足");
        }
        3 => {
            let odds1 = match parse_odds(a_args[1], "赔率1") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let odds2 = match parse_odds(a_args[2], "赔率2") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            execute_mode(
                ModeRequest::Arbitrage {
                    odds1,
                    odds2,
                    capital: None,
                },
                output,
            );
        }
        4 => {
            let odds1 = match parse_odds(a_args[1], "赔率1") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let odds2 = match parse_odds(a_args[2], "赔率2") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            let capital = match parse_positive(a_args[3], "本金") {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            execute_mode(
                ModeRequest::Arbitrage {
                    odds1,
                    odds2,
                    capital: Some(capital),
                },
                output,
            );
        }
        _ => {
            emit_error(output, "套利模式参数错误");
            if !output.is_json() {
                println!();
                println!("用法: bo -a <赔率1> <赔率2> [本金]");
                println!("示例: bo -a 1.9 2.1    # 方案1赔率1.9，方案2赔率2.1");
            }
        }
    }
}

fn handle_multi_arbitrage(args: Vec<String>, output: OutputFormat) {
    let ma_args: Vec<&String> = args.iter().filter(|&a| a != "-A").collect();

    if ma_args.len() < 2 {
        emit_error(output, "多标的套利模式参数不足");
        if !output.is_json() {
            println!();
            println!("用法: bo -A <标的数量> <赔率1> ... <赔率N> [本金]");
            println!("示例: bo -A 3 2.0 3.5 4.0    # 3个标的，赔率分别为2.0, 3.5, 4.0");
        }
        return;
    }

    let count: usize = match ma_args[1].parse() {
        Ok(n) if n >= 2 => n,
        Ok(_) => {
            emit_error(output, "标的数量必须至少为 2");
            return;
        }
        Err(_) => {
            emit_error(output, "标的数量必须是数字");
            return;
        }
    };

    let expected_min = count + 2;
    let has_capital = ma_args.len() == expected_min + 1;

    if ma_args.len() != expected_min && !has_capital {
        emit_error(
            output,
            &format!(
                "参数数量不匹配，期望 {} 个赔率值，实际得到 {}",
                count,
                ma_args.len() - 2
            ),
        );
        if !output.is_json() {
            println!();
            println!("用法: bo -A <标的数量> <赔率1> ... <赔率N> [本金]");
            println!("示例: bo -A 3 2.0 3.5 4.0    # 3个标的，赔率分别为2.0, 3.5, 4.0");
        }
        return;
    }

    let mut odds = Vec::new();
    for i in 0..count {
        let o: f64 = match ma_args[2 + i].parse() {
            Ok(n) if n > 1.0 => n,
            Ok(_) => {
                emit_error(output, "赔率必须大于 1.0");
                return;
            }
            Err(_) => {
                emit_error(output, &format!("赔率{}必须是数字", i + 1));
                return;
            }
        };
        odds.push(o);
    }

    let capital = if has_capital {
        let cap: f64 = match ma_args[ma_args.len() - 1].parse() {
            Ok(n) if n > 0.0 => n,
            _ => {
                emit_error(output, "本金必须为正数");
                return;
            }
        };
        Some(cap)
    } else {
        None
    };

    execute_mode(ModeRequest::MultiArbitrage { odds, capital }, output);
}

fn handle_nash(args: Vec<String>, output: OutputFormat) {
    let n_args: Vec<&String> = args.iter().filter(|&a| a != "-n").collect();

    match n_args.len() {
        1 => {
            emit_error(output, "纳什模式参数不足");
        }
        9 => {
            let labels = ["a11", "a12", "a21", "a22", "b11", "b12", "b21", "b22"];
            let mut values = [0.0_f64; 8];

            for i in 0..8 {
                let value = match parse_f64(n_args[i + 1], labels[i]) {
                    Ok(v) => v,
                    Err(e) => {
                        emit_error(output, &e);
                        return;
                    }
                };
                values[i] = value;
            }

            execute_mode(
                ModeRequest::Nash {
                    row_payoffs: [[values[0], values[1]], [values[2], values[3]]],
                    col_payoffs: [[values[4], values[5]], [values[6], values[7]]],
                },
                output,
            );
        }
        _ => {
            emit_error(output, "纳什模式参数错误");
            if !output.is_json() {
                println!();
                println!("用法: bo -n <a11> <a12> <a21> <a22> <b11> <b12> <b21> <b22>");
                println!("示例: bo -n 3 0 5 1 3 5 0 1    # 囚徒困境收益矩阵");
            }
        }
    }
}

fn handle_portfolio_correlated(args: Vec<String>, output: OutputFormat) {
    let c_args: Vec<&String> = args.iter().filter(|&a| a != "-K").collect();

    if c_args.len() < 3 {
        emit_error(output, "相关情景组合凯利模式参数不足");
        if !output.is_json() {
            println!();
            println!(
                "用法: bo -K <标的数量> <情景数量> <p1> <r11> ... <r1N> ... <pM> <rM1> ... <rMN> [本金]"
            );
            println!("说明: 概率和收益率都按百分数输入，例如 50 代表 50%");
        }
        return;
    }

    let leg_count: usize = match c_args[1].parse() {
        Ok(n) if (1..=12).contains(&n) => n,
        Ok(_) => {
            emit_error(output, "标的数量必须在 1-12 之间");
            return;
        }
        Err(_) => {
            emit_error(output, "标的数量必须是数字");
            return;
        }
    };

    let scenario_count: usize = match c_args[2].parse() {
        Ok(n) if (2..=128).contains(&n) => n,
        Ok(_) => {
            emit_error(output, "情景数量必须在 2-128 之间");
            return;
        }
        Err(_) => {
            emit_error(output, "情景数量必须是数字");
            return;
        }
    };

    let expected_min = 3 + scenario_count * (1 + leg_count);
    let has_capital = c_args.len() == expected_min + 1;
    if c_args.len() != expected_min && !has_capital {
        emit_error(
            output,
            &format!(
                "参数数量不匹配，期望 {} 个情景，每个情景包含 1 个概率 + {} 个收益率",
                scenario_count, leg_count
            ),
        );
        if !output.is_json() {
            println!();
            println!(
                "用法: bo -K <标的数量> <情景数量> <p1> <r11> ... <r1N> ... <pM> <rM1> ... <rMN> [本金]"
            );
            println!("示例: bo -K 2 2 50 20 -10 50 -10 20 10000");
        }
        return;
    }

    let mut scenarios = Vec::with_capacity(scenario_count);
    let mut idx = 3;
    for s in 0..scenario_count {
        let prob = match parse_percent(c_args[idx], &format!("情景{}概率", s + 1)) {
            Ok(v) => v,
            Err(e) => {
                emit_error(output, &e);
                return;
            }
        };
        idx += 1;

        let mut returns = Vec::with_capacity(leg_count);
        for i in 0..leg_count {
            let field = format!("情景{}收益{}", s + 1, i + 1);
            let ret = match parse_return_percent(c_args[idx], &field) {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            returns.push(ret);
            idx += 1;
        }
        scenarios.push(PortfolioScenario {
            probability: prob,
            returns,
        });
    }

    let prob_sum: f64 = scenarios.iter().map(|s| s.probability).sum();
    let tolerance = probability_sum_tolerance(scenario_count);
    if (prob_sum - 1.0).abs() > tolerance {
        emit_error(
            output,
            &format!(
                "所有情景概率之和必须约等于 100%（容差 ±{:.4}%），当前为 {:.4}%",
                tolerance * 100.0,
                prob_sum * 100.0
            ),
        );
        return;
    }

    let capital = if has_capital {
        match parse_positive(c_args[c_args.len() - 1], "本金") {
            Ok(v) => Some(v),
            Err(e) => {
                emit_error(output, &e);
                return;
            }
        }
    } else {
        None
    };

    execute_mode(
        ModeRequest::PortfolioCorrelated {
            leg_count,
            scenarios,
            capital,
        },
        output,
    );
}

fn handle_portfolio(args: Vec<String>, output: OutputFormat) {
    let p_args: Vec<&String> = args.iter().filter(|&a| a != "-k").collect();

    if p_args.len() < 2 {
        emit_error(output, "组合凯利模式参数不足");
        if !output.is_json() {
            println!();
            println!("用法: bo -k <标的数量> <赔率1> <胜率1> ... <赔率N> <胜率N> [本金]");
            println!("示例: bo -k 2 2.0 60 2.5 55 10000");
        }
        return;
    }

    // 新格式: `-k <descriptor1> <descriptor2> ... [本金]`
    // descriptor 支持: std/pm/stock/arb/marb
    if p_args[1].parse::<usize>().is_err() {
        let mut end = p_args.len();
        let mut capital = None;

        if end > 2 && !p_args[end - 1].contains(':') {
            match parse_positive(p_args[end - 1], "本金") {
                Ok(v) => {
                    capital = Some(v);
                    end -= 1;
                }
                Err(e) => {
                    emit_error(output, &format!("组合标的描述错误或本金错误: {}", e));
                    return;
                }
            }
        }

        let mut legs = Vec::new();
        for token in &p_args[1..end] {
            if !token.contains(':') {
                emit_error(output, "组合标的格式错误，示例: std:2.0:60");
                return;
            }
            let leg = match parse_portfolio_leg_descriptor(token) {
                Ok(v) => v,
                Err(e) => {
                    emit_error(output, &e);
                    return;
                }
            };
            legs.push(leg);
        }

        if legs.len() < 2 {
            emit_error(output, "组合凯利至少需要 2 个标的");
            return;
        }
        if legs.len() > 12 {
            emit_error(output, "组合凯利最多支持 12 个标的");
            return;
        }

        execute_mode(ModeRequest::Portfolio { legs, capital }, output);
        return;
    }

    // 兼容旧格式: `-k <数量> <赔率1> <胜率1> ... <赔率N> <胜率N> [本金]`
    let count: usize = match p_args[1].parse() {
        Ok(n) if (2..=12).contains(&n) => n,
        Ok(_) => {
            emit_error(output, "标的数量必须在 2-12 之间");
            return;
        }
        Err(_) => {
            emit_error(output, "标的数量必须是数字");
            return;
        }
    };

    let expected_min = 2 + count * 2;
    let has_capital = p_args.len() == expected_min + 1;

    if p_args.len() != expected_min && !has_capital {
        emit_error(
            output,
            &format!(
                "参数数量不匹配，期望 {} 对(赔率,胜率)参数，实际得到 {} 对",
                count,
                (p_args.len().saturating_sub(2)) / 2
            ),
        );
        if !output.is_json() {
            println!();
            println!("用法1: bo -k <标的数量> <赔率1> <胜率1> ... <赔率N> <胜率N> [本金]");
            println!("用法2: bo -k <descriptor1> <descriptor2> ... [本金]");
            println!("示例: bo -k std:2.0:60 pm:60:75 stock:100:120:90:60 10000");
        }
        return;
    }

    let mut legs = Vec::with_capacity(count);
    for i in 0..count {
        let odds_field = format!("赔率{}", i + 1);
        let win_rate_field = format!("胜率{}", i + 1);

        let odds = match parse_odds(p_args[2 + i * 2], &odds_field) {
            Ok(v) => v,
            Err(e) => {
                emit_error(output, &e);
                return;
            }
        };
        let win_rate = match parse_percent(p_args[3 + i * 2], &win_rate_field) {
            Ok(v) => v,
            Err(e) => {
                emit_error(output, &e);
                return;
            }
        };

        legs.push(build_standard_leg(odds, win_rate));
    }

    let capital = if has_capital {
        match parse_positive(p_args[p_args.len() - 1], "本金") {
            Ok(v) => Some(v),
            Err(e) => {
                emit_error(output, &e);
                return;
            }
        }
    } else {
        None
    };

    execute_mode(ModeRequest::Portfolio { legs, capital }, output);
}

/// 检查是否为交互式模式调用
pub fn is_interactive_call(args: &[String]) -> bool {
    if args.len() == 1 {
        return true;
    }

    let flags = ["-p", "-s", "-a", "-A", "-n", "-k", "-K"];
    for flag in &flags {
        if args.iter().any(|a| a == *flag) && args.len() == 2 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{parse_return_percent, probability_sum_tolerance};

    #[test]
    fn return_percent_rejects_less_than_negative_hundred() {
        assert!(parse_return_percent("-100.01", "收益率").is_err());
    }

    #[test]
    fn return_percent_accepts_negative_hundred() {
        assert_eq!(parse_return_percent("-100", "收益率").unwrap(), -1.0);
    }

    #[test]
    fn probability_tolerance_accepts_three_way_rounding() {
        let sum: f64 = 0.3333 + 0.3333 + 0.3333;
        assert!((sum - 1.0).abs() <= probability_sum_tolerance(3));
    }
}
