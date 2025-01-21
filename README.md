# GoudScape

A RuneScape-inspired MMORPG built with modern technologies.

## Technologies

- **Client**: [Bevy](https://bevyengine.org/) - A data-driven game engine in Rust
- **Server**: [Tokio](https://tokio.rs/) with [Axum](https://github.com/tokio-rs/axum) - Async runtime and web framework
- **Networking**: [renet](https://github.com/lucaspoffo/renet) - Network library for games

## Project Status

🚧 Under Development

## Features (Planned)

- 3D world exploration
- Character customization
- Combat system
- Skills and progression
- Multiplayer interaction

## Project Structure

- `apps/client`: Bevy-based front end client
- `apps/server`: Tokio with Axum-based back end server

## Getting Started

```bash
# Build and run the client
cargo run --bin client

# Build and run the server
cargo run --bin server
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
