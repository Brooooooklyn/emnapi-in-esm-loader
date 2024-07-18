# `esm-loader bug`

- corepack enable
- pnpm install
- pnpm build --target wasm32-wasip1-threads
- node --import ./esm.mjs ./test.mjs
