//! 显示输出相关功能

use crate::types::{ArbitrageResult, KellyResult, MultiArbitrageResult, StockInfo};

/// 格式化百分比
pub fn format_pct(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}

/// 打印分隔线
pub fn separator() {
    println!("{}", "─".repeat(50));
}

/// 打印标准凯利标题
pub fn print_title() {
    separator();
    println!("                    凯利公式计算器");
    separator();
    println!();
}

/// 打印 Polymarket 标题
pub fn print_title_polymarket() {
    separator();
    println!("                Polymarket 凯利计算器");
    println!("            Kelly Criterion for Polymarket");
    separator();
    println!();
}

/// 打印股票标题
pub fn print_title_stock() {
    separator();
    println!("                    股票交易凯利计算器");
    separator();
    println!();
}

/// 打印套利标题
pub fn print_title_arbitrage() {
    separator();
    println!("                      套利/抽水计算器");
    separator();
    println!();
}

/// 打印标准凯利结果
pub fn print_result(odds: f64, win_rate: f64, result: &KellyResult, capital: Option<f64>) {
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
pub fn print_result_polymarket(market_price: f64, your_probability: f64, result: &KellyResult, capital: Option<f64>) {
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
pub fn print_result_stock(info: &StockInfo, win_rate: f64, result: &KellyResult, capital: Option<f64>) {
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

/// 打印套利结果
pub fn print_result_arbitrage(odds1: f64, odds2: f64, result: &ArbitrageResult, capital: Option<f64>) {
    println!();
    separator();
    println!("                      套利/抽水计算结果");
    separator();
    println!();
    println!("  输入参数:");
    println!("    ├─ 方案1赔率: {:.2}", odds1);
    println!("    ├─ 方案2赔率: {:.2}", odds2);
    println!();
    println!("  分析:");
    println!("    ├─ 方案1隐含概率: {:.2}%", (1.0 / odds1) * 100.0);
    println!("    ├─ 方案2隐含概率: {:.2}%", (1.0 / odds2) * 100.0);
    println!("    └─ 隐含概率之和: {:.2}%", result.total_implied_prob * 100.0);
    println!();

    if result.has_arbitrage {
        println!("  ✓ 套利机会存在！");
        println!("    ├─ 套利收益率: {:.2}%", result.arbitrage_profit * 100.0);
        println!("    ├─ 方案1投注比例: {:.2}%", result.stake1_ratio * 100.0);
        println!("    └─ 方案2投注比例: {:.2}%", result.stake2_ratio * 100.0);
        println!();

        if let Some(cap) = capital {
            println!("  基于本金 {:.2} 的投注方案:", cap);
            let stake1 = cap * result.stake1_ratio;
            let stake2 = cap * result.stake2_ratio;
            let total_return = cap * (1.0 + result.arbitrage_profit);
            println!("    ├─ 方案1投注: {:.2}", stake1);
            println!("    ├─ 方案2投注: {:.2}", stake2);
            println!("    └─ 获胜总回报: {:.2} (收益: {:.2})", total_return, total_return - cap);
            println!();
        }
    } else {
        println!("  ✗ 无套利机会");
        println!("    └─ 庄家抽水: {:.2}%", result.juice_rate * 100.0);
        println!();
    }

    separator();
}

/// 打印多标的套利结果
pub fn print_result_multi_arbitrage(odds: &[f64], result: &MultiArbitrageResult, capital: Option<f64>) {
    println!();
    separator();
    println!("                      多标的套利/抽水计算结果");
    separator();
    println!();
    println!("  输入参数 ({}个标的):", odds.len());
    for (i, &o) in odds.iter().enumerate() {
        println!("    ├─ 标的{}赔率: {:.2}", i + 1, o);
    }
    println!();
    println!("  分析:");
    for (i, &o) in odds.iter().enumerate() {
        println!("    ├─ 标的{}隐含概率: {:.2}%", i + 1, (1.0 / o) * 100.0);
    }
    println!("    └─ 隐含概率之和: {:.2}%", result.total_implied_prob * 100.0);
    println!();

    if result.has_arbitrage {
        println!("  ✓ 套利机会存在！");
        println!("    ├─ 套利收益率: {:.2}%", result.arbitrage_profit * 100.0);
        println!("    └─ 投注比例分配:");
        for (i, ratio) in result.stake_ratios.iter().enumerate() {
            println!("       ├─ 标的{}: {:.2}%", i + 1, ratio * 100.0);
        }
        println!();

        if let Some(cap) = capital {
            println!("  基于本金 {:.2} 的投注方案:", cap);
            let total_return = cap * (1.0 + result.arbitrage_profit);
            for (i, ratio) in result.stake_ratios.iter().enumerate() {
                let stake = cap * ratio;
                println!("    ├─ 标的{}投注: {:.2}", i + 1, stake);
            }
            println!("    └─ 获胜总回报: {:.2} (收益: {:.2})", total_return, total_return - cap);
            println!();
        }
    } else {
        println!("  ✗ 无套利机会");
        println!("    └─ 庄家抽水: {:.2}%", result.juice_rate * 100.0);
        println!();
    }

    separator();
}

/// 打印使用说明
pub fn print_usage() {
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
    println!("  bo -a                         # 套利交互式");
    println!("  bo -a <赔率1> <赔率2>         # 套利命令行");
    println!("  bo -a <赔率1> <赔率2> <本金>");
    println!();
    println!("  bo -A <标的数量> <赔率1> ... <赔率N> [本金]  # 多标的套利");
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
    println!();
    println!("  bo -a 1.9 2.1                # 方案1赔率1.9，方案2赔率2.1");
    println!("  bo -a 1.9 2.1 1000            # 本金1000");
    println!();
    println!("  bo -A 3 2.0 3.5 4.0           # 3个标的，赔率分别为2.0, 3.5, 4.0");
    println!("  bo -A 3 2.0 3.5 4.0 1000      # 本金1000");
}
