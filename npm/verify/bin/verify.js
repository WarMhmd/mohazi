#!/usr/bin/env node

const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');

const platform = process.platform;
const arch = process.arch;

const platformPackages = {
  'win32-x64': '@uj_project/verify-win32-x64',
  'win32-arm64': '@uj_project/verify-win32-arm64',
  'darwin-x64': '@uj_project/verify-darwin-x64',
  'darwin-arm64': '@uj_project/verify-darwin-arm64',
  'linux-x64': '@uj_project/verify-linux-x64',
  'linux-arm64': '@uj_project/verify-linux-arm64',
};

const key = `${platform}-${arch}`;
const packageName = platformPackages[key];

if (!packageName) {
  console.error(`Unsupported platform: ${platform} ${arch}`);
  process.exit(1);
}

let binaryPath;
try {
  // Try to resolve the package
  const packagePath = path.dirname(require.resolve(`${packageName}/package.json`));
  const binaryName = platform === 'win32' ? 'verify.exe' : 'verify';
  binaryPath = path.join(packagePath, 'bin', binaryName);
} catch (e) {
  console.error(`Error: Could not find platform-specific package "${packageName}".`);
  console.error(`Please ensure "@uj_project/verify" was installed correctly for your platform.`);
  process.exit(1);
}

const args = process.argv.slice(2);
const result = spawnSync(binaryPath, args, { stdio: 'inherit' });

process.exit(result.status ?? 1);
