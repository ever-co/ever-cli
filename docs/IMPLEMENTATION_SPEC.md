# Ever CLI Router Implementation Spec

## Purpose

This document translates the product-level spec in [SPEC.md](./SPEC.md) into an implementation plan for the `ever-cli` repository.

It is intended for review before coding begins.

The goal is to clarify:

- what problem `ever-cli` is solving
- how it relates to product CLIs such as `ever-works-cli`
- what should be implemented in this repository
- what the MVP must include
- what should be deferred to later phases

## Requirement Understanding

`ever-cli` is no longer meant to be a product-specific TypeScript CLI.

It is meant to become the root Ever ecosystem router:

```bash
ever <product> <command> [args...]
```

Examples:

```bash
ever works init
ever cloc start timer
ever os run agents
```

The router itself must stay small. It should:

- recognize a small set of built-in commands
- resolve product CLIs such as `ever-works`, `ever-cloc`, `ever-os`
- forward all remaining arguments to the resolved product binary
- maintain local installation/discovery metadata
- provide a clear install and diagnostic experience

The router must not contain product business logic.

Product logic remains inside standalone CLIs such as:

- `ever-works-cli`
- `ever-cloc-cli`
- `ever-os-cli`
- `ever-gauzy-cli`

## Relationship To Ever Works

This task is related to Ever Works, but it should not be implemented inside the Ever Works monorepo applications.

The connection is:

- `ever-works-cli` is one of the product CLIs the router must support
- Ever Works has both public and internal CLIs
- this router targets the public CLI surface only

Therefore:

- implementation target: `ever-cli`
- integration target for testing: `ever-works-cli`
- not in scope: moving this router into `apps/cli` or `apps/internal-cli` inside `ever-works`

## Current Project Analysis

Current `ever-cli` state:

- repository: standalone npm package
- implementation: minimal TypeScript stub
- main entrypoint: `src/main.ts`
- behavior today: banner/logo output only

Current package state:

- package name: `ever-cli`
- bin: `ever -> ./dist/main.js`
- old Node/TS stack: `typescript`, `yargs`, `chalk`, `figlet`, `tslint`

This means the current implementation does not match the target architecture in any meaningful way. The router should be treated as a replacement of the current runtime, not an incremental extension of the existing banner tool.

## Proposed Architecture

### Core Model

`ever-cli` becomes a thin router with these responsibilities:

1. Parse the root command line
2. Intercept built-in commands
3. Resolve a product binary
4. Replace the process with the resolved binary when routing
5. Maintain local router metadata under `~/.ever/`

### Resolution Flow

Product resolution should follow the spec exactly:

1. Manifest lookup:
   - `~/.ever/plugins.json`
2. PATH fallback:
   - binary name: `ever-<product>`
3. Not found:
   - clear install guidance

### Local State

Router-owned files:

```text
~/.ever/
├── config.toml
├── plugins.json
└── cache/
```

For MVP, only these are required:

- `~/.ever/plugins.json`
- optional creation of `~/.ever/` if missing

`config.toml` support can start minimal and expand later.

### Built-In Commands

The router reserves these command names:

- `install`
- `uninstall`
- `update`
- `list`
- `doctor`
- `version`
- `help`
- `config`

For MVP, the first commands that should be truly implemented are:

- `help`
- `version`
- `list`
- `doctor`
- `install`

`uninstall`, `update`, and `config` may begin as stubs if needed, but they should still be recognized as reserved router commands.

## Technology Decision

The router should be implemented in Rust.

Reasons:

- the product spec explicitly calls for Rust
- router startup cost should be minimal
- the router does very little but is invoked on every routed command
- the ecosystem pattern described in the spec matches common Rust CLI distribution through npm

### Distribution Model

Primary distribution should remain npm:

```bash
npm install -g ever-cli
```

But the runtime implementation is Rust.

This implies two layers:

1. Rust router binary
2. thin npm wrapper/distribution packaging

For MVP, the Rust router itself is the priority. Cross-platform npm packaging should be treated as a second phase unless explicitly requested in the first delivery.

## Recommended Rust Structure

Suggested first structure:

```text
ever-cli/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── catalog.rs
│   ├── manifest.rs
│   ├── resolver.rs
│   ├── exec.rs
│   ├── fs.rs
│   ├── error.rs
│   └── commands/
│       ├── mod.rs
│       ├── help.rs
│       ├── version.rs
│       ├── list.rs
│       ├── doctor.rs
│       └── install.rs
└── docs/
    ├── SPEC.md
    └── IMPLEMENTATION_SPEC.md
```

### Suggested Dependencies

Keep dependencies conservative:

- `clap` for argument parsing
- `serde` + `serde_json` for manifest serialization
- `directories` or `dirs` for home/config path discovery
- `which` for PATH lookup
- `thiserror` or custom error enums for error handling

Use the Rust standard library for:

- process execution
- file existence checks
- environment access

## MVP Scope

The first implementation should produce a usable router, not the complete long-term ecosystem tooling.

### Must Have

1. Rust entrypoint
2. Router arg parsing
3. Built-in command detection
4. Manifest load/save
5. PATH fallback for `ever-<product>`
6. Exec forwarding to product CLI
7. Static built-in product catalog
8. `ever list`
9. `ever doctor`
10. `ever install <product>` with npm-first installation flow

### Should Have

1. auto-register discovered PATH binaries into manifest
2. stale manifest entry recovery
3. clear human-readable error messages
4. product catalog entry for `works`
5. local validation against `ever-works-cli`

### Not Required For MVP

1. full cargo install flow
2. full GitHub release install flow
3. remote catalog updates
4. Homebrew / winget / curl installer
5. optionalDependencies npm binary packaging matrix
6. advanced config editing
7. alias conflict remediation
8. community plugin registry UX beyond basic future-compatible design

## Install Strategy For MVP

`ever install <product>` should be implemented in the simplest correct way first.

Recommended first behavior:

1. resolve product from static catalog
2. derive npm package name
3. run:

```bash
npm install -g ever-<product>-cli
```

1. resolve installed binary path
2. write manifest entry
3. print success message

This should be npm-first only in MVP.

Cargo and GitHub sources can be added later behind:

- `--from cargo`
- `--from github`

## Product Catalog

The router should ship with a static built-in catalog as described in the main spec.

For MVP, it is acceptable to:

- include the full static list from `SPEC.md`, or
- include a minimal initial list with at least:
  - `works`
  - `cloc`
  - `os`
  - `gauzy`

Recommendation:

- include the full static list now
- it is simple data and avoids revisiting catalog shape immediately

## Manifest Schema

Initial manifest structure should match the product spec:

```json
{
  "version": 1,
  "plugins": {
    "works": {
      "binary": "/usr/local/bin/ever-works",
      "package": "ever-works-cli",
      "source": "npm",
      "version": "0.5.0",
      "installed_at": "2026-04-01T12:00:00Z"
    }
  }
}
```

For MVP:

- missing optional fields may be tolerated internally
- persisted shape should remain compatible with the spec

## Execution Semantics

On Unix/macOS:

- use true process replacement where possible

On Windows:

- use spawn/wait/exit-code propagation behavior appropriate to platform constraints

The goal is:

- correct stdout/stderr passthrough
- correct exit codes
- correct signal behavior

Even if Windows behavior differs internally, the user-facing behavior should remain consistent.

## Phased Delivery Plan

### Phase 0 — Planning

- approve this implementation spec
- confirm MVP scope
- confirm whether npm-first install is sufficient for first PR

### Phase 1 — Router Core

- initialize Rust project
- replace current TS runtime with Rust entrypoint
- implement built-in command parsing
- implement manifest support
- implement PATH resolution
- implement routing exec flow

### Phase 2 — Core Built-Ins

- implement `help`
- implement `version`
- implement `list`
- implement `doctor`
- implement npm-first `install`

### Phase 3 — Ever Works Validation

- test with installed `ever-works-cli`
- validate:
  - `ever works --help`
  - `ever works init`
  - manifest discovery
  - PATH fallback

### Phase 4 — Packaging

- add npm wrapper strategy for native binary distribution
- prepare platform package approach
- define release workflow

This should be a separate step unless it is explicitly required in the first implementation PR.

## Risks And Design Notes

### 1. Install Scope Can Expand Quickly

The spec describes npm, cargo, and GitHub installs. Implementing all three at once increases complexity and testing burden.

Recommendation:

- ship npm-first install in MVP
- keep source abstraction in the code design so cargo/github can be added cleanly later

### 2. Short Alias Support Is Mostly A Sub-CLI Concern

The root router handles:

- `ever works ...`

Short aliases such as:

- `works ...`
- `cloc ...`

are primarily the responsibility of each sub-CLI package and its installation method.

The router should be aware of alias conflicts for `doctor`, but it should not block MVP routing work.

### 3. Packaging Should Not Block Router Architecture

The Rust router can and should be built first even if cross-platform npm publishing is not ready yet.

This separates:

- runtime correctness
- distribution engineering

### 4. Existing TypeScript Code Should Not Drive New Design

The current `src/main.ts` is too small and too far from the target architecture to be treated as a meaningful foundation.

The implementation should optimize for the target router design, not for preserving the existing stub structure.

## Validation Plan

The first implementation should be considered acceptable when the following work:

### Local Router Behavior

```bash
ever --help
ever version
ever list
ever doctor
```

### Product Routing

Assuming `ever-works-cli` is installed:

```bash
ever works --help
ever works init
```

### Install Flow

```bash
ever install works
```

Expected result:

- installs `ever-works-cli`
- resolves `ever-works`
- writes manifest entry
- `ever works --help` works afterward

### PATH Recovery

If a manifest path is stale but `ever-works` exists elsewhere on PATH:

- router should discover it
- update manifest
- continue successfully

## Out Of Scope For This Review Spec

This document does not define:

- exact Rust crate versions
- exact CI YAML for release matrix
- exact npm package publishing automation
- final Windows packaging details

Those belong to implementation PRs once the architecture is approved.

## Recommendation

First PR target:

- Rust router core
- manifest + PATH resolution
- npm-first install
- basic built-ins
- validation against `ever-works-cli`

This gives a usable vertical slice quickly while preserving the architecture required by the product spec.
