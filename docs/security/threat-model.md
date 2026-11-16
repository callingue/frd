# Threat model

## How an agent touches the machine

A coding agent such as Claude Code is a local process running as the user.
The model runs remotely and only requests tool calls. The local process
executes them: file tools are system calls inside the agent process, shell
tools are child processes.

Everything the agent reads goes into the model context and is sent to the
model provider. Reading a file is the same as sending it.

frd confines the whole agent process tree with kernel mechanisms (namespaces,
seccomp, later Landlock). Restrictions apply to every child process. The agent
keeps the user's UID, but it can only touch what frd makes visible.

## Main goal

A secret must never become readable by the agent. Network rules cannot protect
a secret the agent can read, because the provider API is always reachable.
Hiding is the primary defence; network isolation is the second one.

## Assets

| ID  | Asset |
|-----|-------|
| A-1 | Secrets outside the working paths: SSH and GPG keys, cloud credentials, tokens, browser data in `$HOME` and elsewhere |
| A-2 | Secrets in any path visible to the agent: the project, `ro` and `rw` paths, the working directory, git history, environment variables |
| A-3 | Integrity of the host: files outside the writable paths, shell startup files, other repositories |
| A-4 | Integrity of the user's future actions: code the user will run outside the sandbox |
| A-5 | frd configuration and trust records |

## Adversaries

| ID  | Adversary | Capabilities |
|-----|-----------|--------------|
| E-1 | The agent itself | Runs arbitrary code inside the sandbox and reads or writes everything visible, for any reason: mistakes, misunderstood tasks, shortcuts, prompt injection, misbehaviour |
| E-2 | Malicious repository | Controls project files, `.frd.toml`, git config inside the repo, scripts the user may run |
| E-3 | Model provider and any remote endpoint | Receives everything the agent reads or sends: the provider through the API, other hosts through allowed network paths |

Prompt injection is instructions placed in data the agent reads (README,
comments, issues, web pages, command output). It is one way E-1 happens, not a
separate adversary.

The user and files in `~/.config/frd` are trusted.

## Trust boundaries

1. Sandbox process tree ↔ host. Enforced by the kernel.
2. Visible paths ↔ secrets. Enforced by hiding, scanning and git snapshots.
3. Repository configuration ↔ user configuration. Enforced by the merge rules
   in [configuration.md](../spec/configuration.md).
4. Sandbox network namespace ↔ outside network. Enforced by the frd proxy.

## Threats and mitigations

| ID   | Threat | Assets | Mitigation |
|------|--------|--------|------------|
| T-01 | Agent reads files outside the working paths | A-1, A-3 | Separate mount namespace; only configured paths are mounted; `/` and `$HOME` are never mounted whole |
| T-02 | Agent reads secrets in any visible path, sending them to the provider | A-2 | Secret registry and `hide` applied to every mounted path; content scan of every mounted path; masks mounted last |
| T-03 | Agent reads secrets from git history | A-2 | `git = snapshot` by default: temporary repository with one filtered commit; history scanned when `git = ro`/`rw` |
| T-04 | Agent reads secrets from environment variables | A-2 | `--clearenv`; exact-name passthrough; denylist; value scanning |
| T-05 | Agent writes git hooks or git config executed later on the host | A-4 | `git = rw` masks `hooks/`, `config`, `info/`, `modules/`, `worktrees/`; other modes do not expose a writable `.git` |
| T-06 | Agent writes other auto-run files (`.envrc`, `.nvim.lua`, `.vscode/`, `Makefile`, package scripts) | A-4 | Changed auto-run files reported after the session; optional masking (**open**) |
| T-07 | Agent talks to host daemons over unix sockets (Docker, ssh-agent, D-Bus, X11, Wayland, systemd) | A-1, A-3 | `/run/user/$UID` never mounted whole; `--unshare-net` cuts abstract sockets; seccomp blocks unneeded socket families |
| T-08 | Agent injects keystrokes into the user's terminal | A-3 | seccomp blocks `TIOCSTI` and `TIOCLINUX` |
| T-09 | Agent sends terminal escape sequences (clipboard via OSC 52, title tricks) | A-3 | Output filtering (**open**) |
| T-10 | Agent escapes namespaces by creating new ones or through ptrace | A-3 | `--disable-userns`; seccomp blocks `clone(NEWUSER)`, `unshare`, `ptrace`, `io_uring`, x32 ABI; separate PID namespace |
| T-11 | Agent outlives frd | A-3 | `--die-with-parent` |
| T-12 | Agent sends data to hosts other than the provider | A-1, A-2 | `net = allow`: own netns, proxy with allow-list, `CONNECT` to 443 only, SSRF filter, no DNS inside; `net = none`: loopback only |
| T-13 | Agent destroys or corrupts files in writable paths | A-3 | `work = tmp`, `view = all-ro` or `list`; writable paths only where configured |
| T-14 | Repository loosens the sandbox through `.frd.toml` | all | Repository layer is tighten-only; paths, hosts and env require `frd trust`; trust pinned to file hash |
| T-15 | Repository script calls frd with loosening flags | all | Loosening CLI flags require a TTY; no `FRD_*` environment variables |
| T-16 | Agent or repository edits frd configuration | A-5 | `~/.config/frd` and all loaded configs always hidden; config dir from `getpwuid` |
| T-17 | Symlink or race tricks redirect mounts | A-1, A-3 | `openat2(RESOLVE_NO_SYMLINKS)`, mount by file descriptor, symlinked configs rejected, mount destinations validated, mountinfo check |
| T-18 | Configuration error silently weakens the sandbox | all | Strict parsing; every error aborts the run |

## Out of scope

- **Kernel vulnerabilities.** frd shares the kernel with the host. seccomp
  reduces the attack surface, but frd is not a virtual machine.
- **The agent's own credentials.** The agent needs write access to its config
  directory (for Claude Code: `~/.claude`, `~/.claude.json`) and can read its
  own token.
- **Non-secret content sent to the provider.** Project code the agent reads is
  sent to the provider; that is how the agent works. Secrets are not: keeping
  them unreadable is in scope (T-02, T-03, T-04).

## Residual risks

- A secret that matches neither the registry, nor `hide`, nor the content
  scanner is visible to the agent and reaches the provider. Detection is
  heuristic; users should add their own patterns.
- T-06 and T-09 are only partially mitigated.
- A trusted `.frd.toml` can add paths inside the project root. Trust is the
  user's decision and is dropped on any change to the file.
- Damage inside paths the user made writable cannot be prevented, only limited
  by configuration.

