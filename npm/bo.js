#!/usr/bin/env node

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const key = `${process.platform}-${process.arch}`;
const executable = key === "win32-x64" ? "bo.exe" : "bo";
const supportedPlatforms = new Set([
  "darwin-arm64",
  "darwin-x64",
  "linux-x64",
  "win32-x64"
]);

if (!supportedPlatforms.has(key)) {
  console.error(`@inoader/bo does not provide a prebuilt binary for ${key}.`);
  console.error("Supported platforms: darwin-arm64, darwin-x64, linux-x64, win32-x64.");
  process.exit(1);
}

const binaryPath = path.join(__dirname, "bin", key, executable);

if (!fs.existsSync(binaryPath)) {
  console.error(`bo prebuilt binary is missing: ${binaryPath}`);
  console.error("Please reinstall the package.");
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit"
});

if (result.error) {
  console.error(`bo prebuilt binary could not be executed: ${binaryPath}`);
  process.exit(1);
}

if (result.signal) {
  process.kill(process.pid, result.signal);
} else {
  process.exit(result.status ?? 1);
}
