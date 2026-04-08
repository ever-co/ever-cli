# Ever CLI

`ever-cli` is the root router for the Ever CLI ecosystem.

It provides a single top-level command:

```bash
ever <product> <command> [args...]
```

Examples:

```bash
ever works init
ever works --help
ever install works
ever doctor
```

The router does not contain product-specific business logic. It resolves and forwards commands to product CLIs such as `ever-works`.

## Current Architecture

This repository is being migrated from the old TypeScript stub to a Rust-based router with:

- a native Rust binary as the runtime
- a thin npm wrapper in `bin/ever.js`
- platform-specific npm packages for native binaries
- manifest-based plugin resolution via `~/.ever/plugins.json`

Reference docs:

- [docs/SPEC.md](./docs/SPEC.md)
- [docs/IMPLEMENTATION_SPEC.md](./docs/IMPLEMENTATION_SPEC.md)

## Local Development

Build the native router:

```bash
cargo build --release
```

Run the wrapper against the local build:

```bash
node ./bin/ever.js --help
```

Or after install:

```bash
ever --help
```

Useful commands:

```bash
npm run sync:versions
npm run check:publish-prereqs
```

## Packaging Model

The main package is:

- `ever-cli`

It depends on platform-specific native packages such as:

- `@ever-co/cli-linux-x64-gnu`
- `@ever-co/cli-darwin-arm64`
- `@ever-co/cli-win32-x64-msvc`

The publish flow is:

1. build native binaries
2. prepare platform package directories
3. publish platform packages
4. publish the main `ever-cli` package

## Publishing on NPM

Publishing is currently driven by GitHub Actions:

- native build workflow:
  - `.github/workflows/build-native-binaries.yml`
- npm publish workflow:
  - `.github/workflows/publish-npm-packages.yml`

Before publish, the npm token must have publish access to the `@ever-co` npm scope.

The publish workflow now verifies:

- npm authentication via `npm whoami`
- scope access via `npm access list packages @ever-co --json`

If those checks fail, the workflow stops before building and publishing artifacts.

### Manual prerequisite check

```bash
npm run check:publish-prereqs
```

## CI

GitHub Actions is the primary CI path for native builds and npm publishing.

CircleCI is kept as an additional build verification path and no longer documents the old npm release flow.

## License

See [LICENSE.md](./LICENSE.md).
