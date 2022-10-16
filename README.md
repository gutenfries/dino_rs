# Dino_rs

A simple implementation of the Google Chrome Dino game in Rust.

The graphics/game engine is written using [bracket-lib](https://github.com/thebracket/bracket-lib), which is the only dependency.

```toml
[dependencies]
bracket-lib = "0.8.1"
```

Including all whitespace, the project is only 250 LOC.

## Running

```bash
cargo run --release
```

## Controls

-   `P` _or_ `Space` to play & replay the game
-   `Space` _or_ `Up` to jump
-   `P` to pause & unpause (While in game mode)
-   `Esc`, `Q`, _or_ `C` to quit

## Screenshots

### Main Menu

![Main Menu](./docs/Screenshot%202022-10-16%20145922.png)

### Game Play

![Game Play](./docs/Screenshot%202022-10-16%20150052.png)
