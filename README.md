<img src="assets/logo.png" height="150px" alt="Mo - tools to get the job done"/>

# `mo` &mdash; CLI tools to get the job done
A fast, append-only work logger. Single binary, no runtime dependencies.

## Setup

```bash
mo init --path <vault-dir>    # creates mo.toml + vault directory
mo --version                  # check version
```

## Day Lifecycle

```bash
mo login                      # start work day
mo login --mood tired         # start with a mood entry
mo logout                     # end work day
mo break                      # log a break
mo break "lunch"              # log a break with reason
mo mood "focused"             # update mood mid-day
```

## Logging Work

```bash
mo work "fixed auth bug"                  # basic entry
mo work "standup" --meeting               # meeting
mo work "deploy v2" --done                # completed task
mo work "urgent prod fix" --urgent --now  # multiple flags
mo note "check CI config later"           # freeform note
mo talk "cli feels slow on startup"       # feedback/bug report
```

### Work Flags

`--blocked` `--done` `--feature` `--meeting` `--now` `--todo` `--unplanned` `--urgent`

## Viewing Logs

```bash
mo log              # last 5 entries (formatted)
mo log 20           # last 20 entries
mo log today        # today's entries only
mo log file         # print current log file path
```

## Entity Management

```bash
mo project ls                              # list projects
mo project add "name" --alias "shortname"  # add project
mo people ls                               # list people
mo people add "name" --alias "nickname"    # add person
```

## Shell Completions

```bash
mo completions bash > ~/.local/share/bash-completion/completions/mo
mo completions zsh > ~/.zfunc/_mo
mo completions fish > ~/.config/fish/completions/mo.fish
```

## Architecture

- Append-only `.log` files, one per ISO week (`logs/2026-W17.log`)
- Pipe-delimited format, one record per line
- Entity registry stored in `mo.toml` (projects, people with aliases)
- Config supports local (`./mo.toml`) and global (`~/.config/mo/mo.toml`) paths

## Builds

Pre-built binaries for Linux, macOS (Apple Silicon), and Windows (x86_64).
