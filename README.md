# N-Body Problem Simulation

## Build and Run Commands
- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Single test: `cargo test test_name`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## UI Controls
- **Arrow Up/Down**: Increase/decrease simulation speed
- **Arrow Left/Right**: Zoom out/in

## Simulation Components
- **Yellow**: Sun (central body)
- **Blue**: First planet
- **Red**: Second planet
- **Green**: Moon (orbiting first planet)
- **Trails**: Show movement history for each body

## Code Style Guidelines
- **Visibility**: Use `pub(crate)` for internal modules, `pub` only when needed
- **Naming**: Snake case for functions/variables, CamelCase for types/structs
- **Constants**: ALL_CAPS for constants, document physical constants with units
- **Documentation**: Document public interfaces with /// comments
- **Error Handling**: Use Result<T, E> for functions that can fail
- **Vector Operations**: Use Vector struct methods for calculations
- **Parameter Passing**: Pass references (&T) for read-only values
- **Imports**: Group standard library first, then external crates, then local modules
- **Code Organization**: Physics functions in simulation.rs, UI in ui.rs, main entry in main.rs

## Dependencies
- **nannou**: Graphics library for visualization (version 0.18.1)