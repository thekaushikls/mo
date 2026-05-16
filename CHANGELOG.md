# Changelog

## v0.5.0 - 2026-05-16

### Breaking Changes

- `mo.toml` schema changed: the `[vault]` section is retired. `path` and `version` are now top-level fields. Existing `mo.toml` files need to be updated manually or re-initialized with `mo init`

### New Commands

- `mo log month` - View all entries for the current month (spans across weekly log files)

### Improvements

- Tags are now always sorted alphabetically in log entries
- Projects and people are sorted alphabetically when saved to `mo.toml`
- `mo.toml` now includes a `version` field (set to current `mo` version on init)
- Date-range filtering for log queries (supports multi-week spans)

### Internal

- Renamed `registry.rs` → `config.rs`, struct `Registry` → `Vault`
- Renamed `weekly.rs` → `store.rs`, `append_log` → `append_line`
- New `store::get_files()` and `store::read_lines_by_date_range()` for multi-week queries

---

## v0.4.0 - 2026-05-13

### Improvements

- Refactored codebase into modular architecture:
  - `cli.rs` - CLI definitions (commands, tags, subcommands)
  - `handlers.rs` - all command handlers with shared `timestamp()` helper
  - `registry.rs` - registry CRUD including `Registry::create()` and `Registry::vault_path()`
- `handle_init` now delegates to `Registry::create()` - registry logic lives in registry module
- Reduced boilerplate with `Registry::vault_path()` helper (replaces repeated load + path extraction)
- Consolidated timestamp formatting into single `timestamp()` function
- Cleaned up imports across weekly.rs and handlers.rs
- Package renamed to `mo-cli` with binary name `mo` for crates.io compatibility

---

## v0.3.0 - 2026-05-04

### Breaking Changes

- Removed `note` command - use `--note` tag on `work`, `home`, or `play` instead

### New Commands

- `mo home "<message>"` - Log house chores (supports tags)
- `mo play "<message>"` - Log fun / hobby entries (supports tags)

### New Tags

- `--note` - Mark an entry as a note
- `--research` - Mark an entry as research

### Improvements

- Renamed "flags" to "tags" across CLI and codebase
- Unified command handling - `mood`, `talk`, `work`, `home`, and `play` all route through a single `handle_command()` with optional tags
- `mo login --mood` now uses the shared command handler

---

## v0.2.0 - 2026-04-30

### Breaking Changes

- Renamed `feeling` command to `mood`
- Renamed `feedback` command to `talk`

### Improvements
- Added `--research` flag to work entries
- Added `mo today` as a standalone command (formatted daily view)

---

## v0.1.3 - 2026-04-26

### New Commands

- `mo break` / `mo break "lunch"` - Log a break, with optional reason
- `mo log today` - Show only today's entries

### Fixes

- Break handler wired up correctly

---

## v0.1.2 - 2026-04-21

### New Commands

- `mo note "<text>"` - Scratchpad-style freeform notes
- `mo feedback "<text>"` - Bug reports and feedback entries
- `mo project ls` / `mo project add` - Manage projects directly
- `mo people ls` / `mo people add` - Manage people directly

### Improvements

- Fixed timestamp precision - consistent nanosecond formatting across all entries
- Refactored entity management - `Project` and `Person` logic moved to `entity.rs`
- Removed nested `add` subcommand - projects and people managed via `mo project` and `mo people`
- Added macOS to CI builds

---

## v0.1.1 - 2026-04-15

### New Commands

- `mo --version` - Show current version
- `mo log` - Show last N entries from current week's log (default: 5)
- `mo log file` - Print the current log file path

### Improvements

- Default vault path changed to current directory (`.`) instead of `./sample/vault`

---

## v0.1.0 - 2026-04-06

First release of `mo`, a CLI tool to log work from the terminal.

### Commands

- `mo init --path <dir>` - Initialize a new vault with `mo.toml` config
- `mo login` - Start work day (appends to weekly log)
- `mo login --feeling <mood>` - Start with a feeling entry
- `mo logout` - End work day
- `mo work "<message>"` - Log a work entry with timestamp
- `mo feeling <mood>` - Record how you're feeling
- `mo add project "<name>" --alias <alias>` - Add a project to the registry
- `mo add person "<name>" --alias <alias>` - Add a person to the registry
- `mo completions <shell>` - Generate shell completions (bash, zsh, fish)

### Work Flags

Log entries support flags for categorization:

`--meeting` `--feature` `--unplanned` `--todo` `--blocked` `--done` `--urgent`

### Architecture

- Append-only `.log` files, one per ISO week (`logs/2026-W14.log`)
- Pipe-delimited format, one record per line
- Entity registry in `mo.toml` (people, projects with aliases)
- Config supports local (`./mo.toml`) and global (`~/.config/mo/mo.toml`) paths

### Builds

- Linux (x86_64)
- Windows (x86_64)
