# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [1.0.0] - 2025-01-18

### Added
- Initial release of CapSync
- Configuration system with TOML support
- Tool detection for OpenCode, ClaudeCode, Codex, Cursor, Amp, Antigravity
- Symlink synchronization with cleanup
- CLI commands: init, config, detect-tools, sync, status
- CI/CD pipeline with testing and releases
- Cross-platform binary builds (Linux, macOS x86_64/aarch64)

### Features
- Manual sync mode with `capsync sync`
- Automatic tool detection
- Configurable source skills directory
- Enable/disable tools per configuration
- Broken symlink cleanup
- SHA-256 checksum generation for releases

### Documentation
- Complete README with installation and usage instructions
- Inline help and status reporting