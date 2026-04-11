# CapSync Agent Documentation

This document provides comprehensive documentation for AI agents interacting with CapSync.

> **For deeper understanding** of architecture, design philosophy, and implementation details, see `documentation/how-it-works.md`.

## Table of Contents

1. [What is CapSync?](#what-is-capsync)
2. [Core Concepts](#core-concepts)
3. [Commands Reference](#commands-reference)
4. [Configuration](#configuration)
5. [Supported Tools](#supported-tools)
6. [Skill and Command Formats](#skill-and-command-formats)
7. [How It Works](#how-it-works)
8. [Common Use Cases](#common-use-cases)

---

## What is CapSync?

CapSync is a CLI tool that synchronizes AI coding skills and commands across multiple AI coding assistants. Instead of maintaining separate skill directories for each tool, you maintain one source directory, and CapSync creates symlinks to all other locations.

**Problem**: Each AI tool stores skills in different locations:
- OpenCode: `~/.config/opencode/skill/`
- Claude Code: `~/.claude/skills/`
- Cursor: `~/.cursor/skills/`
- Codex: `~/.codex/skills/`

**Solution**: One source of truth → symlinks everywhere.

---

## Core Concepts

### Source Directory
The single directory containing your skills (and optionally commands). Set during `capsync init` as `skills_source`.

### Destination
A target tool's skills/commands location. Each destination has:
- `enabled`: Whether to sync to this tool
- `skills_path`: Where to create symlinks for skills
- `commands_path`: Where to create symlinks for commands (optional)

### Symlink
A symbolic link. CapSync doesn't copy files—it creates symlinks from destination directories pointing to your source directory.

### Clone
The `clone` command fetches a remote Git repository into your skills source, enabling CapSync to sync remote skills to local tools.

---

## Commands Reference

### `capsync init`

Interactive setup. Creates config at `~/.config/capsync/config.toml`.

```bash
capsync init
```

Prompts for:
1. Skills source directory path (supports `$HOME`, `~`, shell variables)
2. Commands source (optional - detected if `commands/` subdirectory exists)
3. Auto-detects installed AI tools

### `capsync config`

Display current configuration.

```bash
capsync config
```

Output includes:
- Config file location
- Skills source path
- Commands source path (if configured)
- Enabled tools with their paths

### `capsync detect-tools`

Scan system for installed AI coding tools without modifying config.

```bash
capsync detect-tools
```

Detects tools by checking for their config directories.

### `capsync sync`

Create or update symlinks for all enabled tools.

```bash
capsync sync
```

Creates symlinks:
- `skills_source/*` → each tool's `skills_path`
- `commands_source/*` (if configured) → each tool's `commands_path`

### `capsync add <tool>`

Add a tool to configuration and optionally sync.

```bash
capsync add <tool>           # Add and sync
capsync add <tool> --no-sync # Add without sync
```

Validates tool name against supported list.

### `capsync remove <tool>`

Remove symlinks for a tool.

```bash
capsync remove <tool>  # Remove specific tool
capsync remove --all  # Remove all symlinks
```

### `capsync status`

Check status of source directory and all symlinks.

```bash
capsync status
```

Shows:
- Source directory existence
- Symlink status (valid, broken, missing)
- For each destination: skills and commands symlink status

### `capsync clone <repo>`

Clone a remote Git repository into your skills source.

```bash
capsync clone <repo>                 # Clone (auto-detect branch)
capsync clone <repo> --branch <name> # Clone specific branch
capsync clone <repo> --no-sync       # Clone without syncing
```

**Arguments:**
- `<repo>`: Repository in `owner/repo` format or full URL

**Behavior:**
- If `skills_source` doesn't exist: Clone directly
- If `skills_source` exists (same repo): Prompt to update or override
- If `skills_source` exists (different repo): Prompt to override
- If `skills_source` exists but is not a git repository: Prompt before overriding
- If `skills_source` is a git repository without an `origin` remote: Prompt before overriding
- If override would discard local changes: Warn and offer to back up first
- If update is selected while the current branch differs from `--branch`: Re-clone the requested branch instead of updating in place

**Branch Detection:**
- Auto-detects the remote's default branch from remote HEAD
- Falls back to fetched branch names only if the remote does not report a default branch

---

## Configuration

### Config File Location
`~/.config/capsync/config.toml`

### Config Structure

```toml
skills_source = "/path/to/skills"
commands_source = "/path/to/commands"  # optional

[destinations.opencode]
enabled = true
skills_path = "/home/user/.config/opencode/skill"
commands_path = "/home/user/.config/opencode/commands"

[destinations.claude]
enabled = true
skills_path = "/home/user/.claude/skills"
commands_path = "/home/user/.claude/commands"

[destinations.cursor]
enabled = false
skills_path = "/home/user/.cursor/skills"
```

### Manual Config Editing

You can manually edit the config file to:
- Change source directories
- Enable/disable specific tools
- Adjust destination paths for non-standard installations

---

## Supported Tools

### Skills Support

CapSync supports 40+ AI coding assistants:

| Category | Tools |
|----------|-------|
| A-C | AdaL, Amp, Antigravity, Augment, Claude Code, Cline, CodeBuddy, Codex, Command Code, Continue, Cortex, Crush, Cursor |
| D-G | Droid, Gemini CLI, GitHub Copilot, Goose |
| I-K | iFlow CLI, Junie, Kilo Code, Kimi CLI, Kiro CLI, Kode |
| M-N | MCPJam, Mistral Vibe, Moltbot, Mux, Neovate |
| O-Q | OpenCode, OpenHands, OpenClaw, Pi, Pochi, Qoder, Qwen Code |
| R-Z | Replit, Roo Code, Trae, Trae CN, Windsurf, Zencoder |

### Commands Support

Only some tools support commands:

- Claude Code: `~/.claude/commands/`
- OpenCode: `~/.config/opencode/commands/`
- Kilo Code: `~/.kilocode/commands/`
- Codex: `~/.codex/commands/`

---

## Skill and Command Formats

### Skill Format

Most AI tools expect:

```
your-skill/
└── SKILL.md
```

`SKILL.md` structure:

```markdown
---
name: your-skill
description: What this skill does
license: MIT
---

# Your Skill

Your skill instructions here...
```

CapSync syncs the entire directory, so you can include multiple files and subdirectories.

### Command Format

Commands are markdown files defining slash commands:

```markdown
---
name: test
description: Run test suite
---

# Test Command

Run the test suite:

cargo test
```

Commands can be in a separate directory or a `commands/` subdirectory in skills.

---

## How It Works

### Symlink Mechanism

CapSync uses symbolic links, not file copying:

```
Before (without CapSync):
~/skills/ → copied to each tool's location (duplicated!)

After (with CapSync):
~/skills/ → ~/.config/opencode/skill/ (symlink)
        → ~/.claude/skills/ (symlink)
        → ~/.cursor/skills/ (symlink)
```

### Benefits

- **Single source of truth**: Edit in one place
- **Instant propagation**: Changes appear everywhere immediately
- **No duplication**: Symlinks are lightweight
- **Easy removal**: Remove tool access without deleting skills

### Clone Workflow

1. Parse repo URL (`owner/repo` or full URL)
2. Determine default branch (fetch remote refs)
3. Handle existing source:
   - Same repo → offer update (git pull) or override
   - Different repo → offer override
   - Non-git directory or no `origin` remote → require explicit confirmation before override
   - Local changes during override → warn and offer backup
4. Clone to `skills_source`
5. Optionally sync to all enabled tools

---

## Common Use Cases

### Setting Up for the First Time

```bash
# 1. Initialize config
capsync init
# Enter: ~/my-skills

# 2. Sync to all detected tools
capsync sync

# 3. Check status
capsync status
```

### Adding a New Tool

```bash
# Add tool and sync
capsync add cursor

# Or add without immediate sync
capsync add codex --no-sync
```

### Cloning Remote Skills

```bash
# Clone a skills repository
capsync clone username/skills-repo

# Clone specific branch
capsync clone username/skills-repo --branch develop

# Clone without syncing (for inspection first)
capsync clone username/skills-repo --no-sync
```

### Updating Cloned Skills

```bash
# If already cloned a repo, this prompts:
# "Update (git pull) or Override (download new)?"
capsync clone username/skills-repo
```

### Removing a Tool

```bash
# Remove specific tool
capsync remove claude

# Remove all tools
capsync remove --all
```

---

## File Structure

```
capsync/
├── src/
│   ├── main.rs      # Entry point
│   ├── lib.rs       # Module definitions
│   ├── cli.rs       # CLI commands
│   ├── config.rs    # Config loading/saving
│   ├── clone.rs    # Git clone functionality
│   ├── detect.rs   # Tool detection
│   ├── sync.rs     # Symlink management
│   └── tools.rs    # Supported tools list
├── Cargo.toml
├── README.md
└── documentation/
    └── agents.md   # This file
```

---

## Error Handling

| Error | Cause | Solution |
|-------|-------|----------|
| "Tool not found" | Invalid tool name | Use valid tool from `capsync detect-tools` |
| "Config not found" | No config file | Run `capsync init` first |
| "Skills source not found" | Source path doesn't exist | Check config or run `capsync clone` |
| "Repository not found" | Invalid repo URL | Check URL format: `owner/repo` |
| "Failed to clone" | Network/auth error | Check internet connection |

---

## Exit Codes

- `0`: Success
- `1`: Error (invalid input, file system errors, etc.)

---

## Related Documentation

- `README.md` - User-facing getting started guide
- `documentation/how-it-works.md` - Architecture, design decisions, and philosophy
- `SPEC-clone.md` - Clone feature specification
- `CHANGELOG.md` - Version history
