#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const rootPackagePath = path.join(repoRoot, 'package.json');
const rootPackage = JSON.parse(fs.readFileSync(rootPackagePath, 'utf8'));
const version = rootPackage.version;

const platformPackageDirs = [
  'npm/cli-darwin-arm64',
  'npm/cli-darwin-x64',
  'npm/cli-linux-arm64-gnu',
  'npm/cli-linux-x64-gnu',
  'npm/cli-linux-x64-musl',
  'npm/cli-win32-arm64-msvc',
  'npm/cli-win32-x64-msvc',
];

for (const dir of platformPackageDirs) {
  const packagePath = path.join(repoRoot, dir, 'package.json');
  const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
  pkg.version = version;
  fs.writeFileSync(packagePath, `${JSON.stringify(pkg, null, 2)}\n`);
}

const optionalDependencies = {};
for (const dir of platformPackageDirs) {
  const packagePath = path.join(repoRoot, dir, 'package.json');
  const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
  optionalDependencies[pkg.name] = version;
}

rootPackage.optionalDependencies = optionalDependencies;
fs.writeFileSync(rootPackagePath, `${JSON.stringify(rootPackage, null, 2)}\n`);

console.log(`Synchronized platform package versions to ${version}`);
