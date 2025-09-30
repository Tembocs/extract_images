# Contributing to Extract Images

Thank you for your interest in contributing to extract_images! This document provides guidelines for contributing to the project.

## Development Setup

1. **Prerequisites**
   - Rust toolchain (install from [rustup.rs](https://rustup.rs/))
   - Windows 10/11 (for testing the core functionality)

2. **Clone and Build**
   ```bash
   git clone https://github.com/Tembocs/extract_images.git
   cd extract_images
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

## Code Guidelines

### Code Style
- Follow standard Rust formatting (`cargo fmt`)
- All code must pass clippy lints without warnings
- Use meaningful variable and function names
- Add documentation for public functions and modules

### Testing
- Write unit tests for new functionality
- Add integration tests for end-to-end features
- Ensure all tests pass before submitting

### Commit Messages
- Use clear, descriptive commit messages
- Follow the format: `type: description`
- Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`

Examples:
- `feat: add support for custom output directories`
- `fix: handle invalid UTF-8 filenames gracefully`
- `docs: update README with new CLI options`

## Submitting Changes

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Write code following the guidelines above
   - Add tests for new functionality
   - Update documentation as needed

4. **Test thoroughly**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

5. **Submit a pull request**
   - Provide a clear description of changes
   - Reference any related issues
   - Ensure CI passes

## Feature Requests and Bug Reports

### Bug Reports
When reporting bugs, please include:
- Your Windows version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Any error messages

### Feature Requests
For new features, please:
- Describe the use case
- Explain the proposed solution
- Consider backward compatibility
- Be open to discussion about implementation

## Areas for Contribution

We welcome contributions in these areas:

1. **Cross-platform support** - Make the tool work on macOS/Linux
2. **Performance improvements** - Optimize file operations
3. **User interface** - Improve CLI experience
4. **Documentation** - Better docs and examples
5. **Testing** - More comprehensive test coverage
6. **Error handling** - Better error messages and recovery

## Questions?

Feel free to open an issue for questions or join the discussion in existing issues.

Thank you for contributing!