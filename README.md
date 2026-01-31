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
# Set up config (only detected tools are added)
capsync init

# See which tools you have
capsync detect-tools

# Sync your skills
capsync sync

# Check what's synced
capsync status

# Add a new tool and sync automatically
capsync add cursor

# Add without syncing
capsync add codex --no-sync

# Remove from a specific tool
capsync remove claude

# Remove from all tools
capsync remove --all
```

## Commands

### `capsync init`

Creates the config file with detected tools at `~/.config/capsync/config.toml`.
Only tools that are currently installed on your system are added.

### `capsync config`

Shows your current settings and enabled tools.

### `capsync detect-tools`

Checks which AI coding tools are installed on your system.

### `capsync sync`

Creates symlinks from your source directory to all enabled tool destinations.

### `capsync add <tool>`

Adds a tool to your configuration and automatically syncs.

```bash
# Add cursor and sync
capsync add cursor

# Add without syncing
capsync add codex --no-sync
```

If the tool name is invalid or unsupported, you'll get an error:
```
Error: Tool 'invalid-tool' does not exist or is unsupported in the current version
```

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
source = "/Users/you/Dev/scripts/skills"

[destinations.opencode]
enabled = true
path = "/Users/you/.config/opencode/skill"

[destinations.claude]
enabled = true
path = "/Users/you/.claude/skills"
```

Only detected tools are added to the config. You can manually add more tools using `capsync add <tool>`.

## How It Works

1. You put skills in one folder
2. CapSync creates a **directory symlink** from that folder to each tool's skills folder
3. Any files you add/remove in the source are automatically available in all destinations
4. No need to re-run sync when you add new skills

## Supported Tools

CapSync supports 40+ AI coding tools:

**A-C:** AdaL, Amp, Antigravity, Claude Code, Cline, CodeBuddy, Codex, Command Code, Continue, Crush, Cursor

**D-G:** Droid, Gemini CLI, GitHub Copilot, Goose

**J-K:** Junie, Kilo Code, Kimi CLI, Kiro CLI, Kode

**M-N:** MCPJam, Moltbot, Mux, Neovate

**O-Q:** OpenCode, OpenHands, OpenClaude IDE, Pi, Pochi, Qoder, Qwen Code

**R-Z:** Replit, Roo Code, Trae, Trae CN, Windsurf, Zencoder

And more...

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
