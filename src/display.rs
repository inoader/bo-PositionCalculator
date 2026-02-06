//! 显示输出相关功能

use crate::types::{
    ArbitrageResult, KellyResult, MultiArbitrageResult, NashResult, PortfolioKellyResult,
    PortfolioLeg, StockInfo,
};

// EV 以百分比显示到小数点后两位，这里使用对应阈值避免出现“显示 0.00% 但判定正/负期望”。
const EV_EPSILON: f64 = 0.00005;

/// 格式化百分比
pub fn format_pct(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}

fn safe_fraction(value: f64) -> f64 {
    if value.is_finite() { value } else { 0.0 }
}

fn effective_fraction(expected_value: f64, fraction: f64) -> f64 {
    if expected_value.abs() <= EV_EPSILON {
        0.0
    } else {
        safe_fraction(fraction)
    }
}

fn print_ev_status(
    positive_ev: bool,
    expected_value: f64,
    positive_label: &str,
    negative_label: &str,
    neutral_label: &str,
) {
    if expected_value.abs() <= EV_EPSILON {
        println!("    ├─ 状态: {}", neutral_label);
    } else if positive_ev {
        println!("    ├─ 状态: {}", positive_label);
    } else {
        println!("    ├─ 状态: {}", negative_label);
    }
}

fn json_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn json_number(value: f64) -> String {
    if !value.is_finite() {
        return "null".to_string();
    }
    let mut s = format!("{:.10}", value);
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    if s == "-0" || s == "-0.0" || s.is_empty() {
        "0".to_string()
    } else {
        s
    }
}

fn json_optional_number(value: Option<f64>) -> String {
    match value {
        Some(v) => json_number(v),
        None => "null".to_string(),
    }
}

fn json_array(values: &[f64]) -> String {
    let parts: Vec<String> = values.iter().map(|&v| json_number(v)).collect();
    format!("[{}]", parts.join(","))
}

fn json_matrix_2x2(matrix: [[f64; 2]; 2]) -> String {
    format!(
        "[[{},{}],[{},{}]]",
        json_number(matrix[0][0]),
        json_number(matrix[0][1]),
        json_number(matrix[1][0]),
        json_number(matrix[1][1])
    )
}

pub fn print_json_error(message: &str) {
    println!(r#"{{"ok":false,"error":"{}"}}"#, json_escape(message));
}

/// 打印分隔线
pub fn separator() {
    println!("{}", "─".repeat(50));
}

/// 打印标准凯利标题
pub fn print_title() {
    separator();
    println!("                    仓位管理计算器");
    separator();
    println!();
}

/// 打印 Polymarket 标题
pub fn print_title_polymarket() {
    separator();
    println!("              Polymarket 仓位管理计算器");
    println!("             Position Sizing for Polymarket");
    separator();
    println!();
}

/// 打印股票标题
pub fn print_title_stock() {
    separator();
    println!("                    股票交易仓位计算器");
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

/// 打印组合凯利标题
pub fn print_title_portfolio() {
    separator();
    println!("                      组合仓位计算器");
    separator();
    println!();
}

/// 打印纳什均衡标题
pub fn print_title_nash() {
    separator();
    println!("                      纳什均衡计算器");
    println!("                    2x2 Normal Form Game");
    separator();
    println!();
}

/// 打印标准凯利结果
pub fn print_result(odds: f64, win_rate: f64, result: &KellyResult, capital: Option<f64>) {
    let fraction = effective_fraction(result.expected_value, result.optimal_fraction);

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
    println!(
        "    ├─ 期望收益 (EV): {:.2}%",
        result.expected_value * 100.0
    );

    print_ev_status(
        result.positive_ev,
        result.expected_value,
        "✓ 正期望值 (值得下注)",
        "✗ 负期望值 (不建议下注)",
        "○ 中性期望值 (长期不赚不亏，建议不下注)",
    );

    if fraction <= 0.0 {
        println!("    └─ 仓位建议: 0% (不下注)");
    } else if fraction > 1.0 {
        println!("    └─ 仓位建议: 100%+ (全仓甚至加杠杆，高风险！)");
    } else {
        println!("    └─ 仓位建议: {}", format_pct(fraction));
    }
    println!();

    if let Some(cap) = capital {
        println!("  基于本金 {:.2} 的投注金额:", cap);
        if fraction > 0.0 {
            println!("    ├─ 全凯利: {:.2}", cap * fraction);
            println!("    ├─ 半凯利: {:.2}", cap * fraction * 0.5);
            println!("    └─ 1/4凯利: {:.2}", cap * fraction * 0.25);
        } else {
            println!("    └─ 建议: 不下注");
        }
        println!();
    }

    separator();
}

/// 打印 Polymarket 结果
pub fn print_result_polymarket(
    market_price: f64,
    your_probability: f64,
    result: &KellyResult,
    capital: Option<f64>,
) {
    let fraction = effective_fraction(result.expected_value, result.optimal_fraction);

    println!();
    separator();
    println!("                    Polymarket 计算结果");
    separator();
    println!();
    println!("  输入参数:");
    println!(
        "    ├─ 市场价格: {:.4}% (市场隐含概率)",
        market_price * 100.0
    );
    println!(
        "    ├─ 你的概率: {} (你估计的真实概率)",
        format_pct(your_probability)
    );
    println!("    └─ 隐含赔率: {:.6}", 1.0 / market_price);
    println!();
    println!("  分析:");
    println!(
        "    ├─ 期望收益 (EV): {:.2}%",
        result.expected_value * 100.0
    );

    print_ev_status(
        result.positive_ev,
        result.expected_value,
        "✓ 正期望值 (值得下注)",
        "✗ 负期望值 (不建议下注)",
        "○ 中性期望值 (长期不赚不亏，建议不下注)",
    );

    if fraction <= 0.0 {
        println!("    └─ 仓位建议: 0% (不下注)");
    } else if fraction > 1.0 {
        println!("    └─ 仓位建议: 100%+ (全仓甚至加杠杆，高风险！)");
    } else {
        println!("    └─ 仓位建议: {}", format_pct(fraction));
    }
    println!();

    if let Some(cap) = capital {
        println!("  基于本金 {:.2} 的投注金额:", cap);
        if fraction > 0.0 {
            println!("    ├─ 全凯利: {:.2}", cap * fraction);
            println!("    ├─ 半凯利: {:.2}", cap * fraction * 0.5);
            println!("    └─ 1/4凯利: {:.2}", cap * fraction * 0.25);
        } else {
            println!("    └─ 建议: 不下注");
        }
        println!();
    }

    separator();
}

/// 打印股票结果
pub fn print_result_stock(
    info: &StockInfo,
    win_rate: f64,
    result: &KellyResult,
    capital: Option<f64>,
) {
    let risk_fraction = effective_fraction(result.expected_value, result.optimal_fraction);
    let stop_loss_pct = info.risk / info.entry_price;
    let position_fraction = if stop_loss_pct > 0.0 {
        risk_fraction / stop_loss_pct
    } else {
        0.0
    };

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
    println!("    ├─ 止损幅度: {}", format_pct(stop_loss_pct));
    println!("    └─ 盈亏比: {:.2}", info.ratio);
    println!();
    println!("  分析:");
    println!("    ├─ 净赔率 (b): {:.2}", info.ratio);
    println!(
        "    ├─ 期望收益 (EV): {:.2}%",
        result.expected_value * 100.0
    );

    print_ev_status(
        result.positive_ev,
        result.expected_value,
        "✓ 正期望值 (值得交易)",
        "✗ 负期望值 (不建议交易)",
        "○ 中性期望值 (长期不赚不亏，建议不交易)",
    );

    if position_fraction <= 0.0 {
        println!("    ├─ 风险建议: 0% (不交易)");
        println!("    └─ 建仓仓位: 0% (不交易)");
    } else {
        println!("    ├─ 风险建议: {}", format_pct(risk_fraction));
        if position_fraction > 1.0 {
            println!(
                "    └─ 建仓仓位: {} (需杠杆 {:.2}x)",
                format_pct(position_fraction),
                position_fraction
            );
        } else {
            println!("    └─ 建仓仓位: {}", format_pct(position_fraction));
        }
    }
    println!();

    if let Some(cap) = capital {
        println!("  基于本金 {:.2} 的仓位金额:", cap);
        if position_fraction > 0.0 {
            let full_risk = cap * risk_fraction;
            let half_risk = full_risk * 0.5;
            let quarter_risk = full_risk * 0.25;
            println!("    ├─ 全凯利风险金: {:.2}", full_risk);
            println!("    ├─ 半凯利风险金: {:.2}", half_risk);
            println!("    ├─ 1/4凯利风险金: {:.2}", quarter_risk);
            println!("    ├─ 全凯利建仓: {:.2}", cap * position_fraction);
            println!("    ├─ 半凯利建仓: {:.2}", cap * (position_fraction * 0.5));
            println!(
                "    └─ 1/4凯利建仓: {:.2}",
                cap * (position_fraction * 0.25)
            );
        } else {
            println!("    └─ 建议: 不交易");
        }
        println!();
    }

    separator();
}

/// 打印套利结果
pub fn print_result_arbitrage(
    odds1: f64,
    odds2: f64,
    result: &ArbitrageResult,
    capital: Option<f64>,
) {
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
    println!(
        "    └─ 隐含概率之和: {:.2}%",
        result.total_implied_prob * 100.0
    );
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
            println!(
                "    └─ 获胜总回报: {:.2} (收益: {:.2})",
                total_return,
                total_return - cap
            );
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
pub fn print_result_multi_arbitrage(
    odds: &[f64],
    result: &MultiArbitrageResult,
    capital: Option<f64>,
) {
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
    println!(
        "    └─ 隐含概率之和: {:.2}%",
        result.total_implied_prob * 100.0
    );
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
            println!(
                "    └─ 获胜总回报: {:.2} (收益: {:.2})",
                total_return,
                total_return - cap
            );
            println!();
        }
    } else {
        println!("  ✗ 无套利机会");
        println!("    └─ 庄家抽水: {:.2}%", result.juice_rate * 100.0);
        println!();
    }

    separator();
}

/// 打印组合凯利结果
pub fn print_result_portfolio(
    legs: &[PortfolioLeg],
    result: &PortfolioKellyResult,
    capital: Option<f64>,
) {
    println!();
    separator();
    println!("                      组合凯利计算结果");
    separator();
    println!();
    println!("  输入参数 ({}个标的):", legs.len());
    for (i, leg) in legs.iter().enumerate() {
        let edge = leg.win_prob * leg.win_return + (1.0 - leg.win_prob) * leg.loss_return;
        println!(
            "    ├─ 标的{} [{}]: {} / EV {:.2}%",
            i + 1,
            leg.source.as_str(),
            leg.summary,
            edge * 100.0
        );
    }
    println!();
    println!("  组合分析:");
    println!("    ├─ 总仓位: {}", format_pct(result.total_allocation));
    println!(
        "    ├─ 最差场景资金倍数: {:.4}",
        result.worst_case_multiplier
    );
    println!(
        "    ├─ 期望线性收益: {:.2}%",
        result.expected_arithmetic_return * 100.0
    );
    println!(
        "    ├─ 期望对数增长: {:.4}%",
        result.expected_log_growth * 100.0
    );
    println!(
        "    └─ 收敛状态: {} (迭代 {} 次)",
        if result.converged {
            "已收敛"
        } else {
            "达到迭代上限"
        },
        result.iterations
    );
    println!();
    println!("  仓位分配:");
    for (i, alloc) in result.allocations.iter().enumerate() {
        println!("    ├─ 标的{}: {}", i + 1, format_pct(*alloc));
    }
    println!();

    if let Some(cap) = capital {
        println!("  基于本金 {:.2} 的分配金额:", cap);
        let full_used: f64 = result.allocations.iter().map(|a| cap * a).sum();
        for (i, alloc) in result.allocations.iter().enumerate() {
            println!(
                "    ├─ 标的{}: 全凯利 {:.2} / 半凯利 {:.2} / 1/4凯利 {:.2}",
                i + 1,
                cap * alloc,
                cap * alloc * 0.5,
                cap * alloc * 0.25
            );
        }
        println!(
            "    ├─ 全凯利剩余现金: {:.2}",
            cap * (1.0 - result.total_allocation).max(0.0)
        );
        println!(
            "    └─ 全凯利总投入: {:.2} (占比 {})",
            full_used,
            format_pct(result.total_allocation)
        );
        println!();
    }

    separator();
}

/// 打印纳什均衡结果
pub fn print_result_nash(
    row_payoffs: [[f64; 2]; 2],
    col_payoffs: [[f64; 2]; 2],
    result: &NashResult,
) {
    println!();
    separator();
    println!("                      2x2 纳什均衡结果");
    separator();
    println!();
    println!("  输入收益矩阵:");
    println!(
        "    ├─ 行玩家 A: [[{:.4}, {:.4}], [{:.4}, {:.4}]]",
        row_payoffs[0][0], row_payoffs[0][1], row_payoffs[1][0], row_payoffs[1][1]
    );
    println!(
        "    └─ 列玩家 B: [[{:.4}, {:.4}], [{:.4}, {:.4}]]",
        col_payoffs[0][0], col_payoffs[0][1], col_payoffs[1][0], col_payoffs[1][1]
    );
    println!();

    println!("  纯策略纳什均衡:");
    if result.pure_equilibria.is_empty() {
        println!("    └─ 无");
    } else {
        for (idx, eq) in result.pure_equilibria.iter().enumerate() {
            let row_label = if eq.row_strategy == 0 { "上" } else { "下" };
            let col_label = if eq.col_strategy == 0 { "左" } else { "右" };
            println!(
                "    ├─ 均衡{}: (行 {}, 列 {}) -> 行收益 {:.4}, 列收益 {:.4}",
                idx + 1,
                row_label,
                col_label,
                eq.row_payoff,
                eq.col_payoff
            );
        }
    }
    println!();

    println!("  混合策略纳什均衡:");
    if let Some(mixed) = &result.mixed_equilibrium {
        println!(
            "    ├─ 行玩家: 上 {:.2}%, 下 {:.2}%",
            mixed.row_top_prob * 100.0,
            (1.0 - mixed.row_top_prob) * 100.0
        );
        println!(
            "    ├─ 列玩家: 左 {:.2}%, 右 {:.2}%",
            mixed.col_left_prob * 100.0,
            (1.0 - mixed.col_left_prob) * 100.0
        );
        println!("    ├─ 行玩家期望收益: {:.4}", mixed.row_expected_payoff);
        println!("    └─ 列玩家期望收益: {:.4}", mixed.col_expected_payoff);
    } else {
        println!("    └─ 无内部混合策略均衡（或解不唯一）");
    }
    println!();

    separator();
}

/// 打印标准凯利 JSON 结果
pub fn print_result_json(odds: f64, win_rate: f64, result: &KellyResult, capital: Option<f64>) {
    let fraction = effective_fraction(result.expected_value, result.optimal_fraction);
    let sizing = match capital {
        Some(cap) => format!(
            r#"{{"full_kelly":{},"half_kelly":{},"quarter_kelly":{}}}"#,
            json_number(cap * fraction),
            json_number(cap * fraction * 0.5),
            json_number(cap * fraction * 0.25)
        ),
        None => "null".to_string(),
    };

    println!(
        r#"{{"ok":true,"mode":"standard","inputs":{{"odds":{},"win_rate":{},"capital":{}}},"result":{{"expected_value":{},"positive_ev":{},"optimal_fraction":{},"recommended_fraction":{}}},"sizing":{}}}"#,
        json_number(odds),
        json_number(win_rate),
        json_optional_number(capital),
        json_number(result.expected_value),
        result.positive_ev,
        json_number(result.optimal_fraction),
        json_number(fraction),
        sizing
    );
}

/// 打印 Polymarket JSON 结果
pub fn print_result_polymarket_json(
    market_price: f64,
    your_probability: f64,
    result: &KellyResult,
    capital: Option<f64>,
) {
    let fraction = effective_fraction(result.expected_value, result.optimal_fraction);
    let sizing = match capital {
        Some(cap) => format!(
            r#"{{"full_kelly":{},"half_kelly":{},"quarter_kelly":{}}}"#,
            json_number(cap * fraction),
            json_number(cap * fraction * 0.5),
            json_number(cap * fraction * 0.25)
        ),
        None => "null".to_string(),
    };

    println!(
        r#"{{"ok":true,"mode":"polymarket","inputs":{{"market_price":{},"your_probability":{},"implied_odds":{},"capital":{}}},"result":{{"expected_value":{},"positive_ev":{},"optimal_fraction":{},"recommended_fraction":{}}},"sizing":{}}}"#,
        json_number(market_price),
        json_number(your_probability),
        json_number(1.0 / market_price),
        json_optional_number(capital),
        json_number(result.expected_value),
        result.positive_ev,
        json_number(result.optimal_fraction),
        json_number(fraction),
        sizing
    );
}

/// 打印股票 JSON 结果
pub fn print_result_stock_json(
    info: &StockInfo,
    win_rate: f64,
    result: &KellyResult,
    capital: Option<f64>,
) {
    let risk_fraction = effective_fraction(result.expected_value, result.optimal_fraction);
    let stop_loss_pct = info.risk / info.entry_price;
    let position_fraction = if stop_loss_pct > 0.0 {
        risk_fraction / stop_loss_pct
    } else {
        0.0
    };
    let leverage = if position_fraction > 1.0 {
        Some(position_fraction)
    } else {
        None
    };

    let sizing = match capital {
        Some(cap) => format!(
            r#"{{"risk":{{"full":{},"half":{},"quarter":{}}},"position":{{"full":{},"half":{},"quarter":{}}}}}"#,
            json_number(cap * risk_fraction),
            json_number(cap * risk_fraction * 0.5),
            json_number(cap * risk_fraction * 0.25),
            json_number(cap * position_fraction),
            json_number(cap * position_fraction * 0.5),
            json_number(cap * position_fraction * 0.25)
        ),
        None => "null".to_string(),
    };

    println!(
        r#"{{"ok":true,"mode":"stock","inputs":{{"entry_price":{},"target_price":{},"stop_loss":{},"win_rate":{},"capital":{}}},"analysis":{{"profit":{},"risk":{},"stop_loss_pct":{},"ratio":{}}},"result":{{"expected_value":{},"positive_ev":{},"risk_fraction":{},"position_fraction":{},"leverage":{}}},"sizing":{}}}"#,
        json_number(info.entry_price),
        json_number(info.target_price),
        json_number(info.stop_loss),
        json_number(win_rate),
        json_optional_number(capital),
        json_number(info.profit),
        json_number(info.risk),
        json_number(stop_loss_pct),
        json_number(info.ratio),
        json_number(result.expected_value),
        result.positive_ev,
        json_number(risk_fraction),
        json_number(position_fraction),
        json_optional_number(leverage),
        sizing
    );
}

/// 打印双标套利 JSON 结果
pub fn print_result_arbitrage_json(
    odds1: f64,
    odds2: f64,
    result: &ArbitrageResult,
    capital: Option<f64>,
) {
    let stake_plan = match (result.has_arbitrage, capital) {
        (true, Some(cap)) => {
            let stake1 = cap * result.stake1_ratio;
            let stake2 = cap * result.stake2_ratio;
            let total_return = cap * (1.0 + result.arbitrage_profit);
            format!(
                r#"{{"stake1":{},"stake2":{},"total_return":{},"profit":{}}}"#,
                json_number(stake1),
                json_number(stake2),
                json_number(total_return),
                json_number(total_return - cap)
            )
        }
        _ => "null".to_string(),
    };

    println!(
        r#"{{"ok":true,"mode":"arbitrage","inputs":{{"odds1":{},"odds2":{},"capital":{}}},"result":{{"has_arbitrage":{},"total_implied_prob":{},"arbitrage_profit":{},"juice_rate":{},"stake_ratios":[{},{}]}},"stake_plan":{}}}"#,
        json_number(odds1),
        json_number(odds2),
        json_optional_number(capital),
        result.has_arbitrage,
        json_number(result.total_implied_prob),
        json_number(result.arbitrage_profit),
        json_number(result.juice_rate),
        json_number(result.stake1_ratio),
        json_number(result.stake2_ratio),
        stake_plan
    );
}

/// 打印多标套利 JSON 结果
pub fn print_result_multi_arbitrage_json(
    odds: &[f64],
    result: &MultiArbitrageResult,
    capital: Option<f64>,
) {
    let stake_plan = match (result.has_arbitrage, capital) {
        (true, Some(cap)) => {
            let stakes: Vec<f64> = result.stake_ratios.iter().map(|r| cap * r).collect();
            let total_return = cap * (1.0 + result.arbitrage_profit);
            format!(
                r#"{{"stakes":{},"total_return":{},"profit":{}}}"#,
                json_array(&stakes),
                json_number(total_return),
                json_number(total_return - cap)
            )
        }
        _ => "null".to_string(),
    };

    println!(
        r#"{{"ok":true,"mode":"multi_arbitrage","inputs":{{"odds":{},"capital":{}}},"result":{{"has_arbitrage":{},"total_implied_prob":{},"arbitrage_profit":{},"juice_rate":{},"stake_ratios":{}}},"stake_plan":{}}}"#,
        json_array(odds),
        json_optional_number(capital),
        result.has_arbitrage,
        json_number(result.total_implied_prob),
        json_number(result.arbitrage_profit),
        json_number(result.juice_rate),
        json_array(&result.stake_ratios),
        stake_plan
    );
}

/// 打印纳什均衡 JSON 结果
pub fn print_result_nash_json(
    row_payoffs: [[f64; 2]; 2],
    col_payoffs: [[f64; 2]; 2],
    result: &NashResult,
) {
    let pure_equilibria = result
        .pure_equilibria
        .iter()
        .map(|eq| {
            format!(
                r#"{{"row_strategy":{},"col_strategy":{},"row_payoff":{},"col_payoff":{}}}"#,
                eq.row_strategy,
                eq.col_strategy,
                json_number(eq.row_payoff),
                json_number(eq.col_payoff)
            )
        })
        .collect::<Vec<String>>()
        .join(",");

    let mixed_equilibrium = match &result.mixed_equilibrium {
        Some(mixed) => format!(
            r#"{{"row_top_prob":{},"col_left_prob":{},"row_expected_payoff":{},"col_expected_payoff":{}}}"#,
            json_number(mixed.row_top_prob),
            json_number(mixed.col_left_prob),
            json_number(mixed.row_expected_payoff),
            json_number(mixed.col_expected_payoff)
        ),
        None => "null".to_string(),
    };

    println!(
        r#"{{"ok":true,"mode":"nash_2x2","inputs":{{"row_payoffs":{},"col_payoffs":{}}},"result":{{"pure_equilibria":[{}],"mixed_equilibrium":{}}}}}"#,
        json_matrix_2x2(row_payoffs),
        json_matrix_2x2(col_payoffs),
        pure_equilibria,
        mixed_equilibrium
    );
}

/// 打印组合凯利 JSON 结果
pub fn print_result_portfolio_json(
    legs: &[PortfolioLeg],
    result: &PortfolioKellyResult,
    capital: Option<f64>,
) {
    let legs_json = legs
        .iter()
        .map(|leg| {
            format!(
                r#"{{"source":"{}","summary":"{}","win_prob":{},"win_return":{},"loss_return":{}}}"#,
                json_escape(leg.source.as_str()),
                json_escape(&leg.summary),
                json_number(leg.win_prob),
                json_number(leg.win_return),
                json_number(leg.loss_return)
            )
        })
        .collect::<Vec<String>>()
        .join(",");

    let sizing = match capital {
        Some(cap) => {
            let full: Vec<f64> = result.allocations.iter().map(|a| cap * a).collect();
            let half: Vec<f64> = result.allocations.iter().map(|a| cap * a * 0.5).collect();
            let quarter: Vec<f64> = result.allocations.iter().map(|a| cap * a * 0.25).collect();
            format!(
                r#"{{"full_kelly":{},"half_kelly":{},"quarter_kelly":{},"full_used":{},"full_remaining":{}}}"#,
                json_array(&full),
                json_array(&half),
                json_array(&quarter),
                json_number(full.iter().sum()),
                json_number(cap * (1.0 - result.total_allocation).max(0.0))
            )
        }
        None => "null".to_string(),
    };

    println!(
        r#"{{"ok":true,"mode":"portfolio_kelly","inputs":{{"legs":[{}],"capital":{}}},"result":{{"allocations":{},"total_allocation":{},"expected_log_growth":{},"expected_arithmetic_return":{},"worst_case_multiplier":{},"converged":{},"iterations":{}}},"sizing":{}}}"#,
        legs_json,
        json_optional_number(capital),
        json_array(&result.allocations),
        json_number(result.total_allocation),
        json_number(result.expected_log_growth),
        json_number(result.expected_arithmetic_return),
        json_number(result.worst_case_multiplier),
        result.converged,
        result.iterations,
        sizing
    );
}

/// 打印使用说明
pub fn print_usage() {
    println!("用法:");
    println!("  bo -h | -help                # 显示用法");
    println!("  bo -v | -version             # 显示版本");
    println!("  bo                           # 交互式模式");
    println!("  bo --json ...                # JSON 输出（仅命令行参数模式）");
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
    println!("  bo -n                         # 纳什均衡交互式");
    println!("  bo -n <a11> <a12> <a21> <a22> <b11> <b12> <b21> <b22>  # 2x2 纳什均衡");
    println!("  bo -k                         # 组合凯利交互式");
    println!("  bo -k <标的数量> <赔率1> <胜率1> ... <赔率N> <胜率N> [本金]  # 组合凯利");
    println!("  bo -k <descriptor1> <descriptor2> ... [本金]  # 跨模式组合凯利");
    println!(
        "     descriptor: std:赔率:胜率 | pm:市场价:概率 | stock:入场:止盈:止损:胜率 | arb:赔率1:赔率2 | marb:赔率1,赔率2,..."
    );
    println!();
    println!("示例:");
    println!("  bo 2.0 60                    # 赔率2.0，胜率60%");
    println!("  bo --json 2.0 60             # JSON 输出");
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
    println!();
    println!("  bo -n 3 0 5 1 3 5 0 1         # 囚徒困境收益矩阵");
    println!("  bo --json -n 1 -1 -1 1 -1 1 1 -1");
    println!();
    println!("  bo -k 2 2.0 60 2.5 55         # 2个标的组合凯利");
    println!("  bo -k 2 2.0 60 2.5 55 10000   # 本金10000");
    println!("  bo -k std:2.0:60 pm:60:75 stock:100:120:90:60 10000");
    println!("  bo --json -k std:2.0:60 arb:2.1:2.2 marb:2.5,4.0,5.0 10000");
}
