## 1. Why Rust?
Can product single binary for each OS, and there are no runtime-dependencies. The other popular choice is Python - which requires python to be installed on client machine, virtual environments setup, and dependencies installed.

## Day Lifecycle
- `mo login` — start work day (optional: `--energy tired`)
- `mo logout` — end work day, compiles .log → .md
- `mo break` — toggle break start/stop
- `mo mood <feeling>` — update energy mid-day

## Work
- `mo work "message"` — timestamped entry
  - `--unplanned` — not on your plate
  - `--decision` — a decision was made
  - `--ship` — shipped/completed
  - `--meeting` — meeting note
  - `--blocker` — blocked on something
  - `--todo` — follow-up needed

## Entity Management
- `mo add person/project/team "<name>"` — add to registry
- `mo edit person/project "<name>"` — update status/alias
- `mo remove person/project "<name>"` — remove from registry
- `mo list people/projects/teams` — show all (optional: `--active`)

## Reviewing
- `mo today` — print today's log entries
- `mo yesterday` — print yesterday's entries
- `mo week` — print current week's compiled .md
- `mo status` — logged in? on break? last entry?

## Compilation
- `mo clean` — manually compile .log → .md without logging out

## Reporting (Phase 2)
- `mo summary` — generate weekly summary
- `mo report --month 2026-03` — monthly contributions
- `mo stats --month 2026-03` — hours, energy, unplanned ratio

## Quality of Life (Phase 3)
- `mo init` — scaffold new vault
- `mo search "<query>"` — full-text search across weeks
- `mo undo` — remove last entry
- `mo completions bash/zsh/fish` — shell completions
