#!/usr/bin/env node

const { execFileSync } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');

const repoRoot = path.resolve(__dirname, '..');
const scope = '@ever-co';

function run(command, args) {
  return execFileSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
}

function fail(message, error) {
  console.error(message);
  if (error?.stderr) {
    console.error(error.stderr.toString().trim());
  } else if (error?.message) {
    console.error(error.message);
  }
  process.exit(1);
}

function loadPlatformPackages() {
  const npmDir = path.join(repoRoot, 'npm');
  return fs
    .readdirSync(npmDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && entry.name.startsWith('cli-'))
    .map((entry) => {
      const packageJsonPath = path.join(npmDir, entry.name, 'package.json');
      return JSON.parse(fs.readFileSync(packageJsonPath, 'utf8')).name;
    })
    .sort();
}

try {
  const username = run('npm', ['whoami']);
  console.log(`Authenticated to npm as ${username}`);
} catch (error) {
  fail(
    'Unable to authenticate to npm. Ensure NPM_TOKEN is set and has permission to publish packages.',
    error,
  );
}

let accessiblePackages;
try {
  const output = run('npm', ['access', 'ls-packages', scope, '--json']);
  accessiblePackages = output ? JSON.parse(output) : {};
} catch (error) {
  fail(
    `Unable to verify publish access for ${scope}. Ensure the npm token belongs to a user with write access to the ${scope} organization.`,
    error,
  );
}

const platformPackages = loadPlatformPackages();
console.log(`Verified npm scope access for ${scope}.`);

const publishedPackages = platformPackages.filter((pkg) => Object.hasOwn(accessiblePackages, pkg));
if (publishedPackages.length > 0) {
  console.log(`Existing platform packages in scope: ${publishedPackages.join(', ')}`);
} else {
  console.log(
    'No platform packages are currently visible in the scope. That is acceptable for a first publish as long as the token has organization publish access.',
  );
}
