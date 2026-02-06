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

/// 2x2 纯策略纳什均衡
#[derive(Debug, Clone)]
pub struct NashPureEquilibrium {
    /// 行玩家策略（0=上，1=下）
    pub row_strategy: usize,
    /// 列玩家策略（0=左，1=右）
    pub col_strategy: usize,
    /// 该均衡下行玩家收益
    pub row_payoff: f64,
    /// 该均衡下列玩家收益
    pub col_payoff: f64,
}

/// 2x2 混合策略纳什均衡
#[derive(Debug, Clone)]
pub struct NashMixedEquilibrium {
    /// 行玩家选择“上”策略的概率
    pub row_top_prob: f64,
    /// 列玩家选择“左”策略的概率
    pub col_left_prob: f64,
    /// 行玩家期望收益
    pub row_expected_payoff: f64,
    /// 列玩家期望收益
    pub col_expected_payoff: f64,
}

/// 2x2 纳什均衡结果
#[derive(Debug, Clone)]
pub struct NashResult {
    /// 所有纯策略纳什均衡
    pub pure_equilibria: Vec<NashPureEquilibrium>,
    /// 唯一内部混合策略均衡（若存在）
    pub mixed_equilibrium: Option<NashMixedEquilibrium>,
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

/// 组合腿来源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortfolioLegSource {
    Standard,
    Polymarket,
    Stock,
    Arbitrage2,
    ArbitrageN,
}

impl PortfolioLegSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Polymarket => "polymarket",
            Self::Stock => "stock",
            Self::Arbitrage2 => "arbitrage2",
            Self::ArbitrageN => "arbitrageN",
        }
    }
}

/// 组合凯利输入（单个标的/策略腿）
#[derive(Debug, Clone)]
pub struct PortfolioLeg {
    /// 来源类型
    pub source: PortfolioLegSource,
    /// 参数摘要，便于展示
    pub summary: String,
    /// 胜率（0-1）
    pub win_prob: f64,
    /// 胜利场景收益率（相对本金）
    pub win_return: f64,
    /// 失败场景收益率（相对本金）
    pub loss_return: f64,
}

/// 组合凯利计算结果
#[derive(Debug, Clone)]
pub struct PortfolioKellyResult {
    /// 每个标的的建议仓位（占总本金）
    pub allocations: Vec<f64>,
    /// 总仓位
    pub total_allocation: f64,
    /// 期望对数增长率 E[ln(W'/W)]
    pub expected_log_growth: f64,
    /// 期望线性收益率 E[(W'-W)/W]
    pub expected_arithmetic_return: f64,
    /// 可达状态中的最差场景资金倍数
    pub worst_case_multiplier: f64,
    /// 优化是否收敛
    pub converged: bool,
    /// 优化迭代次数
    pub iterations: usize,
}
