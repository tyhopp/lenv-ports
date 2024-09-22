# lenv

Manage symlinks from a root file to multiple destinations.

Useful for monorepos that use a single `.env` file as a source of truth for many child projects.

## Usage

In the root of your project:

1. Create a `.env` (or other named) file you want to symlink
2. Create a `.lenv` file with the destination locations to symlink to, such as:

```
project/a/.env
project/b/.env
```

3. Execute `npx lenv@latest link` to create symlinks.

Use the `-help` flag to see all usage instructions.

## WASI experimental warning

The [Node.js implementation](https://nodejs.org/api/wasi.html) of the [WebAssembly System Interface (WASI)](https://wasi.dev/) is experimental. This module uses the WASI binary of [lenv](https://github.com/tyhopp/lenv), so you will see an experimental warning when running this module's bin scripts.

You can silence these warnings with the `--no-warnings` flag:

```
npx --no-warnings lenv@latest link
```
