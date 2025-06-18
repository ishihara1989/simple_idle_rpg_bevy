#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use too_big_float::BigFloat;
    use crate::components::*;
    use crate::events::*;

    #[test]
    fn test_component_integration() {
        // Test that all component types can be created and used together
        let mut world = World::new();
        
        // Create a player entity with all stat types
        let player_entity = world.spawn((
            Player,
            BaseHp(BigFloat::from(100.0)),
            BaseAttack(BigFloat::from(10.0)),
            CurrentHp(BigFloat::from(80.0)),
            CombatAttack(BigFloat::from(12.0)),
            Experience(BigFloat::from(50.0)),
            Level(3),
        )).id();
        
        // Test that we can access individual components
        let base_hp = world.get::<BaseHp>(player_entity).unwrap();
        let base_attack = world.get::<BaseAttack>(player_entity).unwrap();
        let exp = world.get::<Experience>(player_entity).unwrap();
        let level = world.get::<Level>(player_entity).unwrap();
        
        assert_eq!(base_hp.0, BigFloat::from(100.0));
        assert_eq!(base_attack.0, BigFloat::from(10.0));
        assert_eq!(exp.0, BigFloat::from(50.0));
        assert_eq!(level.0, 3);
        
        // Test combat stats
        let current_hp = world.get::<CurrentHp>(player_entity).unwrap();
        let combat_attack = world.get::<CombatAttack>(player_entity).unwrap();
        
        assert_eq!(current_hp.0, BigFloat::from(80.0));
        assert_eq!(combat_attack.0, BigFloat::from(12.0));
    }

    #[test]
    fn test_event_creation_and_fields() {
        // Test all event types can be created and have correct fields
        let turn_event = TurnStartEvent {
            attacker: TurnAttacker::Player,
        };
        assert_eq!(turn_event.attacker, TurnAttacker::Player);
        
        let attack_event = AttackEvent {
            attacker: Entity::PLACEHOLDER,
            target: Entity::PLACEHOLDER,
            damage: BigFloat::from(15.0),
        };
        assert_eq!(attack_event.damage, BigFloat::from(15.0));
        
        let exp_event = ExpGainEvent {
            amount: BigFloat::from(30.0),
        };
        assert_eq!(exp_event.amount, BigFloat::from(30.0));
        
        let enemy_death = EnemyDeathEvent {
            enemy_entity: Entity::PLACEHOLDER,
            enemy_number: 5,
            exp_reward: BigFloat::from(25.0),
        };
        assert_eq!(enemy_death.enemy_number, 5);
        assert_eq!(enemy_death.exp_reward, BigFloat::from(25.0));
    }

    #[test]
    fn test_enemy_scaling_logic() {
        // Test the exponential growth calculation used in enemy scaling
        use crate::upgradeable_stat::calculate_exponential_growth;
        
        let base_hp = BigFloat::from(20.0);
        let multiplier = 1.5;
        
        let enemy_1_hp = calculate_exponential_growth(base_hp.clone(), multiplier, 1);
        let enemy_3_hp = calculate_exponential_growth(base_hp.clone(), multiplier, 3);
        let enemy_5_hp = calculate_exponential_growth(base_hp.clone(), multiplier, 5);
        
        // Higher level enemies should have more HP
        assert!(enemy_3_hp > enemy_1_hp);
        assert!(enemy_5_hp > enemy_3_hp);
        
        // Check actual values for enemy 1 (base_hp * multiplier^1)
        let expected_enemy_1_hp = base_hp.clone() * BigFloat::from(1.5);
        assert_eq!(enemy_1_hp, expected_enemy_1_hp);
    }
    
    #[test]
    fn test_game_state_and_enums() {
        let game_state = GameState {
            is_game_over: false,
            current_enemy_number: 10,
            current_tab: GameTab::Combat,
        };
        
        assert!(!game_state.is_game_over);
        assert_eq!(game_state.current_enemy_number, 10);
        assert_eq!(game_state.current_tab, GameTab::Combat);
        
        // Test enum equality
        assert_eq!(GameTab::Combat, GameTab::Combat);
        assert_ne!(GameTab::Combat, GameTab::Rebirth);
        
        assert_eq!(TurnAttacker::Player, TurnAttacker::Player);
        assert_ne!(TurnAttacker::Player, TurnAttacker::Enemy);
    }

    #[test]
    fn test_component_bundles() {
        // Test that we can create entities with multiple related components
        let mut world = World::new();
        
        // Player entity with management stats
        let player = world.spawn((
            Player,
            BaseHp(BigFloat::from(200.0)),
            BaseAttack(BigFloat::from(25.0)),
            BaseDefense(BigFloat::from(15.0)),
            BaseSpeed(BigFloat::from(3.0)),
            Experience(BigFloat::from(100.0)),
            Level(5),
            RebirthPoints(BigFloat::from(2.0)),
        )).id();
        
        // Enemy entity with combat stats
        let enemy = world.spawn((
            Enemy,
            CurrentHp(BigFloat::from(150.0)),
            MaxHp(BigFloat::from(150.0)),
            CombatAttack(BigFloat::from(20.0)),
            CombatDefense(BigFloat::from(10.0)),
            CombatSpeed(BigFloat::from(2.0)),
            ExpReward(BigFloat::from(40.0)),
            EnemyNumber(7),
        )).id();
        
        // Verify player exists and has all components
        assert!(world.get::<Player>(player).is_some());
        assert!(world.get::<BaseHp>(player).is_some());
        assert!(world.get::<Experience>(player).is_some());
        
        // Verify enemy exists and has all components
        assert!(world.get::<Enemy>(enemy).is_some());
        assert!(world.get::<CurrentHp>(enemy).is_some());
        assert!(world.get::<ExpReward>(enemy).is_some());
        
        // Verify separation - player doesn't have enemy components
        assert!(world.get::<Enemy>(player).is_none());
        assert!(world.get::<Player>(enemy).is_none());
    }
}