# Rust Project Guidelines for Claude

## Build & Development Commands
- Build: `cargo build`
- Run: `cargo run`
- Release build: `cargo build --release`
- Format code: `cargo fmt`
- Lint: `cargo clippy`
- Test: `cargo test`
- Run specific test: `cargo test test_name`
- Install tool: `cargo install --path .`

## Code Style Guidelines
- **Imports**: Group standard library, external crates, then local modules
- **Error Handling**: Use `Result<T, Box<dyn Error>>` for main functions, prefer `?` operator over `unwrap()`/`expect()`
- **Naming**: Use snake_case for functions/variables, CamelCase for types/structs
- **Documentation**: Document public APIs with rustdoc /// comments
- **String Handling**: Prefer `String` over `&str` for owned data, use `.to_string()` over `.into()`
- **Types**: Use Rust's type system effectively; leverage `derive` for common traits
- **Formatting**: Follow rustfmt conventions with 4-space indentation
- **Constants**: Use SCREAMING_SNAKE_CASE for constants
- **Error Messages**: Be specific and descriptive in error messages