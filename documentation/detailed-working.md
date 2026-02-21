# How CapSync Actually Works

_A no-nonsense guide to the tool that stops you from going insane copying files around_

## The Problem We All Face

You've got AI coding skills. Prompts, context files, reusable instructions - whatever you call them. And you've got multiple AI tools: OpenCode, Claude Code, Cursor, maybe a few others you're experimenting with.

Here's the annoying part: each one wants your skills in a different folder.

So you copy `my-skill/` to `~/.config/opencode/skill/`. Then you copy it again to `~/.claude/skills/`. Then Cursor. Then Codex. Now you have five copies. You update one. The others are stale. You forget which is which. Chaos.

## What CapSync Actually Does

Instead of copying, CapSync creates **symbolic links** (symlinks). Think of them as shortcuts that look like real folders to your AI tools.

You keep one master copy of your skills. CapSync points all your tools at that one copy. Update it once, every tool sees the change instantly.

No copies. No syncing. No forgetting.

## The Architecture (Don't Worry, It's Simple)

### The Big Picture

```
You
 │
 ▼
CapSync CLI  ──►  Config File  ──►  Symlinks  ──►  AI Tools
```

That's it. You run commands. CapSync reads a config file. It creates symlinks. Your tools are happy.

### The Code Structure

**`cli.rs`** - The Command Parser
This is where your typing turns into action. Uses a library called `clap` (great name) to figure out what you want. Routes to the right handler.

**`config.rs`** - The Config Manager
Reads and writes your settings to `~/.config/capsync/config.toml`. It's just a TOML file - human readable, easy to edit by hand if you want.

**`detect.rs`** - The Finder
Scans your computer for installed AI tools. Just checks if directories exist. Fast, simple, non-invasive.

**`sync.rs`** - The Worker
Actually creates and removes symlinks. Handles the messy platform differences (Unix vs Windows). Reports what worked and what didn't.

**`tools.rs`** - The Registry
A big list of all supported tools and where they keep their stuff. Currently 40+ tools. Easy to add more.

## How We Approach This

### Philosophy: Do One Thing Well

CapSync doesn't:

- Download skills from the internet
- Create skills for you
- Validate your skill format
- Have a GUI
- Run as a daemon
- Do anything fancy

It just syncs. That's it. That's the feature.

### Why Symlinks?

**Pros:**

- Zero disk space (just pointers)
- Instant updates (no re-syncing)
- Easy to undo (just delete the link)
- Tools can't tell the difference

**Cons:**

- Some tools might not follow symlinks (rare)
- Doesn't work across network drives
- Windows handles them differently (we handle this)

**Why not copies?** Because then we'd need to re-copy every time you change something. That's annoying.

**Why not hard links?** Because they don't work across filesystems and are confusing.

### Why Rust?

Because it's fast, safe, and makes distribution easy. One binary, no dependencies. You download it, it works. No "install this runtime first" nonsense.

Plus, if I mess up memory management, the compiler yells at me. That's nice.

## The Features (What You Can Actually Do)

### `capsync init` - First Time Setup

Interactive setup. Asks where your skills live. Detects what tools you have. Creates a minimal config with just those tools.

```bash
$ capsync init
Welcome to CapSync! Let's set up your configuration.

Enter your skills directory: $HOME/my-skills

Detecting installed tools...
Detected and enabled: claude, opencode
```

Notice it only adds tools you actually have. No clutter.

### `capsync sync` - Make It Happen

Creates symlinks for all enabled tools. Shows what worked.

```bash
$ capsync sync
Syncing skills...
================
Source: /Users/you/my-skills

Synced successfully:
  claude
  opencode
```

If something fails, it tells you. But keeps going with the others.

### `capsync add <tool>` - Add New Tools

Got a new AI tool? Add it anytime.

```bash
$ capsync add cursor
Added 'cursor' to configuration
Running sync...
```

Validates the tool name. If you typo it, you'll know:

```bash
$ capsync add cursor-new
Error: Tool 'cursor-new' does not exist or is unsupported
```

Use `--no-sync` if you just want to add without syncing immediately.

### `capsync remove <tool>` - Clean Up

Removes a tool's symlink. Safe - checks it's actually a symlink first.

```bash
$ capsync remove cursor
Removed symlink from /Users/you/.cursor/skills
```

Use `--all` to nuke everything and start fresh.

### `capsync status` - Check What's Up

Shows if your source exists and which tools have working symlinks.

```bash
$ capsync status
Status:
=======
Source: /Users/you/my-skills

Destinations:
  claude: /Users/you/.claude/skills (symlink -> /Users/you/my-skills)
  opencode: /Users/you/.config/opencode/skill (symlink -> /Users/you/my-skills)
```

### `capsync config` - See Your Setup

Displays your current config in a readable format.

## Real World Examples

### Scenario 1: The Initial Setup

You have skills in `~/projects/ai-skills`. You use Claude Code and OpenCode.

```bash
$ capsync init
# Enter: ~/projects/ai-skills
# Detects: claude, opencode
# Creates config

$ capsync sync
# Creates symlinks
# Done
```

Now both tools see your skills. You edit a skill in `~/projects/ai-skills`, both tools see the update immediately.

### Scenario 2: Adding a New Tool

You install Cursor. Want your skills there too.

```bash
$ capsync add cursor
# Validates 'cursor' is supported
# Adds to config
# Runs sync automatically
# Cursor now sees your skills
```

### Scenario 3: The Breakup

You stop using Claude Code. Want to remove it but keep your skills.

```bash
$ capsync remove claude
# Removes the symlink
# Your skills are still in ~/projects/ai-skills
# Claude Code just can't see them anymore
```

## The Config File

Located at `~/.config/capsync/config.toml`. Looks like this:

```toml
source = "/Users/you/my-skills"

[destinations.claude]
enabled = true
path = "/Users/you/.claude/skills"

[destinations.opencode]
enabled = true
path = "/Users/you/.config/opencode/skill"
```

You can edit this by hand. It's just TOML. Add tools, remove them, change paths. CapSync will respect whatever's there.

## Design Decisions (The "Why")

### Why Only Detected Tools in Config?

Early versions added all 40+ tools to the config, disabled. Made the config huge and ugly. Now we only add what you have. Clean, minimal.

### Why Require User to Enter Source Path?

Used to have a default path. But everyone's setup is different. Forcing you to type it means you think about where your skills actually live.

### Why No Emoji or Fancy Symbols?

They break in some terminals. They're distracting. Plain text works everywhere.

### Why No Skill Discovery?

CapSync doesn't download skills from the internet. That's a different problem. Maybe someday, but for now we solve the sync problem really well.

### Why TOML for Config?

It's readable. It has comments. It's standard in the Rust world. You can edit it by hand without breaking things.

## Technical Details (For the Curious)

### Symlink Creation

On Unix (Linux, macOS):

```rust
std::os::unix::fs::symlink(source, dest)
```

On Windows:

```rust
std::os::windows::fs::symlink_dir(source, dest)
```

Same idea, different system calls. We handle both.

### The Sync Process

1. Check source exists (fail fast if not)
2. For each enabled tool:
   - Create parent directories if needed
   - Remove existing file/symlink at destination (clean slate)
   - Create new symlink
3. Report results

Atomic? No. If something fails halfway, some tools are synced, some aren't. We tell you which.

### Error Handling

Uses `anyhow` crate. Means we can attach context to errors:

```rust
fs::remove_file(dest)
    .with_context(|| format!("Failed to remove symlink at {}", dest.display()))?
```

You get helpful error messages, not just "operation failed."

### Tool Detection

Just checks if directories exist:

```rust
Path::new(&home.join(".claude")).exists()
```

Simple. Fast. Works.

## Limitations (Let's Be Honest)

**What It Won't Do:**

- Download skills from a registry
- Validate your skill format
- Sync to remote machines (SSH, etc.)
- Run as a background service
- Merge conflicting skills
- Handle tools that don't follow symlinks (rare, but possible)

**Edge Cases:**

- If a tool changes their skills directory, you need to update the config
- If your source directory moves, you need to update the config
- Windows users might need admin rights for symlinks (depends on Windows version)

## Future Ideas (Maybe)

- Skill templates (scaffold new skills)
- Import/export (share skills as tarballs)
- Validation (check SKILL.md format)
- Remote sync (SSH to another machine)
- GUI version (for non-terminal folks)

But honestly? It works great as-is. Feature creep is the enemy.

## The Bottom Line

CapSync is a tool that does one thing: keeps your AI skills in sync across multiple tools. It does this by creating symlinks from a single source directory to each tool's expected location.

It's fast. It's simple. It doesn't get in your way. It just works.
