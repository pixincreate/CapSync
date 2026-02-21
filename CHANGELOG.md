# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- **Commands support**: Sync custom slash commands alongside skills
  - Supports OpenCode, Claude Code, Kilo Code, and Codex
  - Auto-detects `commands/` subdirectory during init
- **New tools support**: Added Augment, Cortex, iFlow CLI, Mistral Vibe
- Fixed tool paths: OpenClaw, Pi, Windsurf, GitHub Copilot, Replit
- Created detailed technical documentation in `documentation/detailed-working.md`
- Documentation covers design, approach, features, and working with conversational tone

### Changed

- Renamed config field `source` → `skills_source` (backward serde alias)
- Renamed config field `path` → `skills_path` (backward compatible via serde alias)
- Added `commands_source` and `commands_path` fields to configuration
- `sync` now syncs both skills AND commands (if commands_source is configured)
- Rewrote README with clearer explanations and better structure
- Added "Important Notice" section explaining what CapSync is and is not
- Simplified license section while maintaining accuracy

## [1.3.5] - 2026-01-31

### Added

- `capsync add <tool>` command to add tools to configuration
- Tool validation - shows error if tool doesn't exist or is unsupported
- `--no-sync` flag for `add` command to skip syncing after adding
- Tests for tools module (6 new tests)

### Changed

- `init` command now only adds detected tools to config (not all 40+)
- Config output now only shows enabled tools (cleaner display)

## [1.3.0] - 2026-01-31

### Added

- Support for 40+ AI tools (expanded from 6)
- Shell variable expansion ($HOME, ~) in paths
- Interactive init - requires user to enter source path (no default)
- Single source of truth for all tool definitions in `tools.rs`

### Removed

- Default source path - user must provide their own
- Emoji symbols (✓✗⚠) from output for cleaner display
- Unnecessary comments throughout codebase

## [1.2.2] - 2026-01-31

### Fixed

- Release workflow binary naming (cap_sync → capsync)
- README download URLs

## [1.2.1] - 2026-01-31

### Fixed

- Interactive init command requires user input for source path

## [1.2.0] - 2026-01-31

### Added

- Interactive `init` command with prompts
- Auto-detection of installed tools during init
- Shell variable expansion support ($HOME, ~)

### Changed

- Upgraded to Rust edition 2024
- Added crates.io metadata (license, repository, keywords, categories)
- Renamed package from `cap_sync` to `capsync`

## [1.1.0] - 2026-01-31

### Added

- `remove` command to remove symlinks from specific tools
- `--all` flag for remove command
- `status` command to check symlink status

### Changed

- Refactored to use directory symlinks instead of recursive file symlinking
- Simplified config structure (source + destinations map)

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
