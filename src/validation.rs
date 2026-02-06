//! 输入校验与解析

/// 解析浮点数
pub fn parse_f64(input: &str, field_name: &str) -> Result<f64, String> {
    input.parse::<f64>().map_err(|_| format!("{field_name}必须是数字"))
}

/// 解析赔率（必须大于 1.0）
pub fn parse_odds(input: &str, field_name: &str) -> Result<f64, String> {
    let odds = parse_f64(input, field_name)?;
    if odds > 1.0 {
        Ok(odds)
    } else {
        Err(format!("{field_name}必须大于 1.0"))
    }
}

/// 解析百分比并转换为小数（0-1）
pub fn parse_percent(input: &str, field_name: &str) -> Result<f64, String> {
    let percent = parse_f64(input, field_name)?;
    if (0.0..=100.0).contains(&percent) {
        Ok(percent / 100.0)
    } else {
        Err(format!("{field_name}必须在 0-100 之间"))
    }
}

/// 解析市场价格百分比并转换为小数（0-1），市场价格不允许为 0
pub fn parse_market_price(input: &str) -> Result<f64, String> {
    let percent = parse_f64(input, "市场价格")?;
    if percent > 0.0 && percent <= 100.0 {
        Ok(percent / 100.0)
    } else {
        Err("市场价格必须在 0-100 之间，且不能为 0".to_string())
    }
}

/// 解析正数
pub fn parse_positive(input: &str, field_name: &str) -> Result<f64, String> {
    let value = parse_f64(input, field_name)?;
    if value > 0.0 {
        Ok(value)
    } else {
        Err(format!("{field_name}必须为正数"))
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_market_price, parse_odds, parse_percent, parse_positive};

    #[test]
    fn parse_market_price_rejects_zero() {
        assert!(parse_market_price("0").is_err());
    }

    #[test]
    fn parse_market_price_accepts_positive_percent() {
        assert_eq!(parse_market_price("60").unwrap(), 0.6);
    }

    #[test]
    fn parse_percent_limits_range() {
        assert!(parse_percent("-1", "胜率").is_err());
        assert!(parse_percent("101", "胜率").is_err());
        assert_eq!(parse_percent("50", "胜率").unwrap(), 0.5);
    }

    #[test]
    fn parse_odds_requires_greater_than_one() {
        assert!(parse_odds("1", "赔率").is_err());
        assert!(parse_odds("0.9", "赔率").is_err());
        assert_eq!(parse_odds("2", "赔率").unwrap(), 2.0);
    }

    #[test]
    fn parse_positive_requires_gt_zero() {
        assert!(parse_positive("0", "本金").is_err());
        assert!(parse_positive("-10", "本金").is_err());
        assert_eq!(parse_positive("10", "本金").unwrap(), 10.0);
    }
}
