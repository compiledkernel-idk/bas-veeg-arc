# Bas Veeg Arc

**Version 0.3.0** - A fast-paced beat 'em up game built with Rust and Macroquad

## About

Bas Veeg Arc is an action-packed fighting game where you battle through waves of enemies in a school setting. Choose from 7 unique characters, each with their own special abilities, and fight your way through challenging boss battles.

## Features

- **7 Playable Characters** with unique abilities and fighting styles
- **Wave-based Combat** with increasing difficulty
- **Boss Battles** featuring Bastiaan and Keizer Bom Taha
- **Upgrade System** - Purchase upgrades between waves
- **Custom ECS Architecture** for performant gameplay
- **Smooth 120 FPS Physics** with fixed timestep

## Characters

- **Bas** - Bas Veeg: AOE splash damage attack
- **Berkay** - Special Kebab: Damage & health boost
- **Gefferinho** - Maar Mevrouw Rage: Speed, damage & health boost
- **Hadi** - Dubai Emirates: Massive speed boost
- **Luca** - Winter Arc: Massive damage boost
- **Nitin** - Barra in je Kont: Sets enemies on fire
- **Yigit Baba** - Sivas Rage: Ultimate ability with damage, speed & health (30s cooldown)

## Controls

### Movement

- **W/Up Arrow** - Move Up
- **S/Down Arrow** - Move Down
- **A/Left Arrow** - Move Left
- **D/Right Arrow** - Move Right

### Combat

- **J** - Light Attack
- **K** - Heavy Attack
- **L** - Special Attack
- **E** - Activate Character Ability

### Shop & Menu

- **B** - Open/Close Shop
- **1-8** - Buy Upgrades
- **Enter/J** - Select
- **Space** - Toggle Details (Character Select)
- **Escape** - Pause / Back

## Building & Running

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build from Source

```bash
cargo build --release
```

### Run the Game

```bash
cargo run --release
```

## Technical Details

- **Engine**: Custom ECS (Entity Component System)
- **Rendering**: Macroquad
- **Physics**: Fixed timestep at 120 FPS
- **Lines of Code**: ~11,000
- **Language**: Rust 2021 Edition

## License

MIT
