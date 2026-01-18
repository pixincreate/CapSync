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

### Download Binary (Easy)

1. Go to [Releases](https://github.com/yourusername/cap_sync/releases)
2. Download for your system:
   - `cap_sync-linux-x86_64` for Linux
   - `cap_sync-darwin-x86_64` for Mac Intel
   - `cap_sync-darwin-aarch64` for Mac Apple Silicon
3. Make it executable:
   ```bash
   chmod +x cap_sync-*
   sudo mv cap_sync-* /usr/local/bin/capsync
   ```

### Build From Source

```bash
git clone https://github.com/pixincreate/CapSync.git
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
```

## Commands

### `capsync init`

Creates the config file with sensible defaults.

### `capsync config`

Shows your current settings.

### `capsync detect-tools`

Checks which AI coding tools are installed.

### `capsync sync`

Copies your skills to all enabled tools using symlinks.

### `capsync status`

Shows what's currently synced.

## Configuration

Config file is at `~/.config/capsync/config.toml`:

```toml
[source]
directory = "~/Dev/scripts/skills/skills"

[tools]
opencode = { enabled = true, path = "~/.config/opencode/skill" }
claude = { enabled = true, path = "~/.claude/skills" }
codex = { enabled = false, path = "~/.codex/skills" }
cursor = { enabled = false, path = "~/.cursor/skills" }
```

Change:

- `directory`: Where your skills are stored
- `enabled`: Turn tools on/off
- `path`: Where each tool keeps its skills

## How It Works

1. You put skills in one folder
2. CapSync creates symlinks from that folder to each tool's skills folder
3. Update your skills in one place, they're available everywhere

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
