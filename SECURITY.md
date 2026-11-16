# Security policy

frd is a sandbox for AI coding agents. A way to escape the sandbox, read a
hidden path, reach a blocked network address or weaken the configuration from
an untrusted source is a security bug.

## Reporting

Report vulnerabilities privately through GitHub:
**Security → Report a vulnerability** in this repository.

Do not open a public issue for a vulnerability. Include:

- frd version or commit;
- distribution, kernel version, bubblewrap version;
- the configuration files involved;
- steps to reproduce and what you expected.

## Supported versions

frd is in early development. Only the latest commit on `main` is supported.

## Scope

In scope:

- escape from the sandbox process tree;
- access to paths that the effective configuration hides;
- network access beyond the effective `net` mode and allow-list;
- a repository or a tighten-only layer loosening the configuration;
- frd starting when configuration loading failed.

Out of scope (see [threat model](docs/security/threat-model.md)):

- Linux kernel vulnerabilities;
- the agent reading its own credentials, which it needs to work;
- harm done inside paths the user explicitly made writable.
