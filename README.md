# CapSync

A simple tool to sync your AI coding skills across different tools.

## The Problem

Different AI coding tools store skills in different folders:

| Tool        | Skills Folder               |
| ----------- | --------------------------- |
| OpenCode    | `~/.config/opencode/skill/` |
| ClaudeCode  | `~/.claude/skills/`         |
| Cursor      | `~/.cursor/skills/`         |
| Codex       | `~/.codex/skills/`          |
| Amp         | `~/.agents/skills/`         |
| Antigravity | `~/.agent/skills/`          |

Managing the same skills in all these places is annoying.

## The Solution

Keep your skills in one place and let CapSync sync them everywhere.

## Installation

### Option 1: Download Binary (Quickest)

Download a pre-built binary and use it directly:

**macOS (Apple Silicon):**
```bash
curl -L -o capsync https://github.com/pixincreate/cap_sync/releases/latest/download/capsync-darwin-aarch64
chmod +x capsync
# Move to a directory in your PATH, or use directly:
./capsync init
```

**macOS (Intel):**
```bash
curl -L -o capsync https://github.com/pixincreate/cap_sync/releases/latest/download/capsync-darwin-x86_64
chmod +x capsync
./capsync init
```

**Linux:**
```bash
curl -L -o capsync https://github.com/pixincreate/cap_sync/releases/latest/download/capsync-linux-x86_64
chmod +x capsync
./capsync init
```

**Add to your shell (optional):**
```bash
# Add to ~/.zshrc or ~/.bashrc
export PATH="$PATH:/path/to/capsync/dir"
```

### Option 2: Install via cargo

If you have Rust installed:

```bash
cargo install capsync
```

### Option 3: Build from Source

```bash
git clone https://github.com/pixincreate/cap_sync.git
cd cap_sync
cargo install --path .
```

## Quick Start

```bash
# Set up config
capsync init

# See which tools you have
capsync detect-tools

# Sync your skills
capsync sync

# Check what's synced
capsync status

# Remove from a specific tool
capsync remove claude

# Remove from all tools
capsync remove --all
```

## Commands

### `capsync init`

Creates the config file with sensible defaults at `~/.config/capsync/config.toml`.

### `capsync config`

Shows your current settings.

### `capsync detect-tools`

Checks which AI coding tools are installed on your system.

### `capsync sync`

Creates symlinks from your source directory to all enabled tool destinations.

### `capsync status`

Shows the current sync status:
- Whether source directory exists
- Which destinations have symlinks
- Whether symlinks are valid or broken

### `capsync remove <tool>`

Removes the symlink from a specific tool's directory.

Example: `capsync remove codex`

### `capsync remove --all`

Removes symlinks from all destinations.

## Configuration

Config file is at `~/.config/capsync/config.toml`:

```toml
source = "~/Dev/scripts/skills/skills"

[destinations]
opencode = { enabled = true, path = "~/.config/opencode/skill" }
claude = { enabled = true, path = "~/.claude/skills" }
codex = { enabled = false, path = "~/.codex/skills" }
cursor = { enabled = false, path = "~/.cursor/skills" }
amp = { enabled = false, path = "~/.agents/skills" }
antigravity = { enabled = false, path = "~/.agent/skills" }
```

Change:

- `source`: Where your skills are stored
- `enabled`: Turn tools on/off
- `path`: Where each tool keeps its skills

## How It Works

1. You put skills in one folder
2. CapSync creates a **directory symlink** from that folder to each tool's skills folder
3. Any files you add/remove in the source are automatically available in all destinations
4. No need to re-run sync when you add new skills

## Skill Format

Each skill is a folder with a `SKILL.md` file:

```
my-skill/
└── SKILL.md
```

The `SKILL.md` needs this header:

```markdown
---
name: my-skill
description: What this skill does
license: MIT
---

# My Skill

Details about your skill...
```

## Supported Tools

- OpenCode
- ClaudeCode
- Cursor
- Codex
- Amp
- Antigravity

## Development

```bash
# Build
cargo build

# Test
cargo test

# Run locally
cargo run -- sync
```

## Contributing

1. Fork the repo
2. Make changes
3. Test your changes
4. Send a pull request

## License

CC0 1.0 Universal
