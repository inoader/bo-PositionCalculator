//! 资产等级计算

use crate::types::AssetLevelResult;

pub fn calculate_asset_level(amount: f64) -> AssetLevelResult {
    let digits = amount.log10().floor() + 1.0;
    let a_level = digits + amount / 10_f64.powf(digits);
    let l_level = amount.ln();

    AssetLevelResult {
        amount,
        a_level,
        l_level,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_asset_level;

    #[test]
    fn asset_level_matches_example() {
        let result = calculate_asset_level(50_000_000.0);
        assert_eq!(format!("A{:.1}", result.a_level), "A8.5");
        assert_eq!(format!("L{:.4}", result.l_level), "L17.7275");
    }
}
