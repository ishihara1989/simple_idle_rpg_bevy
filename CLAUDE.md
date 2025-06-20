# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Running
- `cargo run` - Build and run the game (normal mode with UI)
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version

### Balance Testing (Headless Mode)
- `cargo run -- --balance-check` - Run balance check mode (no UI, 60 seconds)
- `cargo run -- --balance-check --duration 30` - Custom duration
- `cargo run -- --balance-check --level 5 --hp-level 3` - Custom initial state
- `cargo run -- --help` - Show all available command line options

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test by name
- `cargo test integration` - Run integration tests
- `cargo test components` - Run component tests
- `cargo test real_time_combat` - Run real-time combat tests

## Architecture Overview

This is a real-time idle RPG built with **Rust + Bevy ECS 0.16.1**. The architecture follows ECS (Entity Component System) principles with a plugin-based design.

### Plugin System
The game is organized into 5 main plugins, each handling a specific domain:

- **PlayerPlugin** (`src/plugins/player.rs`) - Game state initialization and player setup
- **CombatPlugin** (`src/plugins/combat.rs`) - Combat events and real-time battle systems  
- **StatsPlugin** (`src/plugins/stats.rs`) - Stat upgrades and synchronization systems
- **UIPlugin** (`src/plugins/ui.rs`) - User interface management and updates
- **BalanceCheckPlugin** (`src/plugins/balance_check.rs`) - Headless balance testing mode

### Key Components Architecture

**Combat vs Management Stats Separation:**
- `components/combat_stats.rs` - Temporary battle stats (CurrentHp, CombatAttack, AttackCooldown)
- `components/management_stats.rs` - Persistent progression stats (Experience, Level, BaseAttack)

**Upgradeable Stats System:**
- `components/upgradeable_stats.rs` - Upgrade component definitions and bundles
- `systems/upgrades.rs` - Upgrade logic and stat synchronization systems
- Type-safe marker components: `UpgradeableHp`, `UpgradeableAttack`, `UpgradeableDefense`, `UpgradeableSpeed`
- Individual sync systems for each stat type to maintain ECS single responsibility

### Event-Driven Combat Flow
1. **Real-time Phase**: `attack_cooldown_system` → `player_attack_system`/`enemy_attack_system` → `AttackEvent`
2. **Damage Phase**: `damage_application_system` → `DeathEvent`
3. **Resolution Phase**: `death_detection_system` → `EnemyDeathEvent`/`PlayerDeathEvent` → XP/Respawn

### Dependencies
- **Bevy 0.16.1** - ECS game engine
- **too_big_float** - Custom BigFloat library for handling large numbers in idle game progression
- **clap** - Command line argument parsing for balance testing mode

## ECS Design Principles

This codebase follows strict ECS principles after recent refactoring:

- **Single Responsibility**: Each system handles one specific aspect (e.g., `hp_sync_system` only syncs HP)
- **Type Safety**: Marker components instead of string-based identification
- **Small Systems**: Target 20-30 lines per system, max 50 lines
- **Event-Driven**: Loose coupling between systems via Bevy events

## Testing Structure

Tests are organized by concern:
- `components_tests.rs` - Component creation and behavior
- `systems_tests.rs` - Individual system logic
- `integration_tests.rs` - Multi-system interactions  
- `real_time_combat_tests.rs` - Combat flow and timing

## Code Organization Notes

- Main entry point is minimal (13 lines) - just plugin registration
- All game logic is contained within the plugin system
- UI is completely separated from game logic
- Level components are disambiguated: `management_stats::Level` vs `upgradeable_stats::UpgradeLevel`
- Upgrade system now properly separated: components in `/components/`, systems in `/systems/`

## Development Guidelines

- リファクタリングや新要素の追加、ファイルの追加などがあったときはドキュメントの更新が必要か確認する

## Pre-Development Tasks

- 作業開始前にREADME.mdとARCHITECTURE.mdとGAME_SPEC.mdを読む