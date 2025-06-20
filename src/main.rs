use bevy::prelude::*;
use clap::Parser;
use simple_idle_rpg::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable balance check mode (no UI, auto-report game state)
    #[arg(short, long)]
    balance_check: bool,
    
    /// Set initial player level
    #[arg(long, default_value = "1")]
    level: u32,
    
    /// Set initial player experience
    #[arg(long, default_value = "0")]
    experience: u64,
    
    /// Set initial HP upgrade level
    #[arg(long, default_value = "0")]
    hp_level: u32,
    
    /// Set initial Attack upgrade level
    #[arg(long, default_value = "0")]
    attack_level: u32,
    
    /// Set initial Defense upgrade level
    #[arg(long, default_value = "0")]
    defense_level: u32,
    
    /// Set initial Speed upgrade level
    #[arg(long, default_value = "0")]
    speed_level: u32,
    
    /// Duration for balance check mode in seconds
    #[arg(long, default_value = "60")]
    duration: u64,
}

fn main() {
    let args = Args::parse();
    
    let mut app = App::new();
    
    // 起動設定をリソースとして追加
    app.insert_resource(StartupConfig {
        level: args.level,
        experience: args.experience,
        hp_level: args.hp_level,
        attack_level: args.attack_level,
        defense_level: args.defense_level,
        speed_level: args.speed_level,
        duration: args.duration,
    });
    
    if args.balance_check {
        println!("Starting in Balance Check Mode...");
        if args.level > 1 || args.experience > 0 || 
           args.hp_level > 0 || args.attack_level > 0 || 
           args.defense_level > 0 || args.speed_level > 0 {
            println!("Initial State: Level={}, Exp={}, HP_LV={}, ATK_LV={}, DEF_LV={}, SPD_LV={}", 
                     args.level, args.experience, args.hp_level, 
                     args.attack_level, args.defense_level, args.speed_level);
        }
        println!("Duration: {} seconds", args.duration);
        
        // ヘッドレスモード：UIなし、最小限のプラグイン
        app.add_plugins(MinimalPlugins)
           .add_plugins((
               PlayerPlugin,
               CombatPlugin,
               StatsPlugin,
               BalanceCheckPlugin,
           ));
    } else {
        // 通常モード：UI付き
        app.add_plugins(DefaultPlugins)
           .add_plugins((
               PlayerPlugin,
               CombatPlugin,
               StatsPlugin,
               UIPlugin,
           ));
    }
    
    app.run();
}

