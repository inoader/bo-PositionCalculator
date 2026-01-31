//! 凯利公式计算器
//! f* = (bp - q) / b
//! 其中 b 为赔率-1，p 为胜率，q = 1-p

use std::io::{self, Write};

/// 凯利公式计算结果
struct KellyResult {
    /// 最优仓位比例 (0-1)
    optimal_fraction: f64,
    /// 是否为正期望
    positive_ev: bool,
    /// 期望收益
    expected_value: f64,
}

/// 计算凯利公式
fn kelly_criterion(odds: f64, win_rate: f64) -> KellyResult {
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
fn kelly_polymarket(market_price: f64, your_probability: f64) -> KellyResult {
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
fn kelly_stock(entry_price: f64, target_price: f64, stop_loss: f64, win_rate: f64) -> KellyResult {
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

/// 股票交易信息
struct StockInfo {
    entry_price: f64,
    target_price: f64,
    stop_loss: f64,
    profit: f64,
    risk: f64,
    ratio: f64,
}

/// 格式化百分比
fn format_pct(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}

/// 打印分隔线
fn separator() {
    println!("{}", "─".repeat(50));
}

/// 打印标题
fn print_title() {
    separator();
    println!("                    凯利公式计算器");
    separator();
    println!();
}

/// 打印 Polymarket 标题
fn print_title_polymarket() {
    separator();
    println!("                Polymarket 凯利计算器");
    println!("            Kelly Criterion for Polymarket");
    separator();
    println!();
}

/// 打印股票标题
fn print_title_stock() {
    separator();
    println!("                    股票交易凯利计算器");
    separator();
    println!();
}

/// 打印标准结果
fn print_result(odds: f64, win_rate: f64, result: &KellyResult, capital: Option<f64>) {
    println!();
    separator();
    println!("                        计算结果");
    separator();
    println!();
    println!("  输入参数:");
    println!("    ├─ 赔率: {:.2}", odds);
    println!("    ├─ 净赔率 (b): {:.2}", odds - 1.0);
    println!("    └─ 胜率 (p): {}", format_pct(win_rate));
    println!();
    println!("  分析:");
    println!("    ├─ 期望收益 (EV): {:.2}%", result.expected_value * 100.0);

    if result.positive_ev {
        println!("    ├─ 状态: ✓ 正期望值 (值得下注)");
    } else {
        println!("    ├─ 状态: ✗ 负期望值 (不建议下注)");
    }

    if result.optimal_fraction <= 0.0 {
        println!("    └─ 仓位建议: 0% (不下注)");
    } else if result.optimal_fraction > 1.0 {
        println!("    └─ 仓位建议: 100%+ (全仓甚至加杠杆，高风险！)");
    } else {
        println!("    └─ 仓位建议: {}", format_pct(result.optimal_fraction));
    }
    println!();

    if let Some(cap) = capital {
        println!("  基于本金 {:.2} 的投注金额:", cap);
        if result.optimal_fraction > 0.0 {
            println!("    ├─ 全凯利: {:.2}", cap * result.optimal_fraction);
            println!("    ├─ 半凯利: {:.2}", cap * result.optimal_fraction * 0.5);
            println!("    └─ 1/4凯利: {:.2}", cap * result.optimal_fraction * 0.25);
        } else {
            println!("    └─ 建议: 不下注");
        }
        println!();
    }

    separator();
}

/// 打印 Polymarket 结果
fn print_result_polymarket(market_price: f64, your_probability: f64, result: &KellyResult, capital: Option<f64>) {
    println!();
    separator();
    println!("                    Polymarket 计算结果");
    separator();
    println!();
    println!("  输入参数:");
    println!("    ├─ 市场价格: {} (市场隐含概率)", format_pct(market_price));
    println!("    ├─ 你的概率: {} (你估计的真实概率)", format_pct(your_probability));
    println!("    └─ 隐含赔率: {:.2}", 1.0 / market_price);
    println!();
    println!("  分析:");
    println!("    ├─ 期望收益 (EV): {:.2}%", result.expected_value * 100.0);

    if result.positive_ev {
        println!("    ├─ 状态: ✓ 正期望值 (值得下注)");
    } else {
        println!("    ├─ 状态: ✗ 负期望值 (不建议下注)");
    }

    if result.optimal_fraction <= 0.0 {
        println!("    └─ 仓位建议: 0% (不下注)");
    } else if result.optimal_fraction > 1.0 {
        println!("    └─ 仓位建议: 100%+ (全仓甚至加杠杆，高风险！)");
    } else {
        println!("    └─ 仓位建议: {}", format_pct(result.optimal_fraction));
    }
    println!();

    if let Some(cap) = capital {
        println!("  基于本金 {:.2} 的投注金额:", cap);
        if result.optimal_fraction > 0.0 {
            println!("    ├─ 全凯利: {:.2}", cap * result.optimal_fraction);
            println!("    ├─ 半凯利: {:.2}", cap * result.optimal_fraction * 0.5);
            println!("    └─ 1/4凯利: {:.2}", cap * result.optimal_fraction * 0.25);
        } else {
            println!("    └─ 建议: 不下注");
        }
        println!();
    }

    separator();
}

/// 打印股票结果
fn print_result_stock(info: &StockInfo, win_rate: f64, result: &KellyResult, capital: Option<f64>) {
    println!();
    separator();
    println!("                        股票交易计算结果");
    separator();
    println!();
    println!("  输入参数:");
    println!("    ├─ 当前价: {:.2}", info.entry_price);
    println!("    ├─ 止盈价: {:.2}", info.target_price);
    println!("    ├─ 止损价: {:.2}", info.stop_loss);
    println!("    └─ 胜率 (p): {}", format_pct(win_rate));
    println!();
    println!("  风险分析:");
    println!("    ├─ 预期收益: {:.2}", info.profit);
    println!("    ├─ 风险: {:.2}", info.risk);
    println!("    └─ 盈亏比: {:.2}", info.ratio);
    println!();
    println!("  分析:");
    println!("    ├─ 净赔率 (b): {:.2}", info.ratio);
    println!("    ├─ 期望收益 (EV): {:.2}%", result.expected_value * 100.0);

    if result.positive_ev {
        println!("    ├─ 状态: ✓ 正期望值 (值得交易)");
    } else {
        println!("    ├─ 状态: ✗ 负期望值 (不建议交易)");
    }

    if result.optimal_fraction <= 0.0 {
        println!("    └─ 仓位建议: 0% (不交易)");
    } else if result.optimal_fraction > 1.0 {
        println!("    └─ 仓位建议: 100%+ (全仓甚至加杠杆，高风险！)");
    } else {
        println!("    └─ 仓位建议: {}", format_pct(result.optimal_fraction));
    }
    println!();

    if let Some(cap) = capital {
        println!("  基于本金 {:.2} 的建仓金额:", cap);
        if result.optimal_fraction > 0.0 {
            println!("    ├─ 全凯利: {:.2}", cap * result.optimal_fraction);
            println!("    ├─ 半凯利: {:.2}", cap * result.optimal_fraction * 0.5);
            println!("    └─ 1/4凯利: {:.2}", cap * result.optimal_fraction * 0.25);
        } else {
            println!("    └─ 建议: 不交易");
        }
        println!();
    }

    separator();
}

/// 交互式模式
fn interactive() {
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

        let odds: f64 = match odds_input.trim().parse() {
            Ok(n) if n > 1.0 => n,
            Ok(_) => {
                println!("✗ 赔率必须大于 1.0\n");
                continue;
            }
            Err(_) => {
                println!("✗ 无效输入\n");
                continue;
            }
        };

        println!("请输入胜率 (0-100，如 60 表示 60%):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut win_rate_input = String::new();
        io::stdin().read_line(&mut win_rate_input).unwrap();

        let win_rate_percent: f64 = match win_rate_input.trim().parse() {
            Ok(n) if n >= 0.0 && n <= 100.0 => n,
            Ok(_) => {
                println!("✗ 胜率必须在 0-100 之间\n");
                continue;
            }
            Err(_) => {
                println!("✗ 无效输入\n");
                continue;
            }
        };

        let win_rate = win_rate_percent / 100.0;

        println!("请输入本金 (可选，直接回车跳过):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match capital_input.trim().parse() {
                Ok(n) if n > 0.0 => Some(n),
                _ => {
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
fn interactive_polymarket() {
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

        let market_price: f64 = match price_input.trim().parse::<f64>() {
            Ok(n) if n > 0.0 && n <= 100.0 => n / 100.0,
            Ok(_) => {
                println!("✗ 价格必须在 0-100 之间\n");
                continue;
            }
            Err(_) => {
                println!("✗ 无效输入\n");
                continue;
            }
        };

        println!("请输入你估计的真实概率 (0-100):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut prob_input = String::new();
        io::stdin().read_line(&mut prob_input).unwrap();

        let your_probability: f64 = match prob_input.trim().parse::<f64>() {
            Ok(n) if n >= 0.0 && n <= 100.0 => n / 100.0,
            Ok(_) => {
                println!("✗ 概率必须在 0-100 之间\n");
                continue;
            }
            Err(_) => {
                println!("✗ 无效输入\n");
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
            match capital_input.trim().parse() {
                Ok(n) if n > 0.0 => Some(n),
                _ => {
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
fn interactive_stock() {
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

        let entry_price: f64 = match entry_input.trim().parse() {
            Ok(n) if n > 0.0 => n,
            _ => {
                println!("✗ 输入必须是正数\n");
                continue;
            }
        };

        println!("请输入止盈价:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut target_input = String::new();
        io::stdin().read_line(&mut target_input).unwrap();

        let target_price: f64 = match target_input.trim().parse() {
            Ok(n) if n > entry_price => n,
            _ => {
                println!("✗ 止盈价必须大于当前价\n");
                continue;
            }
        };

        println!("请输入止损价:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut stop_input = String::new();
        io::stdin().read_line(&mut stop_input).unwrap();

        let stop_loss: f64 = match stop_input.trim().parse() {
            Ok(n) if n < entry_price => n,
            _ => {
                println!("✗ 止损价必须小于当前价\n");
                continue;
            }
        };

        println!("请输入胜率 (0-100):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut win_rate_input = String::new();
        io::stdin().read_line(&mut win_rate_input).unwrap();

        let win_rate_percent: f64 = match win_rate_input.trim().parse() {
            Ok(n) if n >= 0.0 && n <= 100.0 => n,
            _ => {
                println!("✗ 胜率必须在 0-100 之间\n");
                continue;
            }
        };

        let win_rate = win_rate_percent / 100.0;

        println!("请输入本金 (可选，直接回车跳过):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match capital_input.trim().parse() {
                Ok(n) if n > 0.0 => Some(n),
                _ => {
                    println!("✗ 本金必须为正数，已跳过\n");
                    None
                }
            }
        };

        let profit = target_price - entry_price;
        let risk = entry_price - stop_loss;
        let ratio = profit / risk;

        let info = StockInfo {
            entry_price,
            target_price,
            stop_loss,
            profit,
            risk,
            ratio,
        };

        let result = kelly_stock(entry_price, target_price, stop_loss, win_rate);
        print_result_stock(&info, win_rate, &result, capital);
        println!();
    }
}

/// CLI 模式
fn cli_mode(odds: f64, win_rate: f64, capital: Option<f64>) {
    let result = kelly_criterion(odds, win_rate);
    print_result(odds, win_rate, &result, capital);
}

/// Polymarket CLI 模式
fn cli_mode_polymarket(market_price: f64, your_probability: f64, capital: Option<f64>) {
    let result = kelly_polymarket(market_price, your_probability);
    print_result_polymarket(market_price, your_probability, &result, capital);
}

/// 股票 CLI 模式
fn cli_mode_stock(entry_price: f64, target_price: f64, stop_loss: f64, win_rate: f64, capital: Option<f64>) {
    let profit = target_price - entry_price;
    let risk = entry_price - stop_loss;
    let ratio = profit / risk;

    let info = StockInfo {
        entry_price,
        target_price,
        stop_loss,
        profit,
        risk,
        ratio,
    };

    let result = kelly_stock(entry_price, target_price, stop_loss, win_rate);
    print_result_stock(&info, win_rate, &result, capital);
}

/// 打印使用说明
fn print_usage() {
    println!("用法:");
    println!("  bo                           # 交互式模式");
    println!("  bo <赔率> <胜率>              # 命令行模式");
    println!("  bo <赔率> <胜率> <本金>        # 指定本金");
    println!();
    println!("  bo -p                         # Polymarket 交互式");
    println!("  bo -p <价格> <概率>           # Polymarket 命令行");
    println!("  bo -p <价格> <概率> <本金>");
    println!();
    println!("  bo -s                         # 股票交易交互式");
    println!("  bo -s <当前价> <止盈价> <止损价> <胜率>");
    println!("  bo -s <当前价> <止盈价> <止损价> <胜率> <本金>");
    println!();
    println!("示例:");
    println!("  bo 2.0 60                    # 赔率2.0，胜率60%");
    println!("  bo 2.0 60 10000              # 本金10000");
    println!();
    println!("  bo -p 60 75                  # 市场价格60c，你认为75%");
    println!("  bo -p 60 75 1000             # 本金1000");
    println!();
    println!("  bo -s 100 120 90 60            # 当前价100，止盈120，止损90，胜率60%");
    println!("  bo -s 100 120 90 60 10000       # 本金10000");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let is_polymarket = args.iter().any(|a| a == "-p");
    let is_stock = args.iter().any(|a| a == "-s");

    if is_stock {
        let s_args: Vec<&String> = args.iter().filter(|&a| a != "-s").collect();

        match s_args.len() {
            1 => interactive_stock(),
            6 => {
                let entry: f64 = s_args[1].parse::<f64>().expect("当前价必须是数字");
                let target: f64 = s_args[2].parse::<f64>().expect("止盈价必须是数字");
                let stop: f64 = s_args[3].parse::<f64>().expect("止损价必须是数字");
                let win_rate: f64 = s_args[4].parse::<f64>().expect("胜率必须是数字") / 100.0;

                if target <= entry || stop >= entry {
                    println!("✗ 参数错误: 止盈价必须大于当前价，止损价必须小于当前价");
                } else {
                    cli_mode_stock(entry, target, stop, win_rate, None);
                }
            }
            7 => {
                let entry: f64 = s_args[1].parse::<f64>().expect("当前价必须是数字");
                let target: f64 = s_args[2].parse::<f64>().expect("止盈价必须是数字");
                let stop: f64 = s_args[3].parse::<f64>().expect("止损价必须是数字");
                let win_rate: f64 = s_args[4].parse::<f64>().expect("胜率必须是数字") / 100.0;
                let capital: f64 = s_args[5].parse::<f64>().expect("本金必须是数字");

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
    } else if is_polymarket {
        let pm_args: Vec<&String> = args.iter().filter(|&a| a != "-p").collect();

        match pm_args.len() {
            1 => interactive_polymarket(),
            3 => {
                let market_price: f64 = pm_args[1].parse::<f64>().expect("市场价格必须是数字") / 100.0;
                let your_prob: f64 = pm_args[2].parse::<f64>().expect("你的概率必须是数字") / 100.0;
                cli_mode_polymarket(market_price, your_prob, None);
            }
            4 => {
                let market_price: f64 = pm_args[1].parse::<f64>().expect("市场价格必须是数字") / 100.0;
                let your_prob: f64 = pm_args[2].parse::<f64>().expect("你的概率必须是数字") / 100.0;
                let capital: f64 = pm_args[3].parse::<f64>().expect("本金必须是数字");
                cli_mode_polymarket(market_price, your_prob, Some(capital));
            }
            _ => {
                println!("✗ Polymarket 模式参数错误");
                println!();
                println!("用法: bo -p <市场价格> <你的概率> [本金]");
                println!("示例: bo -p 60 75    # 市场价格60c，你认为75%");
            }
        }
    } else {
        match args.len() {
            1 => interactive(),
            2 => {
                if args[1] == "-h" || args[1] == "--help" {
                    print_usage();
                } else {
                    println!("✗ 参数不足");
                    print_usage();
                }
            }
            3 => {
                let odds: f64 = args[1].parse::<f64>().expect("赔率必须是数字");
                let win_rate: f64 = args[2].parse::<f64>().expect("胜率必须是数字") / 100.0;
                cli_mode(odds, win_rate, None);
            }
            4 => {
                let odds: f64 = args[1].parse::<f64>().expect("赔率必须是数字");
                let win_rate: f64 = args[2].parse::<f64>().expect("胜率必须是数字") / 100.0;
                let capital: f64 = args[3].parse::<f64>().expect("本金必须是数字");
                cli_mode(odds, win_rate, Some(capital));
            }
            _ => {
                println!("✗ 参数过多");
                print_usage();
            }
        }
    }
}
