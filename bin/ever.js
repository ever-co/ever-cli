#!/usr/bin/env node

const { spawnSync } = require('child_process');
const fs = require('fs');
const path = require('path');

function resolveBinary() {
  if (process.env.EVER_CLI_BINARY) {
    return process.env.EVER_CLI_BINARY;
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
  console.error('Build it first with: cargo build --release');
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
