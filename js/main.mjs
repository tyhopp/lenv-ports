#!/usr/bin/env node

import fs from 'node:fs/promises';
import process from 'node:process';
import { WASI } from 'node:wasi';

const cwd = process.cwd();

const wasi = new WASI({
  version: 'preview1',
  args: process.argv.slice(1),
  env: process.env,
  preopens: {
    [cwd]: cwd,
  },
});

const wasiBinaryName = "lenv-wasip1-0.1.7.wasm";
const wasiBinaryPath = new URL(wasiBinaryName, import.meta.url)

try {
  await fs.access(wasiBinaryPath, fs.constants.F_OK)
} catch (_) {
  console.error(`lenv: wasi binary ${wasiBinaryPath} not found`);
  process.exit(1);
}

let wasiBinaryBuffer;

try {
  wasiBinaryBuffer = await fs.readFile(wasiBinaryPath)
} catch (_) {
  console.error(`lenv: wasi binary ${wasiBinaryPath} not readable`);
  process.exit(1);
}

const wasm = await WebAssembly.compile(wasiBinaryBuffer);
const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());

wasi.start(instance);