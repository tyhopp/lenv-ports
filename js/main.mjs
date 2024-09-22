import { readFile } from 'node:fs/promises';
import { WASI } from 'node:wasi';
import { argv, env } from 'node:process';

const cwd = process.cwd();

const wasi = new WASI({
  version: 'preview1',
  args: argv.slice(1),
  env,
  preopens: {
    [cwd]: cwd,
  },
});

const wasm = await WebAssembly.compile(
  await readFile(new URL('./lenv-wasip1-0.1.7.wasm', import.meta.url)),
);
const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());

wasi.start(instance);