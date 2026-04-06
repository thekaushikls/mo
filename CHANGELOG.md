# Changelog

## v0.1.0 — 2026-04-06

First release of `mo`, a CLI tool to log work from the terminal.

### Commands

- `mo init --path <dir>` — Initialize a new vault with `mo.toml` config
- `mo login` — Start work day (appends to weekly log)
- `mo login --feeling <mood>` — Start with a mood entry
- `mo logout` — End work day
- `mo work "<message>"` — Log a work entry with timestamp
- `mo feeling <mood>` — Record how you're feeling
- `mo add project "<name>" --alias <alias>` — Add a project to the registry
- `mo add person "<name>" --alias <alias>` — Add a person to the registry
- `mo completions <shell>` — Generate shell completions (bash, zsh, fish)

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
