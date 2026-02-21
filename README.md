# CapSync

Stop copying your AI skills and commands between tools. Do it once, use them everywhere.

## What is CapSync?

You have a collection of AI coding skills and custom commands. Maybe they are prompts, context files, reusable instructions, or slash commands. You want to use these across multiple AI coding assistants: OpenCode, Claude Code, Cursor, Codex, and dozens of others.

The problem? Each tool stores skills and commands in different locations:

| Tool        | Skills Location             | Commands Location           |
| ----------- | --------------------------- | -------------------------- |
| OpenCode    | `~/.config/opencode/skill/` | `~/.config/opencode/commands/` |
| Claude Code | `~/.claude/skills/`         | `~/.claude/commands/`      |
| Cursor      | `~/.cursor/skills/`        | -                          |
| Codex       | `~/.codex/skills/`         | `~/.codex/commands/`       |
| Kilo Code   | `~/.kilocode/skills/`      | `~/.kilocode/commands/`   |
| Amp         | `~/.agents/skills/`        | -                          |
| Antigravity | `~/.agent/skills/`         | -                          |

And that is just six tools. CapSync supports over 40.

CapSync solves this by creating a single source of truth for your skills and commands. You maintain one skills directory and optionally one commands directory. CapSync creates symlinks from those directories to each tool's expected location. Add a skill or command once, it appears in every tool. Remove it once, it disappears everywhere.

## Important Notice

**What CapSync Is:**

- A synchronization tool that links your existing skills and commands to multiple AI coding assistants
- A symlink manager that keeps your skills and commands directories in sync with many tools

**What CapSync Is Not:**

- A skill or command discovery tool. CapSync does not find or download skills/commands from the internet
- A skill or command installer. You must already have skills/commands in your source directories
- A skill or command creator. CapSync only syncs what you already have

**Prerequisites:**
You need to have skills (and optionally commands) already installed in local directories before using CapSync. CapSync assumes you have:

- A directory containing your skills (e.g., `~/Dev/scripts/skills`)
- Optionally, a directory containing your commands (e.g., `~/Dev/scripts/commands`)
- Skills and commands formatted for your AI tools

If you are looking for a tool to discover and install skills from a registry or repository, that is not what CapSync does. That may be a future feature, but for now, CapSync only syncs skills you already possess.

## Installation

### Download Pre-built Binary

**macOS (Apple Silicon):**

```bash
curl -L -o capsync https://github.com/pixincreate/cap_sync/releases/latest/download/capsync-darwin-aarch64
chmod +x capsync
./capsync init
```

**macOS (Intel):**

```bash
curl -L -o capsync https://github.com/pixincreate/cap_sync/releases/latest/download/capsync-darwin-x86_64
chmod +x capsync
./capsync init
```

**Linux (x86_64):**

```bash
curl -L -o capsync https://github.com/pixincreate/cap_sync/releases/latest/download/capsync-linux-x86_64
chmod +x capsync
./capsync init
```

**Optional: Add to PATH**

```bash
# Add to ~/.zshrc or ~/.bashrc
export PATH="$PATH:/path/to/capsync/dir"
```

### Install via Cargo

```bash
cargo install capsync
```

### Build from Source

```bash
git clone https://github.com/pixincreate/cap_sync.git
cd cap_sync
cargo install --path .
```

## Getting Started

### Initial Setup

Run `capsync init` to create your configuration. The tool will:

1. Ask for your skills directory path (supports `$HOME`, `~`, and other shell variables)
2. Automatically detect if a `commands/` subdirectory exists in your skills directory
3. Prompt to enable commands if found
4. Scan your system for installed AI coding tools
5. Add only the detected tools to your configuration
6. Enable those tools automatically

```bash
$ capsync init
Welcome to CapSync! Let's set up your configuration.

Enter your skills directory: $HOME/Dev/scripts/skills

Detecting installed tools...
Detected and enabled: claude, opencode

Configuration created at /Users/you/.config/capsync/config.toml
```

If a `commands/` subdirectory is found in your skills directory:

```bash
$ capsync init
Welcome to CapSync! Let's set up your configuration.

Enter your skills directory: $HOME/Dev/scripts/skills

Found commands/ subdirectory. Enable commands? [Y/n]: Y

Detecting installed tools...
Detected and enabled: claude, opencode

Configuration created at /Users/you/.config/capsync/config.toml
```

### Daily Workflow

After initial setup, your workflow is simple:

```bash
# Sync your skills and commands to all enabled tools
capsync sync

# Check the status of your symlinks
capsync status

# View your current configuration
capsync config
```

> **Note:** If `commands_source` is not configured, `sync` will only sync skills.

### Adding New Tools

When you install a new AI coding tool, add it to CapSync:

```bash
# Add the tool and sync automatically
capsync add cursor

# Or add without syncing
capsync add codex --no-sync
```

CapSync validates the tool name against its supported tools list. If you try to add an unsupported tool:

```bash
$ capsync add unknown-tool
Error: Tool 'unknown-tool' does not exist or is unsupported in the current version
```

### Removing Tools

Remove a specific tool's symlink:

```bash
capsync remove claude
```

Remove all symlinks:

```bash
capsync remove --all
```

## How It Works

CapSync uses directory symlinks (symbolic links) to connect your skills to each tool.

### The Setup

1. You designate one directory as your skills source (e.g., `~/Dev/scripts/skills/skills`)
2. CapSync creates symlinks from that directory to each tool's expected skills location
3. Each tool sees your skills as if they were native to that tool

### The Benefits

- **Single source of truth**: Edit skills in one place
- **Automatic propagation**: Changes appear in all tools immediately
- **No copying**: Symlinks are lightweight pointers, not copies
- **Easy management**: Add or remove tools without reorganizing files
- **Clean removal**: Remove a tool's access without deleting your skills

### Example

Before CapSync:

```
~/Dev/scripts/skills/
  ├── my-skill/
  │   └── SKILL.md

~/.config/opencode/skill/
  └── my-skill/          (copy)
      └── SKILL.md       (copy)

~/.claude/skills/
  └── my-skill/          (copy)
      └── SKILL.md       (copy)
```

After CapSync:

```
~/Dev/scripts/skills/
  └── my-skill/
      └── SKILL.md       (original)

~/.config/opencode/skill/
  └── my-skill -> ~/Dev/scripts/skills/my-skill  (symlink)

~/.claude/skills/
  └── my-skill -> ~/Dev/scripts/skills/my-skill  (symlink)
```

## Configuration File

CapSync stores its configuration at `~/.config/capsync/config.toml`:

```toml
skills_source = "/Users/you/Dev/scripts/skills"
commands_source = "/Users/you/Dev/scripts/commands"

[destinations.opencode]
enabled = true
skills_path = "/Users/you/.config/opencode/skill"
commands_path = "/Users/you/.config/opencode/commands"

[destinations.claude]
enabled = true
skills_path = "/Users/you/.claude/skills"
commands_path = "/Users/you/.claude/commands"

[destinations.cursor]
enabled = false
skills_path = "/Users/you/.cursor/skills"
```

You can manually edit this file to:

- Change the skills and/or commands source directories
- Enable or disable specific tools
- Adjust destination paths if your tools use non-standard locations

## Supported Tools

CapSync currently supports 40+ AI coding assistants:

**A-C**: AdaL, Amp, Antigravity, Augment, Claude Code, Cline, CodeBuddy, Codex, Command Code, Continue, Cortex, Crush, Cursor

**D-G**: Droid, Gemini CLI, GitHub Copilot, Goose

**I-K**: iFlow CLI, Junie, Kilo Code, Kimi CLI, Kiro CLI, Kode

**M-N**: MCPJam, Mistral Vibe, Moltbot, Mux, Neovate

**O-Q**: OpenCode, OpenHands, OpenClaw, Pi, Pochi, Qoder, Qwen Code

**R-Z**: Replit, Roo Code, Trae, Trae CN, Windsurf, Zencoder

### Commands Support

Not all tools support commands. Currently, the following tools support command syncing:

- **Claude Code**: `~/.claude/commands/`
- **OpenCode**: `~/.config/opencode/commands/`
- **Kilo Code**: `~/.kilocode/commands/`
- **Codex**: `~/.codex/commands/`

Missing a tool? CapSync is designed to easily add new tools. Open an issue or submit a PR.

## Command Reference

### `capsync init`

Initialize configuration. Detects installed tools and creates config file.

### `capsync config`

Display current configuration and enabled tools.

### `capsync detect-tools`

Scan system for installed AI coding tools without modifying config.

### `capsync sync`

Create or update symlinks for all enabled tools.

### `capsync add <tool>`

Add a tool to configuration and sync automatically.

Options:

- `--no-sync`: Add tool without running sync

### `capsync remove <tool>`

Remove symlink for a specific tool.

Options:

- `--all`: Remove all symlinks

### `capsync status`

Check status of source directory and all symlinks.

## Skill Format

While CapSync does not enforce a specific skill format, most AI tools expect:

```
your-skill/
└── SKILL.md
```

The `SKILL.md` typically includes metadata and instructions:

```markdown
---
name: your-skill
description: What this skill does
license: MIT
---

# Your Skill

Detailed instructions for the AI...
```

CapSync syncs the entire directory structure, so your skills can include multiple files, subdirectories, or any format your tools support.

## Command Format

Commands are typically markdown files that define slash commands for AI tools:

```
your-command.md
```

Example command file:

```markdown
---
name: test
description: Run test suite
---

# Test Command

Run the test suite with coverage:

cargo test
```

Commands are named with a tool prefix (e.g., `opencode_test.md`, `claude_test.md`) in your source directory. CapSync strips the prefix when syncing, so `opencode_test.md` becomes `test.md` in the destination.

### Directory Structure

Your commands source can be:

1. **A separate directory** (recommended):
   ```
   ~/my-commands/
   ├── opencode_deploy.md
   ├── opencode_test.md
   ├── claude_deploy.md
   └── claude_test.md
   ```

2. **A `commands/` subdirectory** in your skills directory:
   ```
   ~/my-skills/
   ├── commands/
   │   ├── opencode_deploy.md
   │   └── claude_deploy.md
   └── my-skill/
       └── SKILL.md
   ```

CapSync auto-detects the `commands/` subdirectory during init and prompts to enable command syncing.

## Development

Build the project:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run locally:

```bash
cargo run -- init
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is dedicated to the public domain under the [CC0 1.0 Universal License](LICENSE).
