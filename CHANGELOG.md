# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [2.2.1] - 2026-04-12

- Darwin release builds now vendor `libgit2` and OpenSSL through `git2`, avoiding macOS OpenSSL discovery failures during tagged release builds

## [2.2.0] - 2026-04-12

### Added

- `capsync clone <repo>` to materialize a whole remote skills repository into `skills_source`
  - Supports `owner/repo`, `owner/repo.git`, HTTPS, and SSH references
  - Resolves the remote default branch automatically and syncs after clone by default
- `capsync install <reference>` to install a single skill into `skills_source/<slug>`
  - Supports explicit HTTPS `skills.sh` URLs, GitHub tree URLs, and `owner/repo/...` references
  - Resolves one concrete skill from a temporary checkout and syncs after install by default
- New clone/install coverage and expanded technical documentation for workflow, configuration, and safety behavior

### Changed

- Expanded README and technical documentation to explain the clone vs install mental model and current workflow limitations
- Renamed `documentation/detailed-working.md` to `documentation/how-it-works.md`
- Updated dependencies:
  - `toml` `1.0.3+spec-1.1.0` → `1.1.2+spec-1.1.0`
  - `tempfile` `3.25.0` → `3.27.0`
  - `clap` `4.5.60` → `4.6.0`
  - `shellexpand` `3.1.1` → `3.1.2`

### Fixed

- Supported-agent registry path mappings for Amp, Antigravity, GitHub Copilot, Kimi CLI, OpenClaw, Pi, Replit, Trae CN, and Windsurf
- Clone safety and correctness around same-repo detection, default-branch resolution, repo-without-origin handling, and branch-mismatch confirmation before re-cloning
- Install safety around HTTP `skills.sh` rejection, GitHub tree URL validation, URL-encoded branch handling, path traversal guards, symlink rejection, and staged replacement of existing installs

## [2.0.1] - 2026-02-22

### Added

- Allow re-initialization with confirmation prompt

### Fixed

- Check gh auth status before creating PR
- Handle empty commands_source string gracefully

## [2.0.0] - 2026-02-22

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
