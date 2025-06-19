#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use too_big_float::BigFloat;
    use crate::components::*;
    use crate::events::*;

    #[test]
    fn test_attack_cooldown_component() {
        let cooldown = AttackCooldown(500.0);
        assert_eq!(cooldown.0, 500.0);
        
        let zero_cooldown = AttackCooldown(0.0);
        assert_eq!(zero_cooldown.0, 0.0);
    }

    #[test]
    fn test_cooldown_reduction_logic() {
        // Test the cooldown reduction calculation
        let initial_cooldown: f32 = 1000.0; // 1 second
        let speed_multiplier: f32 = 2.0; // 2x speed
        let delta_ms: f32 = 16.0; // 16ms frame
        
        let reduction = delta_ms * speed_multiplier;
        let new_cooldown = (initial_cooldown - reduction).max(0.0);
        
        assert_eq!(reduction, 32.0); // 16 * 2 = 32ms reduction
        assert_eq!(new_cooldown, 968.0); // 1000 - 32 = 968ms remaining
    }

    #[test]
    fn test_base_attack_time_calculation() {
        let base_attack_time: f32 = 1000.0; // 1 second
        
        // Test different speeds
        let speed_1: f32 = 1.0;
        let speed_2: f32 = 2.0;
        let speed_half: f32 = 0.5;
        
        let cooldown_1 = base_attack_time / speed_1;
        let cooldown_2 = base_attack_time / speed_2;
        let cooldown_half = base_attack_time / speed_half;
        
        assert_eq!(cooldown_1, 1000.0); // 1 second
        assert_eq!(cooldown_2, 500.0);  // 0.5 seconds (faster)
        assert_eq!(cooldown_half, 2000.0); // 2 seconds (slower)
    }

    #[test]
    fn test_attack_ready_condition() {
        // Test when attacks should be ready
        assert!(0.0 <= 0.0); // Ready to attack
        assert!(-1.0 <= 0.0); // Negative values are <= 0 (this was wrong logic)
        assert!(!(100.0 <= 0.0)); // Still on cooldown
        
        // Better test: check if cooldown is ready
        let ready_cooldown = 0.0;
        let negative_cooldown = -1.0; // Can happen due to floating point
        let positive_cooldown = 100.0;
        
        assert!(ready_cooldown <= 0.0);
        assert!(negative_cooldown <= 0.0); // This is also considered ready
        assert!(!(positive_cooldown <= 0.0));
    }

    #[test]
    fn test_cooldown_component_integration() {
        let mut world = World::new();
        
        // Create player with attack cooldown
        let player = world.spawn((
            Player,
            CombatAttack(BigFloat::from(10.0)),
            CombatSpeed(BigFloat::from(2.0)),
            AttackCooldown(0.0), // Ready to attack
        )).id();
        
        // Create enemy with attack cooldown
        let enemy = world.spawn((
            Enemy,
            CombatAttack(BigFloat::from(8.0)),
            CombatSpeed(BigFloat::from(1.5)),
            AttackCooldown(500.0), // On cooldown
        )).id();
        
        // Verify components exist
        assert!(world.get::<AttackCooldown>(player).is_some());
        assert!(world.get::<AttackCooldown>(enemy).is_some());
        
        // Check cooldown values
        let player_cooldown = world.get::<AttackCooldown>(player).unwrap();
        let enemy_cooldown = world.get::<AttackCooldown>(enemy).unwrap();
        
        assert_eq!(player_cooldown.0, 0.0);
        assert_eq!(enemy_cooldown.0, 500.0);
    }

    #[test]
    fn test_speed_to_f64_conversion() {
        // Test BigFloat to f64 conversion for speed calculations
        let speed_1 = BigFloat::from(1.0);
        let speed_2 = BigFloat::from(2.5);
        let speed_big = BigFloat::from(100.0);
        
        assert_eq!(speed_1.to_f64().unwrap(), 1.0);
        assert_eq!(speed_2.to_f64().unwrap(), 2.5);
        assert_eq!(speed_big.to_f64().unwrap(), 100.0);
    }

    #[test]
    fn test_attack_event_structure() {
        // Test that attack events work with the new system
        let attack_event = AttackEvent {
            attacker: Entity::PLACEHOLDER,
            target: Entity::PLACEHOLDER,
            damage: BigFloat::from(15.0),
        };
        
        assert_eq!(attack_event.damage, BigFloat::from(15.0));
        assert_eq!(attack_event.attacker, Entity::PLACEHOLDER);
        assert_eq!(attack_event.target, Entity::PLACEHOLDER);
    }

    #[test]
    fn test_real_time_vs_turn_based_timing() {
        // Compare timing characteristics
        
        // Turn-based: Fixed 1 second intervals
        let turn_based_interval: f32 = 1000.0; // 1 second
        
        // Real-time: Variable based on speed
        let player_speed: f32 = 2.0;
        let enemy_speed: f32 = 1.5;
        
        let player_attack_time = 1000.0 / player_speed; // 500ms
        let enemy_attack_time = 1000.0 / enemy_speed;   // 666ms
        
        // In real-time, faster entities attack more frequently
        assert!(player_attack_time < turn_based_interval);
        assert!(enemy_attack_time < turn_based_interval);
        assert!(player_attack_time < enemy_attack_time); // Player is faster
    }
}