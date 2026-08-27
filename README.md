# Majora's Mask Museum Maze

A small first-person maze game written in Rust with Raylib. It uses classic
Wolfenstein-style raycasting to render a museum filled with artwork inspired by
*The Legend of Zelda: Majora's Mask*.

## Features

- Three procedurally generated maze sizes
- Textured walls, floor, and panoramic sky
- Museum artwork distributed throughout the maze
- Mouse and keyboard camera controls
- Wall collision and a live minimap
- Sword attack animation and sound
- Looping background music
- Victory wall and level-completion screen

## Controls

| Input | Action |
| --- | --- |
| `1`, `2`, `3` | Select difficulty |
| `W` / Up arrow | Move forward |
| `S` / Down arrow | Move backward |
| Mouse / `A`, `D` / Left, Right | Rotate camera |
| `E` | Attack |
| `M` | Toggle 2D and 3D views |
| `Esc` | Exit |

## Running the game

Install a recent Rust toolchain and the native build dependencies required by
Raylib. Then run:

```bash
cargo run --release
```

To run the automated checks:

```bash
cargo test
```

The game opens in a window sized to approximately 80% of the current monitor.
