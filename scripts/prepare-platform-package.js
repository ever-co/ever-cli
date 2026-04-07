#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const [packageDirArg, binaryPathArg, outputDirArg] = process.argv.slice(2);

if (!packageDirArg || !binaryPathArg || !outputDirArg) {
  console.error(
    'Usage: node ./scripts/prepare-platform-package.js <package-dir> <binary-path> <output-dir>',
  );
  process.exit(1);
}

const packageDir = path.resolve(repoRoot, packageDirArg);
const binaryPath = path.resolve(repoRoot, binaryPathArg);
const outputDir = path.resolve(repoRoot, outputDirArg);

if (!fs.existsSync(packageDir)) {
  console.error(`Package directory not found: ${packageDir}`);
  process.exit(1);
}

if (!fs.existsSync(binaryPath)) {
  console.error(`Binary not found: ${binaryPath}`);
  process.exit(1);
}

fs.mkdirSync(outputDir, { recursive: true });

const packageJsonPath = path.join(packageDir, 'package.json');
const pkg = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const binaryName = pkg.files[0];
const licensePath = path.join(repoRoot, 'LICENSE.md');

fs.copyFileSync(packageJsonPath, path.join(outputDir, 'package.json'));
if (fs.existsSync(licensePath)) {
  fs.copyFileSync(licensePath, path.join(outputDir, 'LICENSE.md'));
}
fs.copyFileSync(path.join(repoRoot, 'README.md'), path.join(outputDir, 'README.md'));
fs.copyFileSync(binaryPath, path.join(outputDir, binaryName));

console.log(`Prepared ${pkg.name} in ${outputDir}`);
