# Wassemble Components

Wassemble Components is a monorepo providing reusable WebAssembly (Wasm) components for various services. These components expose APIs (such as Discord, GitHub, OpenAI, and more) or other useful functionality as Wasm modules, making them easy to integrate into any Wasm-compatible runtime or application.

## Available Components

- [discord](crates/discord): Bindings for the Discord API
- [github](crates/github): Bindings for the GitHub API
- [hello-world](crates/hello-world): Minimal example component
- [openai](crates/openai): Bindings for the OpenAI API

## Usage

All components are published as Wasm components to the [GitHub Container Registry (ghcr.io)](https://ghcr.io/). You can consume them in your own projects using [cargo-component](https://github.com/bytecodealliance/cargo-component) and compatible tooling.

To build a component locally:

```
cargo component build -p <component>
```

To publish a component to the registry:

```
cargo component publish -p <component>
```

The registry is configured as `ghcr.io` and publishing is automated via GitHub Actions on push to `main`.

## Contributing

Contributions are welcome! To add a new component or improve an existing one:

- Fork and clone the repository
- Install the required tools:
  - `cargo-component`
  - `cargo-machete`
  - `cargo-sort-derives`
  - `cargo-watch`
  - `wkg`
- Use the provided `justfile` for common tasks (build, check, publish, etc.)
- Follow the style and structure of existing components
- Ensure your changes pass CI (formatting, linting, and tests)
- Open a pull request

All contributions are subject to the [Apache 2.0 License](LICENSE).

## CI/CD

- **CI**: On every push and pull request, the following checks are run:
  - Formatting (`cargo fmt`)
  - Linting (`cargo clippy`, `cargo machete`, `cargo sort-derives`)
  - Build and tests (`cargo check`, `cargo test`)
- **CD**: On push to `main`, changed components are automatically published to the GitHub Container Registry as Wasm components.

## License

This project is licensed under the [Apache License 2.0](LICENSE).
