//! CLI 命令行模式

use std::process::ExitCode;

use crate::app::{ModeRequest, OutputFormat, execute_mode};
use crate::display::{print_json_error, print_usage};
use crate::portfolio_input::{build_standard_leg, parse_portfolio_leg_descriptor};
use crate::types::PortfolioScenario;
use crate::validation::{parse_f64, parse_market_price, parse_odds, parse_percent, parse_positive};

const MODE_FLAGS: [&str; 8] = ["-L", "-p", "-s", "-a", "-A", "-n", "-k", "-K"];
const INTERACTIVE_MODE_FLAGS: [&str; 8] = ["-L", "-p", "-s", "-a", "-A", "-n", "-k", "-K"];

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

fn emit_error(output: OutputFormat, message: &str, show_usage: bool) -> ExitCode {
    if output.is_json() {
        print_json_error(message);
    } else {
        eprintln!("✗ {}", message);
        if show_usage {
            print_usage();
        }
    }

    ExitCode::FAILURE
}

fn finish(mode: ModeRequest, output: OutputFormat) -> ExitCode {
    match execute_mode(mode, output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => emit_error(output, &e, false),
    }
}

fn args_without_flag<'a>(args: &'a [String], flag: &str) -> Vec<&'a str> {
    args.iter()
        .skip(1)
        .filter_map(|arg| {
            if arg == flag {
                None
            } else {
                Some(arg.as_str())
            }
        })
        .collect()
}

fn user_args(args: &[String]) -> Vec<&str> {
    args.iter().skip(1).map(String::as_str).collect()
}

/// 处理命令行参数
pub fn handle_args(args: Vec<String>) -> ExitCode {
    let output = if args.iter().any(|a| a == "--json") {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    let args: Vec<String> = args.into_iter().filter(|a| a != "--json").collect();

    if args.len() == 2 && is_help_flag(&args[1]) {
        print_usage();
        return ExitCode::SUCCESS;
    }
    if args.len() == 2 && is_version_flag(&args[1]) {
        print_version();
        return ExitCode::SUCCESS;
    }
    if args.len() == 1 && output.is_json() {
        return emit_error(output, "JSON 模式需要命令行参数，不支持交互式模式", false);
    }

    let present_flags: Vec<&str> = MODE_FLAGS
        .iter()
        .copied()
        .filter(|flag| args.iter().any(|arg| arg == flag))
        .collect();

    if present_flags.len() > 1 {
        return emit_error(output, "一次只能指定一种计算模式", false);
    }

    let parsed = match present_flags.first().copied() {
        Some("-L") => parse_asset_level(&args_without_flag(&args, "-L")),
        Some("-K") => parse_portfolio_correlated(&args_without_flag(&args, "-K")),
        Some("-k") => parse_portfolio(&args_without_flag(&args, "-k")),
        Some("-n") => parse_nash(&args_without_flag(&args, "-n")),
        Some("-A") => parse_multi_arbitrage(&args_without_flag(&args, "-A")),
        Some("-a") => parse_arbitrage(&args_without_flag(&args, "-a")),
        Some("-s") => parse_stock(&args_without_flag(&args, "-s")),
        Some("-p") => parse_polymarket(&args_without_flag(&args, "-p")),
        Some(_) => unreachable!("mode flag list is exhaustive"),
        None => parse_standard(&user_args(&args)),
    };

    match parsed {
        Ok(mode) => finish(mode, output),
        Err(e) => {
            let show_usage = !output.is_json()
                && present_flags.is_empty()
                && matches!(e.as_str(), "参数不足" | "参数错误");
            emit_error(output, &e, show_usage)
        }
    }
}

fn parse_asset_level(args: &[&str]) -> Result<ModeRequest, String> {
    match args.len() {
        1 => Ok(ModeRequest::AssetLevel {
            amount: parse_positive(args[0], "资金")?,
        }),
        0 => Err("资产等级模式参数不足".to_string()),
        _ => Err("资产等级模式参数错误".to_string()),
    }
}

fn parse_standard(args: &[&str]) -> Result<ModeRequest, String> {
    match args.len() {
        2 | 3 => {
            let odds = parse_odds(args[0], "赔率")?;
            let win_rate = parse_percent(args[1], "胜率")?;
            let capital = if args.len() == 3 {
                Some(parse_positive(args[2], "本金")?)
            } else {
                None
            };
            Ok(ModeRequest::Standard {
                odds,
                win_rate,
                capital,
            })
        }
        0 | 1 => Err("参数不足".to_string()),
        _ => Err("参数错误".to_string()),
    }
}

fn parse_polymarket(args: &[&str]) -> Result<ModeRequest, String> {
    match args.len() {
        2 | 3 => {
            let market_price = parse_market_price(args[0])?;
            let your_probability = parse_percent(args[1], "你的概率")?;
            let capital = if args.len() == 3 {
                Some(parse_positive(args[2], "本金")?)
            } else {
                None
            };
            Ok(ModeRequest::Polymarket {
                market_price,
                your_probability,
                capital,
            })
        }
        0 | 1 => Err("Polymarket 模式参数不足".to_string()),
        _ => Err("Polymarket 模式参数错误".to_string()),
    }
}

fn parse_stock(args: &[&str]) -> Result<ModeRequest, String> {
    match args.len() {
        4 | 5 => {
            let entry_price = parse_positive(args[0], "当前价")?;
            let target_price = parse_positive(args[1], "止盈价")?;
            let stop_loss = parse_positive(args[2], "止损价")?;
            let win_rate = parse_percent(args[3], "胜率")?;
            let capital = if args.len() == 5 {
                Some(parse_positive(args[4], "本金")?)
            } else {
                None
            };

            if target_price <= entry_price || stop_loss >= entry_price {
                return Err("参数错误: 止盈价必须大于当前价，止损价必须小于当前价".to_string());
            }

            Ok(ModeRequest::Stock {
                entry_price,
                target_price,
                stop_loss,
                win_rate,
                capital,
            })
        }
        0..=3 => Err("股票模式参数不足".to_string()),
        _ => Err("股票模式参数错误".to_string()),
    }
}

fn parse_arbitrage(args: &[&str]) -> Result<ModeRequest, String> {
    match args.len() {
        2 | 3 => {
            let odds1 = parse_odds(args[0], "赔率1")?;
            let odds2 = parse_odds(args[1], "赔率2")?;
            let capital = if args.len() == 3 {
                Some(parse_positive(args[2], "本金")?)
            } else {
                None
            };
            Ok(ModeRequest::Arbitrage {
                odds1,
                odds2,
                capital,
            })
        }
        0 | 1 => Err("套利模式参数不足".to_string()),
        _ => Err("套利模式参数错误".to_string()),
    }
}

fn parse_multi_arbitrage(args: &[&str]) -> Result<ModeRequest, String> {
    if args.is_empty() {
        return Err("多标的套利模式参数不足".to_string());
    }

    let count: usize = args[0]
        .parse()
        .map_err(|_| "标的数量必须是数字".to_string())?;
    if count < 2 {
        return Err("标的数量必须至少为 2".to_string());
    }

    let expected = 1 + count;
    let has_capital = args.len() == expected + 1;
    if args.len() != expected && !has_capital {
        return Err(format!(
            "参数数量不匹配，期望 {} 个赔率值，实际得到 {}",
            count,
            args.len().saturating_sub(1)
        ));
    }

    let mut odds = Vec::with_capacity(count);
    for i in 0..count {
        odds.push(parse_odds(args[1 + i], &format!("赔率{}", i + 1))?);
    }

    let capital = if has_capital {
        Some(parse_positive(args[args.len() - 1], "本金")?)
    } else {
        None
    };

    Ok(ModeRequest::MultiArbitrage { odds, capital })
}

fn parse_nash(args: &[&str]) -> Result<ModeRequest, String> {
    if args.len() != 8 {
        return Err(if args.is_empty() {
            "纳什模式参数不足".to_string()
        } else {
            "纳什模式参数错误".to_string()
        });
    }

    let labels = ["a11", "a12", "a21", "a22", "b11", "b12", "b21", "b22"];
    let mut values = [0.0_f64; 8];
    for (i, label) in labels.iter().enumerate() {
        values[i] = parse_f64(args[i], label)?;
    }

    Ok(ModeRequest::Nash {
        row_payoffs: [[values[0], values[1]], [values[2], values[3]]],
        col_payoffs: [[values[4], values[5]], [values[6], values[7]]],
    })
}

fn parse_portfolio_correlated(args: &[&str]) -> Result<ModeRequest, String> {
    if args.len() < 2 {
        return Err("相关情景组合凯利模式参数不足".to_string());
    }

    let leg_count: usize = args[0]
        .parse()
        .map_err(|_| "标的数量必须是数字".to_string())?;
    if !(1..=12).contains(&leg_count) {
        return Err("标的数量必须在 1-12 之间".to_string());
    }

    let scenario_count: usize = args[1]
        .parse()
        .map_err(|_| "情景数量必须是数字".to_string())?;
    if !(2..=128).contains(&scenario_count) {
        return Err("情景数量必须在 2-128 之间".to_string());
    }

    let expected = 2 + scenario_count * (1 + leg_count);
    let has_capital = args.len() == expected + 1;
    if args.len() != expected && !has_capital {
        return Err(format!(
            "参数数量不匹配，期望 {} 个情景，每个情景包含 1 个概率 + {} 个收益率",
            scenario_count, leg_count
        ));
    }

    let mut scenarios = Vec::with_capacity(scenario_count);
    let mut idx = 2;
    for s in 0..scenario_count {
        let probability = parse_percent(args[idx], &format!("情景{}概率", s + 1))?;
        idx += 1;

        let mut returns = Vec::with_capacity(leg_count);
        for i in 0..leg_count {
            let field = format!("情景{}收益{}", s + 1, i + 1);
            returns.push(parse_return_percent(args[idx], &field)?);
            idx += 1;
        }

        scenarios.push(PortfolioScenario {
            probability,
            returns,
        });
    }

    let prob_sum: f64 = scenarios.iter().map(|s| s.probability).sum();
    let tolerance = probability_sum_tolerance(scenario_count);
    if (prob_sum - 1.0).abs() > tolerance {
        return Err(format!(
            "所有情景概率之和必须约等于 100%（容差 ±{:.4}%），当前为 {:.4}%",
            tolerance * 100.0,
            prob_sum * 100.0
        ));
    }

    let capital = if has_capital {
        Some(parse_positive(args[args.len() - 1], "本金")?)
    } else {
        None
    };

    Ok(ModeRequest::PortfolioCorrelated {
        leg_count,
        scenarios,
        capital,
    })
}

fn parse_portfolio(args: &[&str]) -> Result<ModeRequest, String> {
    if args.is_empty() {
        return Err("组合凯利模式参数不足".to_string());
    }

    if args[0].parse::<usize>().is_err() {
        return parse_descriptor_portfolio(args);
    }

    let count: usize = args[0]
        .parse()
        .map_err(|_| "标的数量必须是数字".to_string())?;
    if !(2..=12).contains(&count) {
        return Err("标的数量必须在 2-12 之间".to_string());
    }

    let expected = 1 + count * 2;
    let has_capital = args.len() == expected + 1;
    if args.len() != expected && !has_capital {
        return Err(format!(
            "参数数量不匹配，期望 {} 对(赔率,胜率)参数，实际得到 {} 对",
            count,
            args.len().saturating_sub(1) / 2
        ));
    }

    let mut legs = Vec::with_capacity(count);
    for i in 0..count {
        let odds = parse_odds(args[1 + i * 2], &format!("赔率{}", i + 1))?;
        let win_rate = parse_percent(args[2 + i * 2], &format!("胜率{}", i + 1))?;
        legs.push(build_standard_leg(odds, win_rate));
    }

    let capital = if has_capital {
        Some(parse_positive(args[args.len() - 1], "本金")?)
    } else {
        None
    };

    Ok(ModeRequest::Portfolio { legs, capital })
}

fn parse_descriptor_portfolio(args: &[&str]) -> Result<ModeRequest, String> {
    let mut end = args.len();
    let mut capital = None;

    if end > 1 && !args[end - 1].contains(':') {
        capital = Some(
            parse_positive(args[end - 1], "本金")
                .map_err(|e| format!("组合标的描述错误或本金错误: {}", e))?,
        );
        end -= 1;
    }

    let mut legs = Vec::new();
    for token in &args[..end] {
        if !token.contains(':') {
            return Err("组合标的格式错误，示例: std:2.0:60".to_string());
        }
        legs.push(parse_portfolio_leg_descriptor(token)?);
    }

    if legs.len() < 2 {
        return Err("组合凯利至少需要 2 个标的".to_string());
    }
    if legs.len() > 12 {
        return Err("组合凯利最多支持 12 个标的".to_string());
    }

    Ok(ModeRequest::Portfolio { legs, capital })
}

/// 检查是否为交互式模式调用
pub fn is_interactive_call(args: &[String]) -> bool {
    if args.len() == 1 {
        return true;
    }

    INTERACTIVE_MODE_FLAGS
        .iter()
        .any(|flag| args.iter().any(|arg| arg == *flag) && args.len() == 2)
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
