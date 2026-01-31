//! 数据结构定义

/// 凯利公式计算结果
#[derive(Debug, Clone)]
pub struct KellyResult {
    /// 最优仓位比例 (0-1)
    pub optimal_fraction: f64,
    /// 是否为正期望
    pub positive_ev: bool,
    /// 期望收益
    pub expected_value: f64,
}

/// 套利机会计算结果
#[derive(Debug, Clone)]
pub struct ArbitrageResult {
    /// 是否存在套利机会
    pub has_arbitrage: bool,
    /// 隐含概率之和
    pub total_implied_prob: f64,
    /// 套利收益率（如果存在套利）
    pub arbitrage_profit: f64,
    /// 抽水率（如果不存在套利）
    pub juice_rate: f64,
    /// 方案1的投注比例
    pub stake1_ratio: f64,
    /// 方案2的投注比例
    pub stake2_ratio: f64,
}

/// 多标的套利机会计算结果
#[derive(Debug, Clone)]
pub struct MultiArbitrageResult {
    /// 是否存在套利机会
    pub has_arbitrage: bool,
    /// 隐含概率之和
    pub total_implied_prob: f64,
    /// 套利收益率（如果存在套利）
    pub arbitrage_profit: f64,
    /// 抽水率（如果不存在套利）
    pub juice_rate: f64,
    /// 各标的投注比例
    pub stake_ratios: Vec<f64>,
}

/// 股票交易信息
#[derive(Debug, Clone)]
pub struct StockInfo {
    pub entry_price: f64,
    pub target_price: f64,
    pub stop_loss: f64,
    pub profit: f64,
    pub risk: f64,
    pub ratio: f64,
}
