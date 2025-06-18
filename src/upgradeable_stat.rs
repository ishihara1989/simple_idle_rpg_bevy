use bevy::prelude::*;
use too_big_float::BigFloat;

// 個別のコンポーネント - 疎結合で再利用可能
#[derive(Component, Clone, Debug, PartialEq)]
pub struct CurrentValue(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct UpgradeCost(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct UpgradeMultiplier(pub f64);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct CostMultiplier(pub f64);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct BaseValue(pub BigFloat);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Level(pub u32);

// マーカーコンポーネント - アップグレード可能なStatをグループ化
#[derive(Component, Clone, Debug)]
pub struct UpgradeableStat {
    pub name: String,
}

impl UpgradeableStat {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}

// エンティティ作成用のヘルパーBundle
#[derive(Bundle)]
pub struct UpgradeableStatBundle {
    pub upgradeable_stat: UpgradeableStat,
    pub current_value: CurrentValue,
    pub base_value: BaseValue,
    pub level: Level,
    pub upgrade_cost: UpgradeCost,
    pub upgrade_multiplier: UpgradeMultiplier,
    pub cost_multiplier: CostMultiplier,
}

impl UpgradeableStatBundle {
    pub fn new(
        name: impl Into<String>,
        base_value: BigFloat,
        initial_cost: BigFloat,
        upgrade_multiplier: f64,
        cost_multiplier: f64,
    ) -> Self {
        Self {
            upgradeable_stat: UpgradeableStat::new(name),
            current_value: CurrentValue(base_value.clone()),
            base_value: BaseValue(base_value),
            level: Level(0),
            upgrade_cost: UpgradeCost(initial_cost),
            upgrade_multiplier: UpgradeMultiplier(upgrade_multiplier),
            cost_multiplier: CostMultiplier(cost_multiplier),
        }
    }
}

// ユーティリティ関数 - アップグレード可能かチェック
pub fn can_upgrade(
    available_resource: &BigFloat,
    upgrade_cost: &UpgradeCost,
) -> bool {
    available_resource >= &upgrade_cost.0
}

// ユーティリティ関数 - レベルに基づいてCurrentValueを再計算
pub fn recalculate_current_value(
    base_value: &BaseValue,
    level: &Level,
    multiplier: &UpgradeMultiplier,
) -> BigFloat {
    let mut result = base_value.0.clone();
    for _ in 0..level.0 {
        result = result * BigFloat::from(multiplier.0);
    }
    result
}

// 従来の関数型計算（後方互換性用）
pub fn calculate_exponential_growth(
    base: BigFloat,
    multiplier: f64,
    level: u32,
) -> BigFloat {
    let multiplier_bf = BigFloat::from(multiplier);
    let mut result = base;
    for _ in 0..level {
        result = result * multiplier_bf.clone();
    }
    result
}

// 新しいアップグレードシステム - 個別コンポーネント設計
pub fn upgradeable_stat_upgrade_system(
    mut player_experience_query: Query<&mut crate::components::Experience, With<crate::components::Player>>,
    mut upgradeable_stats: Query<(
        &UpgradeableStat,
        &mut CurrentValue,
        &BaseValue,
        &mut Level,
        &mut UpgradeCost,
        &UpgradeMultiplier,
        &CostMultiplier,
    )>,
) {
    let Ok(mut player_exp) = player_experience_query.single_mut() else { return };
    
    let mut upgraded = true;
    while upgraded {
        upgraded = false;
        
        for (stat, mut current_value, base_value, mut level, mut upgrade_cost, upgrade_multiplier, cost_multiplier) in upgradeable_stats.iter_mut() {
            if can_upgrade(&player_exp.0, &upgrade_cost) {
                let cost = upgrade_cost.0.clone();
                player_exp.0 = player_exp.0.clone() - cost.clone();
                
                // レベルアップ
                level.0 += 1;
                
                // 現在値を再計算
                current_value.0 = recalculate_current_value(base_value, &level, upgrade_multiplier);
                
                // コスト更新
                upgrade_cost.0 = upgrade_cost.0.clone() * BigFloat::from(cost_multiplier.0);
                
                upgraded = true;
                println!("DEBUG UPGRADE: {} upgraded! New level: {}, New value: {}, Cost was: {}", 
                    stat.name, level.0, current_value.0, cost);
            }
        }
    }
}

// BaseValue、Level、UpgradeMultiplierが変更されたときの再計算システム
pub fn update_current_value_on_change(
    mut query: Query<(
        &mut CurrentValue,
        &BaseValue,
        &Level,
        &UpgradeMultiplier,
    ), Or<(Changed<BaseValue>, Changed<Level>, Changed<UpgradeMultiplier>)>>,
) {
    for (mut current_value, base_value, level, multiplier) in query.iter_mut() {
        current_value.0 = recalculate_current_value(base_value, level, multiplier);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgradeable_stat_creation() {
        let stat = UpgradeableStat::new("Attack");
        assert_eq!(stat.name, "Attack");
    }

    #[test]
    fn test_upgradeable_stat_bundle_creation() {
        let bundle = UpgradeableStatBundle::new(
            "Attack",
            BigFloat::from(10.0),
            BigFloat::from(100.0),
            1.15,
            1.3,
        );
        
        assert_eq!(bundle.upgradeable_stat.name, "Attack");
        assert_eq!(bundle.current_value.0, BigFloat::from(10.0));
        assert_eq!(bundle.base_value.0, BigFloat::from(10.0));
        assert_eq!(bundle.level.0, 0);
        assert_eq!(bundle.upgrade_cost.0, BigFloat::from(100.0));
        assert_eq!(bundle.upgrade_multiplier.0, 1.15);
        assert_eq!(bundle.cost_multiplier.0, 1.3);
    }

    #[test]
    fn test_can_upgrade() {
        let upgrade_cost = UpgradeCost(BigFloat::from(100.0));
        
        assert!(can_upgrade(&BigFloat::from(100.0), &upgrade_cost));
        assert!(can_upgrade(&BigFloat::from(150.0), &upgrade_cost));
        assert!(!can_upgrade(&BigFloat::from(50.0), &upgrade_cost));
    }

    #[test]
    fn test_recalculate_current_value() {
        let base_value = BaseValue(BigFloat::from(10.0));
        let level = Level(3);
        let multiplier = UpgradeMultiplier(1.5);
        
        let result = recalculate_current_value(&base_value, &level, &multiplier);
        
        let expected = BigFloat::from(10.0) * BigFloat::from(1.5) * BigFloat::from(1.5) * BigFloat::from(1.5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_individual_components() {
        let current_value = CurrentValue(BigFloat::from(10.0));
        let upgrade_cost = UpgradeCost(BigFloat::from(100.0));
        let upgrade_multiplier = UpgradeMultiplier(1.15);
        let cost_multiplier = CostMultiplier(1.3);
        let base_value = BaseValue(BigFloat::from(10.0));
        let level = Level(0);
        
        assert_eq!(current_value.0, BigFloat::from(10.0));
        assert_eq!(upgrade_cost.0, BigFloat::from(100.0));
        assert_eq!(upgrade_multiplier.0, 1.15);
        assert_eq!(cost_multiplier.0, 1.3);
        assert_eq!(base_value.0, BigFloat::from(10.0));
        assert_eq!(level.0, 0);
    }

    #[test]
    fn test_exponential_growth() {
        let result = calculate_exponential_growth(
            BigFloat::from(10.0),
            1.5,
            3,
        );
        
        let expected = BigFloat::from(10.0) * BigFloat::from(1.5) * BigFloat::from(1.5) * BigFloat::from(1.5);
        assert_eq!(result, expected);
    }
}