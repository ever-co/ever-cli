#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const PLATFORM_PACKAGES = {
  'darwin-arm64': '@ever-co/cli-darwin-arm64/ever',
  'darwin-x64': '@ever-co/cli-darwin-x64/ever',
  'linux-arm64': '@ever-co/cli-linux-arm64-gnu/ever',
  'linux-x64': '@ever-co/cli-linux-x64-gnu/ever',
  'win32-arm64': '@ever-co/cli-win32-arm64-msvc/ever.exe',
  'win32-x64': '@ever-co/cli-win32-x64-msvc/ever.exe',
};

const extension = process.platform === 'win32' ? '.exe' : '';
const releaseBinary = path.resolve(__dirname, '..', 'target', 'release', `ever${extension}`);
const debugBinary = path.resolve(__dirname, '..', 'target', 'debug', `ever${extension}`);

function hasInstalledPlatformBinary() {
  const key = `${process.platform}-${process.arch}`;
  const packageEntry = PLATFORM_PACKAGES[key];

  if (!packageEntry) {
    return false;
  }

  try {
    require.resolve(packageEntry);
    return true;
  } catch {
    return false;
  }
}

if (!hasInstalledPlatformBinary() && !fs.existsSync(releaseBinary) && !fs.existsSync(debugBinary)) {
  console.warn('[ever-cli] Native router binary is not present yet. Build with `cargo build --release`.');
}
