#!/usr/bin/env node

const { spawnSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const PLATFORM_PACKAGES = {
  'darwin-arm64': '@ever-co/cli-darwin-arm64/ever',
  'darwin-x64': '@ever-co/cli-darwin-x64/ever',
  'linux-arm64': '@ever-co/cli-linux-arm64-gnu/ever',
  'linux-x64': '@ever-co/cli-linux-x64-gnu/ever',
  'linux-x64-musl': '@ever-co/cli-linux-x64-musl/ever',
  'win32-arm64': '@ever-co/cli-win32-arm64-msvc/ever.exe',
  'win32-x64': '@ever-co/cli-win32-x64-msvc/ever.exe',
};

function isMuslLinux() {
  if (process.platform !== 'linux') {
    return false;
  }

  if (typeof process.report?.getReport === 'function') {
    const report = process.report.getReport();
    if (report?.header?.glibcVersionRuntime) {
      return false;
    }
  }

  if (fs.existsSync('/etc/alpine-release')) {
    return true;
  }

  const muslPaths = ['/lib', '/usr/sbin'];
  return muslPaths.some((dir) => {
    try {
      return fs.readdirSync(dir).some((entry) => entry.startsWith('ld-musl-') || entry.startsWith('libc.musl-'));
    } catch {
      return false;
    }
  });
}

function resolveInstalledPlatformBinary() {
  const key =
    process.platform === 'linux' && process.arch === 'x64' && isMuslLinux()
      ? 'linux-x64-musl'
      : `${process.platform}-${process.arch}`;
  const packageEntry = PLATFORM_PACKAGES[key];

  if (!packageEntry) {
    return null;
  }

  try {
    return require.resolve(packageEntry);
  } catch {
    return null;
  }
}

function resolveBinary() {
  if (process.env.EVER_CLI_BINARY) {
    return process.env.EVER_CLI_BINARY;
  }

  const installedBinary = resolveInstalledPlatformBinary();
  if (installedBinary) {
    return installedBinary;
  }

  const repoRoot = path.resolve(__dirname, '..');
  const extension = process.platform === 'win32' ? '.exe' : '';
  const candidates = [
    path.join(repoRoot, 'target', 'release', `ever${extension}`),
    path.join(repoRoot, 'target', 'debug', `ever${extension}`),
  ];

  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }

  return null;
}

const binary = resolveBinary();

if (!binary) {
  console.error('Error: Ever CLI native binary was not found.');
  console.error('Install the matching platform package or build it locally with: cargo build --release');
  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), {
  stdio: 'inherit',
  env: process.env,
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);
