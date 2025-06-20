# Simple Idle RPG

A real-time idle RPG built with Rust and Bevy ECS. Battle enemies, upgrade stats, and progress through an endless adventure.

## Quick Start

### Prerequisites
- Rust 1.70+ with Cargo

### Running the Game
```bash
# Normal mode with UI
cargo run

# Balance check mode (headless, for development)
cargo run -- --balance-check

# Custom initial state for testing
cargo run -- --balance-check --level 5 --hp-level 3 --attack-level 2 --duration 30
```

### Development Commands
```bash
# Build project
cargo build
cargo build --release

# Run tests
cargo test
cargo test integration
cargo test real_time_combat

# Check code
cargo clippy
cargo fmt
```

## Balance Check Mode

For game balance testing without UI overhead:

```bash
# Basic balance check (60 seconds)
cargo run -- --balance-check

# Custom parameters
cargo run -- --balance-check \
  --level 10 \
  --experience 5000 \
  --hp-level 5 \
  --attack-level 4 \
  --defense-level 3 \
  --speed-level 2 \
  --duration 120
```

**Available Options:**
- `--balance-check`: Enable headless mode
- `--level <N>`: Set initial player level (default: 1)
- `--experience <N>`: Set initial experience (default: 0)
- `--hp-level <N>`: Set HP upgrade level (default: 0)
- `--attack-level <N>`: Set Attack upgrade level (default: 0)
- `--defense-level <N>`: Set Defense upgrade level (default: 0)
- `--speed-level <N>`: Set Speed upgrade level (default: 0)
- `--duration <N>`: Run duration in seconds (default: 60)

## Project Structure

```
src/
├── components/     # ECS Components
├── systems/        # ECS Systems
├── events/         # Game Events
├── plugins/        # Bevy Plugins
├── ui/            # User Interface
└── tests/         # Test modules
```

## Documentation

- **[Game Specification](GAME_SPEC.md)** - Gameplay mechanics, balance, and features
- **[Architecture Guide](ARCHITECTURE.md)** - Technical design, ECS patterns, and development guidelines
- **[CLAUDE.md](CLAUDE.md)** - Development instructions for Claude Code

## Key Features

- **Real-time Combat** - Continuous battle system with attack cooldowns
- **Event-driven Auto Retry** - Seamless combat restart with immediate response
- **Stat Upgrades** - Exponential growth progression system
- **ECS Architecture** - Clean, modular design using Bevy ECS
- **Balance Testing** - Headless mode for gameplay tuning
- **Type Safety** - Rust's strong typing prevents common bugs

## Development Workflow

1. **Make Changes** - Edit source code in `src/`
2. **Test Locally** - Run `cargo test` and `cargo run`
3. **Balance Check** - Use `--balance-check` mode for gameplay testing
4. **Build Release** - `cargo build --release` for optimized builds

## Contributing

This project follows strict ECS design principles:
- Single responsibility systems
- Type-safe marker components
- Event-driven architecture
- Small, focused functions (20-30 lines)

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed development guidelines.