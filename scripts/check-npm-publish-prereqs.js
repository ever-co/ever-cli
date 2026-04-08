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
  try {
    return fs
      .readdirSync(npmDir, { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && entry.name.startsWith('cli-'))
      .map((entry) => {
        const packageJsonPath = path.join(npmDir, entry.name, 'package.json');
        const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

        if (!packageJson.name || typeof packageJson.name !== 'string') {
          throw new Error(`Missing package name in ${packageJsonPath}`);
        }

        return packageJson.name;
      })
      .sort();
  } catch (error) {
    fail('Failed to load platform package definitions from npm/.', error);
  }
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
  const output = run('npm', ['access', 'list-packages', scope, '--json']);
  accessiblePackages = output ? JSON.parse(output) : {};
} catch (error) {
  fail(
    `Unable to verify publish access for ${scope}. Ensure the npm token belongs to a user with write access to the ${scope} organization.`,
    error,
  );
}

const platformPackages = loadPlatformPackages();
console.log(`Verified npm scope access for ${scope}.`);

const writablePackages = platformPackages.filter(
  (pkg) => Object.hasOwn(accessiblePackages, pkg) && accessiblePackages[pkg] === 'read-write',
);
const readOnlyPackages = platformPackages.filter(
  (pkg) => Object.hasOwn(accessiblePackages, pkg) && accessiblePackages[pkg] !== 'read-write',
);

if (readOnlyPackages.length > 0) {
  fail(
    `The npm token only has read-only access for: ${readOnlyPackages.join(
      ', ',
    )}. Publish requires read-write access.`,
  );
}

if (writablePackages.length > 0) {
  console.log(`Existing writable platform packages in scope: ${writablePackages.join(', ')}`);
} else {
  console.log(
    'No platform packages are currently visible in the scope. That is acceptable for a first publish as long as the token has organization publish access.',
  );
}
