#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const extension = process.platform === 'win32' ? '.exe' : '';
const releaseBinary = path.resolve(__dirname, '..', 'target', 'release', `ever${extension}`);
const debugBinary = path.resolve(__dirname, '..', 'target', 'debug', `ever${extension}`);

if (!fs.existsSync(releaseBinary) && !fs.existsSync(debugBinary)) {
  console.warn('[ever-cli] Native router binary is not present yet. Build with `cargo build --release`.');
}
