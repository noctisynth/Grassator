import { execSync } from 'child_process';
import { readFileSync, writeFileSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

execSync('npx changeset version', { stdio: 'inherit' });
execSync('pnpm install --no-frozen-lockfile', { stdio: 'inherit' });

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const cargoFilePath = path.join(__dirname, '../src-tauri/Cargo.toml');
const nodeFilePath = path.join(__dirname, '../src-tauri/package.json');
const workspaceFilePath = path.join(__dirname, '../package.json');

const cargoData = readFileSync(cargoFilePath, 'utf8');
const nodeData = readFileSync(nodeFilePath, 'utf8');
const workspaceData = readFileSync(workspaceFilePath, 'utf8');

const newVersion = JSON.parse(workspaceData).version;
const newCargoVersion = JSON.parse(nodeData).version;
const updatedData = cargoData.replace(
  /version = ".*"/,
  `version = "${newCargoVersion}"`
);

writeFileSync(cargoFilePath, updatedData, 'utf8');

// const nodeChangelogFilePath = path.join(__dirname, '../CHANGELOG.md');
// const nodeChangelogData = readFileSync(nodeChangelogFilePath, 'utf8');
// const cargoChangeLogFilePath = path.join(
//   __dirname,
//   '../src-tauri/CHANGELOG.md'
// );
// const cargoChangeLogData = readFileSync(cargoChangeLogFilePath, 'utf8');
