# frd

A Faraday cage for AI coding agents.

frd runs Claude Code (other agents later) inside a bubblewrap sandbox,
so the agent sees only what you allow: no SSH keys, no tokens, no stray `.env` files,
and optionally no network beyond an allowlist.

> Status: early development. The Rust rewrite is in progress.
> The original zsh prototype (two-three files lmao) lives in `zsh-plugin/`.

## Why frd

- **Configs the agent can't touch.** Settings live outside the project (or not, I have not decided yet) and are
  always hidden inside the sandbox, so the agent can't widen its own access.
- **Secrets hidden by name.** A built-in registry (`*.key`, `.env*`, `*secret*`, …)
  masks matching files; add exceptions with `unhide`.
- **Secrets found by content.** Files are scanned with gitleaks before launch.
- **Levels as presets.** Pick a level, override any single setting.

## Levels (planned)

| Level | Workdir         | Visible files       | Network   |
|-------|-----------------|---------------------|-----------|
| 1     | project         | whole project       | full      |
| 2     | project         | only allowed paths  | full      |
| 3     | tmpfs           | only mounted paths  | full      |
| 4     | tmpfs           | only mounted paths  | allowlist |

## Usage (planned)

    frd                    # run Claude in the current project
    frd -l 3               # stricter level
    frd --shell            # shell inside the sandbox
    frd --show-config      # print the merged configuration

## Requirements

Linux with bubblewrap. macOS support is planned.

## License

TBD
