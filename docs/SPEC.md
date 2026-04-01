# Ever CLI — Multiplexer / Router Architecture

## Overview

The `ever` CLI acts as a **thin router (multiplexer)** that dispatches commands to product-specific sub-CLIs. Each product in the Ever ecosystem ships its own standalone CLI binary, and the `ever` root CLI discovers and invokes them transparently.

```
ever <product> <command> [subcommand] [args] [flags]
```

**Examples:**

```bash
ever os run agents
ever os plugin install yo
ever teams notify kostya
ever teams add member vanya
ever cloc start timer
ever gauzy export invoices --format=csv
ever dev setup workspace
```

The user types one unified command. Under the hood, `ever` resolves the product name to the correct binary and forwards the remaining arguments.

---

## Design Principles

1. **The root CLI is tiny.** It handles routing, plugin management, and global flags only. No product logic lives here.
2. **Sub-CLIs are fully independent.** Each can be a separate npm package, Rust crate, Go binary — any language, any release cadence, any maintainer.
3. **Sub-CLIs work standalone.** `ever-os run agents` works identically whether invoked directly or via `ever os run agents`. The sub-CLI receives the same argv either way.
4. **Zero coupling between products.** Adding a new product CLI never requires changes to the router.
5. **Graceful degradation.** If a sub-CLI is not installed, the router provides a clear install command instead of a cryptic error.

---

## Architecture: Hybrid Discovery (Approach 3)

The router uses a two-phase lookup: **manifest first, PATH fallback**.

```
ever teams notify kostya
  │
  ├─ Phase 1: Check ~/.ever/plugins.json
  │    └─ Found "teams" → /usr/local/bin/ever-teams
  │
  ├─ Phase 2 (if not in manifest): Search PATH for "ever-teams"
  │    └─ Found → exec ever-teams notify kostya
  │
  ├─ Phase 3 (if not found anywhere):
  │    └─ Error: "Unknown product 'teams'. Run: ever install teams"
  │
  └─ Exec: ever-teams notify kostya
       └─ Sub-CLI receives argv: ["notify", "kostya"]
```

### Phase 1 — Manifest Lookup

The router reads `~/.ever/plugins.json` (created on first run or first install). This file maps product names to binary paths and metadata.

```json
{
  "version": 1,
  "plugins": {
    "os": {
      "binary": "/usr/local/bin/ever-os",
      "package": "ever-os-cli",
      "source": "npm",
      "version": "0.3.1",
      "installed_at": "2026-04-01T12:00:00Z"
    },
    "teams": {
      "binary": "/home/user/.cargo/bin/ever-teams",
      "package": "ever-teams-cli",
      "source": "cargo",
      "version": "1.0.0",
      "installed_at": "2026-04-01T12:05:00Z"
    }
  }
}
```

If the product is found in the manifest and the binary exists at the recorded path, the router execs it immediately. If the binary path is stale (file no longer exists), the router falls through to Phase 2 and, if found, updates the manifest.

### Phase 2 — PATH Fallback

If the product is not in the manifest, the router searches the system PATH for a binary named `ever-<product>`. This enables sub-CLIs installed manually, via system package managers, or by other tools to be discovered automatically.

On successful PATH discovery, the router **auto-registers** the binary into the manifest for faster subsequent lookups.

### Phase 3 — Not Found

If neither phase finds a binary, the router prints:

```
Error: Product 'teams' is not installed.

Install it with:
  ever install teams

Available products:
  ever list
```

---

## Execution Model

Once the router resolves a binary path, it performs an **exec** (process replacement), not a subprocess spawn. This means:

- The sub-CLI fully replaces the router process (no parent process hanging around).
- stdin, stdout, stderr pass through natively.
- Exit codes propagate correctly.
- Signal handling works as expected (Ctrl+C, SIGTERM, etc.).

**Argument forwarding:**

```
# Mode A: Via router
User types:           ever os run agents --verbose
Router resolves:      /usr/local/bin/ever-os
Exec call:            exec /usr/local/bin/ever-os run agents --verbose
Sub-CLI receives:     argv = ["ever-os", "run", "agents", "--verbose"]

# Mode B: Direct prefixed
User types:           ever-os run agents --verbose
Shell resolves:       /usr/local/bin/ever-os
Sub-CLI receives:     argv = ["ever-os", "run", "agents", "--verbose"]

# Mode C: Short alias
User types:           os run agents --verbose
Shell resolves:       /usr/local/bin/os (symlink → ever-os)
Sub-CLI receives:     argv = ["os", "run", "agents", "--verbose"]
```

In all three modes, the sub-CLI receives the same meaningful arguments: `["run", "agents", "--verbose"]`. Only `argv[0]` differs (the binary name), which the sub-CLI should ignore.

---

## Built-in Commands

The root `ever` CLI reserves the following built-in commands (these are NOT forwarded to sub-CLIs):

| Command | Description |
|---|---|
| `ever install <product>` | Install a product sub-CLI |
| `ever uninstall <product>` | Remove a product sub-CLI and its manifest entry |
| `ever update [product]` | Update one or all installed sub-CLIs |
| `ever list` | List all known products and their install status |
| `ever doctor` | Verify all manifest entries point to valid binaries |
| `ever version` | Show router version |
| `ever help` | Show global help and list of available products |
| `ever config` | Manage global configuration (~/.ever/config.toml) |

**Collision rule:** Built-in commands take priority. If a sub-CLI is ever named `ever-install`, the built-in `ever install` wins. This set of reserved names should remain small and stable.

---

## Sub-CLI Registry

All product CLIs in the Ever ecosystem. Each sub-CLI exposes **three command names**: the short alias, the prefixed binary name, and the router invocation.

| Product | Short Alias | Prefixed Binary | Router Invocation | Package (npm) | Package (crate) | Status |
|---|---|---|---|---|---|---|
| **ever** (router) | — | `ever` | `ever <product> <cmd>` | `ever-cli` | `ever-cli` | ✅ Exists ([npm](https://www.npmjs.com/package/ever-cli)) |
| **gauzy** | `gauzy` | `ever-gauzy` | `ever gauzy <cmd>` | `ever-gauzy-cli` | `ever-gauzy-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-gauzy-cli)) |
| **os** | `os` | `ever-os` | `ever os <cmd>` | `ever-os-cli` | `ever-os-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-os-cli)) |
| **teams** | `teams` | `ever-teams` | `ever teams <cmd>` | `ever-teams-cli` | `ever-teams-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-teams-cli)) |
| **works** | `works` | `ever-works` | `ever works <cmd>` | `ever-works-cli` | `ever-works-cli` | ✅ Exists ([npm](https://www.npmjs.com/package/ever-works-cli)) |
| **rec** | `rec` | `ever-rec` | `ever rec <cmd>` | `ever-rec-cli` | `ever-rec-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-rec-cli)) |
| **hust** | `hust` | `ever-hust` | `ever hust <cmd>` | `ever-hust-cli` | `ever-hust-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-hust-cli)) |
| **jobs** | `jobs` | `ever-jobs` | `ever jobs <cmd>` | `ever-jobs-cli` | `ever-jobs-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-jobs-cli)) |
| **dev** | `dev` | `ever-dev` | `ever dev <cmd>` | `ever-dev-cli` | `ever-dev-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-dev-cli)) |
| **cloc** | `cloc` | `ever-cloc` | `ever cloc <cmd>` | `ever-cloc-cli` | `ever-cloc-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-cloc-cli)) |
| **demand** | `demand` | `ever-demand` | `ever demand <cmd>` | `ever-demand-cli` | `ever-demand-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-demand-cli)) |
| **traduora** | `traduora` | `ever-traduora` | `ever traduora <cmd>` | `ever-traduora-cli` | `ever-traduora-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-traduora-cli)) |
| **tech** | `tech` | `ever-tech` | `ever tech <cmd>` | `ever-tech-cli` | `ever-tech-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-tech-cli)) |
| **saas** | `saas` | `ever-saas` | `ever saas <cmd>` | `ever-saas-cli` | `ever-saas-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-saas-cli)) |
| **docs** | `docs` | `ever-docs` | `ever docs <cmd>` | `ever-docs-cli` | `ever-docs-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-docs-cli)) |
| **digital** | `digital` | `ever-digital` | `ever digital <cmd>` | `ever-digital-cli` | `ever-digital-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-digital-cli)) |
| **shop** | `shop` | `ever-shop` | `ever shop <cmd>` | `ever-shop-cli` | `ever-shop-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-shop-cli)) |
| **iq** | `iq` | `ever-iq` | `ever iq <cmd>` | `ever-iq-cli` | `ever-iq-cli` | ✅ Reserved ([npm](https://www.npmjs.com/package/ever-iq-cli)) |

> **Note:** The short alias (e.g. `cloc`, `teams`) and the prefixed binary (e.g. `ever-cloc`, `ever-teams`) both point to the same underlying executable. See **"Short Alias Commands"** below for how this is implemented.

---

## Directory Structure

```
~/.ever/
├── config.toml            # Global configuration (API endpoints, auth tokens, defaults)
├── plugins.json           # Manifest: product → binary path mapping
└── cache/                 # Optional: cached package metadata, update checks
```

### config.toml

```toml
[global]
telemetry = false
auto_update_check = true
default_org = "ever-co"

[auth]
# Shared auth token usable by all sub-CLIs
api_token = "ey..."
api_endpoint = "https://api.ever.co"

[registry]
# Where to look for sub-CLI packages
npm_prefix = "ever"
```

Sub-CLIs can read `~/.ever/config.toml` for shared configuration (auth tokens, org defaults) so the user authenticates once via `ever config set auth.api_token <token>` and all products inherit it.

---

## Router Implementation

### Pseudocode

```
fn main():
    args = parse_cli_args()

    if args.command in BUILT_IN_COMMANDS:
        handle_builtin(args)
        return

    product = args.command           # e.g. "os"
    rest    = args.remaining         # e.g. ["run", "agents", "--verbose"]

    # Phase 1: Manifest lookup
    manifest = load_manifest("~/.ever/plugins.json")
    if product in manifest.plugins:
        binary = manifest.plugins[product].binary
        if file_exists(binary):
            exec(binary, rest)       # Process replacement, never returns
        else:
            warn("Binary not found at {binary}, searching PATH...")

    # Phase 2: PATH fallback
    binary_name = "ever-" + product  # e.g. "ever-os"
    binary = which(binary_name)
    if binary:
        # Auto-register for future lookups
        manifest.plugins[product] = { binary: binary, source: "path" }
        save_manifest(manifest)
        exec(binary, rest)

    # Phase 3: Not found
    error("Product '{product}' is not installed.")
    hint("Run: ever install {product}")
    hint("Run: ever list  (to see available products)")
    exit(1)
```

### Key Implementation Details

**Language choice for the router:** The router is built in Rust for instant startup (~5ms vs ~200ms for Node.js). This matters because every `ever <x>` command pays the router's startup cost before reaching the sub-CLI. Distribution to end users is handled via npm using platform-specific optional dependencies — users run `npm install -g ever-cli` and get the native Rust binary transparently. See **"Distribution: Rust Binary via npm"** below for full details.

**exec semantics by platform:**
- Unix/macOS: Use `execvp()` (Rust: `std::os::unix::process::CommandExt::exec()`) for true process replacement.
- Windows: Use `CreateProcess` + wait + propagate exit code (Windows does not support exec-style process replacement).

**Binary naming convention:** All sub-CLI binaries MUST be named `ever-<product>` (lowercase, hyphenated). This is the contract between the router and sub-CLIs.

---

## Distribution: Rust Binary via npm

### The Problem

The router is built in Rust for instant startup (~5ms vs ~200ms for Node.js). But developers expect to install CLI tools via npm:

```bash
npm install -g ever-cli
# or
npx ever os run agents
```

They should never need to install Rust, run `cargo install`, or download binaries manually.

### The Solution: Platform-Specific npm Packages

This is the same pattern used by **esbuild**, **SWC**, **Biome**, **Turbo**, **Oxlint**, **Prisma**, and **Lightning CSS**. The Rust binary is compiled for every OS/arch combination and published as separate npm packages. The main package pulls in the correct one automatically.

### Package Structure

```
ever-cli                              ← Main package, unscoped (what users install)
├── package.json                       ← Has optionalDependencies for all platforms
├── bin/ever                           ← Thin JS wrapper (5 lines)
└── npm/postinstall.js                 ← Optional: verify binary works

@ever-co/cli-darwin-arm64             ← macOS Apple Silicon (scoped)
└── ever                               ← Native Rust binary

@ever-co/cli-darwin-x64               ← macOS Intel (scoped)
└── ever

@ever-co/cli-linux-x64-gnu            ← Linux x64 glibc (scoped)
└── ever

@ever-co/cli-linux-x64-musl           ← Linux x64 musl/Alpine (scoped)
└── ever

@ever-co/cli-linux-arm64-gnu          ← Linux ARM64 glibc (scoped)
└── ever

@ever-co/cli-win32-x64-msvc           ← Windows x64 (scoped)
└── ever.exe

@ever-co/cli-win32-arm64-msvc         ← Windows ARM64 (scoped)
└── ever.exe
```

> **Why scoped for platform packages?** Product CLIs (e.g. `ever-gauzy-cli`) are unscoped to reserve the public names. Platform binary packages are internal implementation details — users never install them directly. Scoping them under `@ever-co/` keeps them organized, protected by the org, and avoids polluting the public namespace with long names like `ever-cli-linux-arm64-gnu`.

### Main Package: `ever-cli/package.json`

```json
{
  "name": "ever-cli",
  "version": "1.0.0",
  "description": "Ever CLI — unified command-line interface for the Ever ecosystem",
  "bin": {
    "ever": "bin/ever"
  },
  "optionalDependencies": {
    "@ever-co/cli-darwin-arm64": "1.0.0",
    "@ever-co/cli-darwin-x64": "1.0.0",
    "@ever-co/cli-linux-x64-gnu": "1.0.0",
    "@ever-co/cli-linux-x64-musl": "1.0.0",
    "@ever-co/cli-linux-arm64-gnu": "1.0.0",
    "@ever-co/cli-win32-x64-msvc": "1.0.0",
    "@ever-co/cli-win32-arm64-msvc": "1.0.0"
  },
  "scripts": {
    "postinstall": "node npm/postinstall.js"
  }
}
```

npm automatically installs only the `optionalDependency` matching the current platform and skips the rest. This is built-in npm behavior — no custom logic needed.

### Thin JS Wrapper: `bin/ever`

This is the only JavaScript in the entire router. It exists solely to locate and exec the native binary:

```js
#!/usr/bin/env node

const { execFileSync } = require("child_process");
const path = require("path");

const PLATFORMS = {
  "darwin-arm64":  "@ever-co/cli-darwin-arm64/ever",
  "darwin-x64":   "@ever-co/cli-darwin-x64/ever",
  "linux-x64":    "@ever-co/cli-linux-x64-gnu/ever",
  "linux-arm64":  "@ever-co/cli-linux-arm64-gnu/ever",
  "win32-x64":    "@ever-co/cli-win32-x64-msvc/ever.exe",
  "win32-arm64":  "@ever-co/cli-win32-arm64-msvc/ever.exe",
};

const key = `${process.platform}-${process.arch}`;
const binPkg = PLATFORMS[key];

if (!binPkg) {
  console.error(`Error: Unsupported platform ${key}`);
  process.exit(1);
}

const binPath = require.resolve(binPkg);

try {
  const result = execFileSync(binPath, process.argv.slice(2), {
    stdio: "inherit",
    env: process.env,
  });
} catch (e) {
  process.exit(e.status ?? 1);
}
```

> **Note:** This wrapper runs only once per invocation and adds ~30ms of Node overhead. Since the actual routing and all subsequent work happens in the native Rust binary, the total overhead is negligible. For users who want zero Node overhead, they can add the native binary directly to their PATH (see "Direct Binary Installation" below).

### Platform Package: `@ever-co/cli-darwin-arm64/package.json`

```json
{
  "name": "@ever-co/cli-darwin-arm64",
  "version": "1.0.0",
  "os": ["darwin"],
  "cpu": ["arm64"],
  "files": ["ever"],
  "preferUnpacked": true
}
```

The `os` and `cpu` fields tell npm to only install this package on matching systems. The `files` array ensures only the binary is included (tiny package size).

### CI/CD Build Pipeline

Each release compiles the Rust router for all targets and publishes all npm packages atomically:

```
GitHub Actions / Release Workflow
│
├─ Build Matrix:
│   ├─ target: aarch64-apple-darwin      → @ever-co/cli-darwin-arm64
│   ├─ target: x86_64-apple-darwin       → @ever-co/cli-darwin-x64
│   ├─ target: x86_64-unknown-linux-gnu  → @ever-co/cli-linux-x64-gnu
│   ├─ target: x86_64-unknown-linux-musl → @ever-co/cli-linux-x64-musl
│   ├─ target: aarch64-unknown-linux-gnu → @ever-co/cli-linux-arm64-gnu
│   ├─ target: x86_64-pc-windows-msvc    → @ever-co/cli-win32-x64-msvc
│   └─ target: aarch64-pc-windows-msvc   → @ever-co/cli-win32-arm64-msvc
│
├─ For each target:
│   1. cargo build --release --target <target>
│   2. Strip binary (strip / llvm-strip)
│   3. Copy binary into platform npm package dir
│   4. npm publish @ever-co/cli-<platform> --access public
│
└─ Finally:
    5. Update version in ever-cli/package.json
    6. npm publish ever-cli
```

**Cross-compilation tools:** Use `cross` (https://github.com/cross-rs/cross) or `cargo-zigbuild` for reliable cross-compilation from a single CI runner, or use GitHub Actions' matrix strategy with native runners for each OS.

### What the User Sees

```bash
# Install globally — identical to any Node CLI
$ npm install -g ever-cli

# Or use npx — no global install needed
$ npx ever-cli os run agents

# Then just use it
$ ever os run agents
$ ever teams notify kostya
$ teams notify kostya
```

The user never knows it's Rust. No cargo, no rustup, no downloading binaries. Just npm.

### Alternative Installation Methods

While npm is the primary distribution channel, the Rust binary should also be available through other channels for non-Node users:

```bash
# Cargo (for Rust developers)
cargo install ever-cli

# Homebrew (macOS/Linux)
brew install ever-co/tap/ever

# Shell script (curl | sh pattern, like rustup)
curl -fsSL https://cli.ever.co/install.sh | sh

# GitHub Releases (manual download)
# https://github.com/ever-co/ever-cli/releases/latest

# Windows installer (MSI or winget)
winget install ever-co.ever-cli
```

These are secondary channels. npm is the primary one and covers 90%+ of the target audience.

### Sub-CLI Distribution: Same Pattern

Each product sub-CLI (ever-os, ever-teams, etc.) can use the exact same distribution strategy if built in Rust:

```bash
# User installs via the ever router
ever install os
  └─ internally runs: npm install -g ever-os-cli
      └─ pulls in ever-os-cli (native binary or Node package)

# Or user installs directly
npm install -g ever-os-cli
```

Sub-CLIs can also be pure Node/TypeScript if Rust is overkill for that product. The router doesn't care what language the sub-CLI is written in — it just execs a binary.

### Binary Size Targets

The router binary should be small since it does very little:

| Component | Expected size |
|---|---|
| Router binary (stripped) | ~2–4 MB |
| Platform npm package | ~1–2 MB (compressed) |
| Main npm package (JS wrapper only) | ~5 KB |

For reference, Biome's binary is ~30MB (it does a lot more). The router should be a fraction of that.

---

## Install Flow

### `ever install <product>`

```
$ ever install os

Resolving ever-os...
  → Found ever-os-cli@0.3.1 on npm
Installing...
  → npm install -g ever-os-cli
  → Binary installed at /usr/local/bin/ever-os
  → Short alias installed at /usr/local/bin/os
Registered in ~/.ever/plugins.json

✓ ever os is ready. Try: ever os --help or os --help
```

The installer should support multiple sources:

```bash
# Auto-detect (checks npm first, then cargo, then GitHub releases)
ever install os

# Explicit source
ever install os --from npm
ever install os --from cargo
ever install os --from github

# Specific version
ever install os@1.2.3
```

**Resolution order (auto-detect):**

1. Check if a known mapping exists in a built-in product catalog (hardcoded in the router, updated with router releases).
2. Try npm: `npm install -g ever-<product>-cli`
3. Try cargo: `cargo install ever-<product>-cli`
4. Try GitHub releases: `https://github.com/ever-co/ever-<product>-cli/releases/latest`

### Built-in Product Catalog

The router ships with a hardcoded catalog that maps product names to package coordinates. This enables `ever install os` to know where to look without guessing:

```json
{
  "os":        { "npm": "ever-os-cli",        "crate": "ever-os-cli",        "repo": "ever-co/ever-os-cli" },
  "teams":     { "npm": "ever-teams-cli",     "crate": "ever-teams-cli",     "repo": "ever-co/ever-teams-cli" },
  "gauzy":     { "npm": "ever-gauzy-cli",     "crate": "ever-gauzy-cli",     "repo": "ever-co/ever-gauzy-cli" },
  "works":     { "npm": "ever-works-cli",     "crate": "ever-works-cli",     "repo": "ever-co/ever-works-cli" },
  "rec":       { "npm": "ever-rec-cli",       "crate": "ever-rec-cli",       "repo": "ever-co/ever-rec-cli" },
  "hust":      { "npm": "ever-hust-cli",      "crate": "ever-hust-cli",      "repo": "ever-co/ever-hust-cli" },
  "jobs":      { "npm": "ever-jobs-cli",      "crate": "ever-jobs-cli",      "repo": "ever-co/ever-jobs-cli" },
  "dev":       { "npm": "ever-dev-cli",       "crate": "ever-dev-cli",       "repo": "ever-co/ever-dev-cli" },
  "cloc":      { "npm": "ever-cloc-cli",      "crate": "ever-cloc-cli",      "repo": "ever-co/ever-cloc-cli" },
  "demand":    { "npm": "ever-demand-cli",    "crate": "ever-demand-cli",    "repo": "ever-co/ever-demand-cli" },
  "traduora":  { "npm": "ever-traduora-cli",  "crate": "ever-traduora-cli",  "repo": "ever-co/ever-traduora-cli" },
  "tech":      { "npm": "ever-tech-cli",      "crate": "ever-tech-cli",      "repo": "ever-co/ever-tech-cli" },
  "saas":      { "npm": "ever-saas-cli",      "crate": "ever-saas-cli",      "repo": "ever-co/ever-saas-cli" },
  "docs":      { "npm": "ever-docs-cli",      "crate": "ever-docs-cli",      "repo": "ever-co/ever-docs-cli" },
  "digital":   { "npm": "ever-digital-cli",   "crate": "ever-digital-cli",   "repo": "ever-co/ever-digital-cli" },
  "shop":      { "npm": "ever-shop-cli",      "crate": "ever-shop-cli",      "repo": "ever-co/ever-shop-cli" },
  "iq":        { "npm": "ever-iq-cli",        "crate": "ever-iq-cli",        "repo": "ever-co/ever-iq-cli" }
}
```

This catalog is shipped as static data inside the router binary. It can also be augmented at runtime from a remote endpoint (e.g. `https://cli.ever.co/catalog.json`) for discovering new products without upgrading the router.

---

## Sub-CLI Contract

Every product CLI that integrates with the `ever` router MUST follow these rules:

### 1. Binary Naming

Each sub-CLI package MUST register **two bin entries** pointing to the same executable: the short alias and the prefixed name.

**npm example (package.json):**

```json
{
  "name": "ever-os-cli",
  "version": "0.3.1",
  "bin": {
    "os": "./bin/cli.js",
    "ever-os": "./bin/cli.js"
  }
}
```

Both `os` and `ever-os` are symlinked to the same script on `npm install -g`. The third invocation mode (`ever os`) is handled by the `ever` router, not by the package's bin field.

**Rust example (Cargo.toml):**

```toml
[package]
name = "ever-os-cli"
version = "0.3.1"

[[bin]]
name = "ever-os"
path = "src/main.rs"

[[bin]]
name = "os"
path = "src/main.rs"
```

### 2. Argument Handling

The sub-CLI receives everything after the product name. It must NOT expect "os" as its first argument. All three invocation modes produce identical argv:

```
Mode A (router):     ever os run agents --verbose
Mode B (prefixed):   ever-os run agents --verbose
Mode C (short):      os run agents --verbose

All three → Sub-CLI receives: ["run", "agents", "--verbose"]
```

The sub-CLI binary detects how it was invoked by inspecting `argv[0]` but this should not affect behavior — all three modes are functionally identical.

### 3. Exit Codes

Standard Unix conventions:

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | General error |
| `2` | Usage / argument error |
| `126` | Command found but not executable |
| `127` | Command not found (sub-sub-command) |
| `130` | Interrupted (Ctrl+C / SIGINT) |

### 4. Shared Configuration

Sub-CLIs SHOULD read `~/.ever/config.toml` for shared settings (auth, org, API endpoint). They MAY maintain their own product-specific config under `~/.ever/<product>/` if needed.

```
~/.ever/
├── config.toml              # Shared (auth, org, API endpoint)
├── plugins.json             # Router manifest
├── os/
│   └── config.toml          # ever-os specific settings
├── teams/
│   └── config.toml          # ever-teams specific settings
└── ...
```

### 5. Help Output

Sub-CLIs SHOULD include a header line identifying themselves and showing all invocation methods:

```
$ os --help
ever-os 0.3.1 — Ever OS command-line interface

USAGE:
    os <command> [options]
    ever-os <command> [options]
    ever os <command> [options]

COMMANDS:
    run         Run OS agents
    plugin      Manage plugins
    config      Configure ever-os
    ...
```

### 6. Version Output

```bash
$ ever os --version
ever-os 0.3.1

$ os --version
ever-os 0.3.1
```

---

## Short Alias Commands

Every sub-CLI is accessible via a short alias that omits the `ever-` prefix. This means `cloc start timer`, `teams notify kostya`, and `gauzy export invoices` all work as top-level commands.

### How It Works

**For npm packages:** The `bin` field in `package.json` registers both names pointing to the same script. When a user runs `npm install -g ever-cloc-cli`, both `cloc` and `ever-cloc` are symlinked into the user's PATH.

```json
{
  "bin": {
    "cloc": "./bin/cli.js",
    "ever-cloc": "./bin/cli.js"
  }
}
```

**For Rust crates:** Two approaches:

*Option A — Multiple bin entries (same source, preferred):*

```toml
[[bin]]
name = "ever-cloc"
path = "src/main.rs"

[[bin]]
name = "cloc"
path = "src/main.rs"
```

*Option B — Post-install symlink:*

The install script creates a symlink: `ln -sf ever-cloc /usr/local/bin/cloc`

### Conflict Handling

Short aliases like `teams`, `rec`, `demand` are unlikely to conflict with existing system commands. However, some names carry higher risk:

| Alias | Conflict Risk | Notes |
|---|---|---|
| `os` | Low | No standard binary uses this name |
| `dev` | Medium | Some systems have `/dev` references; unlikely binary conflict |
| `cloc` | ⚠️ High | `cloc` is a well-known lines-of-code counter tool |
| `jobs` | ⚠️ High | `jobs` is a Bash/Zsh shell built-in for background job control |
| `tech` | Low | No known conflicts |
| `docs` | Low | No known conflicts |
| `shop` | Low | No known conflicts |
| `digital` | Low | No known conflicts |
| `demand` | Low | No known conflicts |
| `rec` | Low | No known conflicts |
| `hust` | Low | No known conflicts |
| `gauzy` | None | Unique name |
| `teams` | Low | No standard binary (Microsoft Teams is a GUI app, not a CLI) |
| `traduora` | None | Unique name |
| `saas` | Low | No known conflicts |
| `iq` | Low | No known conflicts |
| `works` | Low | No known conflicts |

**Conflict resolution strategy:**

1. The sub-CLI npm package registers the short alias in `bin` by default.
2. If a user already has a conflicting binary (e.g. they have the `cloc` line-counter installed), npm will warn on install. The user can still invoke via `ever-cloc` or `ever cloc`.
3. The `ever doctor` command should detect and report alias conflicts:

```
$ ever doctor

Checking short aliases...

✓ gauzy       → /usr/local/bin/gauzy (ever-gauzy-cli)
✓ teams       → /usr/local/bin/teams (ever-teams-cli)
⚠ cloc        → /usr/bin/cloc (CONFLICT: existing binary found, not ever-cloc)
               The 'cloc' alias is shadowed by an existing binary.
               Use 'ever cloc' or 'ever-cloc' instead.
✓ demand      → /usr/local/bin/demand (ever-demand-cli)
```

### Priority Order

When a user types a bare command like `cloc start timer`, the shell resolves it via standard PATH lookup. The command that appears **first** in PATH wins. There is no special Ever-level logic for short aliases — they are just regular binaries/symlinks on PATH.

The three invocation modes have different resolution paths:

| Mode | Resolution | Requires |
|---|---|---|
| `ever cloc <cmd>` | Router looks up manifest/PATH for `ever-cloc` | `ever` router installed |
| `ever-cloc <cmd>` | Direct PATH lookup for `ever-cloc` binary | Sub-CLI installed |
| `cloc <cmd>` | Direct PATH lookup for `cloc` binary (short alias) | Sub-CLI installed, no conflicts |

---

## Global Flags

The root `ever` CLI intercepts these flags BEFORE routing:

| Flag | Description |
|---|---|
| `--version`, `-V` | Print router version |
| `--help`, `-h` | Print router help (when no product specified) |
| `--verbose` | Enable verbose output (also forwarded to sub-CLI) |
| `--no-color` | Disable colored output (also forwarded to sub-CLI) |
| `--config <path>` | Override config file path |

**Important:** When a product IS specified, `--help` and `--version` are forwarded to the sub-CLI, not intercepted by the router.

```bash
ever --help          # Router help
ever os --help       # Forwarded to ever-os
ever --version       # Router version
ever os --version    # Forwarded to ever-os

# Short aliases work the same way:
os --help            # ever-os help (direct, no router involved)
os --version         # ever-os version (direct)
```

---

## `ever list` Output

```
$ ever list

Ever CLI v1.0.0 — https://ever.co

PRODUCT       STATUS       VERSION    SOURCE
gauzy         ✓ installed  1.2.0      npm
os            ✓ installed  0.3.1      cargo
teams         ✓ installed  2.1.0      npm
works         ✓ installed  0.5.0      npm
rec           ✗ not installed
hust          ✗ not installed
jobs          ✗ not installed
dev           ✓ installed  0.1.0      path
cloc          ✗ not installed
demand        ✗ not installed
traduora      ✗ not installed
tech          ✗ not installed
saas          ✗ not installed
docs          ✗ not installed
digital       ✗ not installed
shop          ✗ not installed
iq            ✗ not installed

Install a product:  ever install <product>
Update all:         ever update
```

---

## `ever doctor` Output

```
$ ever doctor

Checking Ever CLI installation...

✓ Router binary:     /usr/local/bin/ever (v1.0.0)
✓ Config directory:  ~/.ever/
✓ Config file:       ~/.ever/config.toml
✓ Manifest file:     ~/.ever/plugins.json
✓ Auth token:        configured

Checking installed products...

✓ ever-gauzy   /usr/local/bin/ever-gauzy    v1.2.0   OK
✓ ever-os      /home/user/.cargo/bin/ever-os v0.3.1  OK
✗ ever-teams   /usr/local/bin/ever-teams    MISSING (binary not found at recorded path)
✓ ever-works   /usr/local/bin/ever-works    v0.5.0   OK
✓ ever-dev     /usr/local/bin/ever-dev      v0.1.0   OK

Issues found: 1
  → ever-teams: binary missing. Run: ever install teams
```

---

## Third-Party / Community Plugins

The router should support community-contributed sub-CLIs that are not in the built-in catalog. Users can register any binary manually:

```bash
# Register a custom/community sub-CLI
ever plugin add mycustomtool --binary /path/to/ever-mycustomtool

# Now it works
ever mycustomtool do-something
```

This allows the ecosystem to grow beyond Ever's own products. Any binary following the `ever-<name>` convention can plug in.

---

## Migration Path

Since `ever-cli` and `ever-works-cli` already exist on npm:

1. **Phase 1:** Refactor the existing `ever-cli` npm package to be the thin router described here. Any existing commands currently baked into `ever-cli` should be extracted into a product sub-CLI (likely `ever-gauzy` or `ever-dev`, depending on what they do today).

2. **Phase 2:** Wrap `ever-works-cli` so its binary is renamed/aliased from whatever it is today to `ever-works`. Register it in the built-in catalog.

3. **Phase 3:** Build out new product sub-CLIs one at a time. Each is independent — no need to coordinate releases.

---

## Summary

```
┌─────────────────────────────────────────────────────────────────────┐
│  Three ways to invoke — all equivalent:                             │
│                                                                     │
│  A) ever os run agents --verbose      (via router)                  │
│  B) ever-os run agents --verbose      (direct, prefixed)            │
│  C) os run agents --verbose           (direct, short alias)         │
└───────┬──────────────────┬───────────────────┬──────────────────────┘
        │ A                │ B                  │ C
        ▼                  │                    │
┌──────────────────────┐   │                    │
│  ever (router)       │   │                    │
│                      │   │                    │
│  1. Parse: "os"      │   │                    │
│  2. Lookup ever-os   │   │                    │
│  3. exec(ever-os)    │   │                    │
└──────────┬───────────┘   │                    │
           │               │                    │
           ▼               ▼                    ▼
┌──────────────────────────────────────────────────────────────┐
│  ever-os (same binary, same behavior)                        │
│                                                              │
│  argv: ["run", "agents", "--verbose"]                        │
│                                                              │
│  → Runs agents                                               │
└──────────────────────────────────────────────────────────────┘
```

The `ever` CLI is the front door to the entire Ever ecosystem. It stays thin, stable, and backwards-compatible. Product complexity lives in the sub-CLIs where it belongs. Short aliases let power users skip the router entirely for faster, more direct access.
