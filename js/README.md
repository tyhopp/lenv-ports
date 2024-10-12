# lenv-js

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

3. Execute `npm install --save-dev lenv-js` to install lenv
4. Create a run script in `package.json`:

```json
{
  "scripts": {
    "lenv": "NODE_NO_WARNINGS=1 lenv",
  }
}
```

5. Execute `npm run lenv link` to create symlinks

Use the `-help` flag to see all usage instructions.

## WASI experimental warning

The [Node.js implementation](https://nodejs.org/api/wasi.html) of the [WebAssembly System Interface (WASI)](https://wasi.dev/) is experimental. This module uses the WASI binary of [lenv](https://github.com/tyhopp/lenv) (which is written in Go), so you will see an experimental warning when running this module's bin scripts.

You can silence these warnings by declaring a truthy value for the `NODE_NO_WARNINGS` variable: 

```
NODE_NO_WARNINGS=1 npx lenv-js@latest link
```
