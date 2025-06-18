#[cfg(test)]
mod tests {
    use crate::components::*;
    use too_big_float::BigFloat;

    #[test]
    fn test_management_stat_components() {
        let base_hp = BaseHp(BigFloat::from(100.0));
        let base_attack = BaseAttack(BigFloat::from(10.0));
        let base_defense = BaseDefense(BigFloat::from(5.0));
        let base_speed = BaseSpeed(BigFloat::from(1.0));
        let experience = Experience(BigFloat::from(50.0));
        let level = Level(5);
        let rebirth_points = RebirthPoints(BigFloat::from(10.0));

        assert_eq!(base_hp.0, BigFloat::from(100.0));
        assert_eq!(base_attack.0, BigFloat::from(10.0));
        assert_eq!(base_defense.0, BigFloat::from(5.0));
        assert_eq!(base_speed.0, BigFloat::from(1.0));
        assert_eq!(experience.0, BigFloat::from(50.0));
        assert_eq!(level.0, 5);
        assert_eq!(rebirth_points.0, BigFloat::from(10.0));
    }

    #[test]
    fn test_combat_stat_components() {
        let current_hp = CurrentHp(BigFloat::from(80.0));
        let max_hp = MaxHp(BigFloat::from(100.0));
        let combat_attack = CombatAttack(BigFloat::from(15.0));
        let combat_defense = CombatDefense(BigFloat::from(8.0));
        let combat_speed = CombatSpeed(BigFloat::from(2.0));
        let exp_reward = ExpReward(BigFloat::from(25.0));
        let enemy_number = EnemyNumber(3);

        assert_eq!(current_hp.0, BigFloat::from(80.0));
        assert_eq!(max_hp.0, BigFloat::from(100.0));
        assert_eq!(combat_attack.0, BigFloat::from(15.0));
        assert_eq!(combat_defense.0, BigFloat::from(8.0));
        assert_eq!(combat_speed.0, BigFloat::from(2.0));
        assert_eq!(exp_reward.0, BigFloat::from(25.0));
        assert_eq!(enemy_number.0, 3);
    }

    #[test]
    fn test_component_cloning() {
        let original_hp = BaseHp(BigFloat::from(100.0));
        let cloned_hp = original_hp.clone();
        
        assert_eq!(original_hp.0, cloned_hp.0);
        assert_eq!(original_hp, cloned_hp);
    }

    #[test]
    fn test_game_state() {
        let game_state = GameState {
            is_game_over: false,
            current_enemy_number: 5,
            current_tab: GameTab::Combat,
        };

        assert!(!game_state.is_game_over);
        assert_eq!(game_state.current_enemy_number, 5);
        assert_eq!(game_state.current_tab, GameTab::Combat);
    }

    #[test]
    fn test_game_tab_equality() {
        assert_eq!(GameTab::Combat, GameTab::Combat);
        assert_eq!(GameTab::Rebirth, GameTab::Rebirth);
        assert_ne!(GameTab::Combat, GameTab::Rebirth);
    }
}