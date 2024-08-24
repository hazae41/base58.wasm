# base58.wasm

WebAssembly port of Base58

```bash
npm i @hazae41/base58.wasm
```

[**Node Package 📦**](https://www.npmjs.com/package/@hazae41/base58.wasm)

## Features
- Reproducible building
- Pre-bundled and streamed
- Zero-copy memory slices

## Modules
- bs58

## Algorithms
- Base58

## Usage

```typescript
import { Base58Wasm, base58_encode, base58_decode } from "@hazae41/base58.wasm";

// Wait for WASM to load
await Base58Wasm.initBundled();

using memory = new Memory(crypto.getRandomValues(new Uint8Array(256)))

const encoded = base58_encode(memory)
using decoded = base58_decode(encoded)

console.log(encoded, decoded.bytes)
```

## Building

### Unreproducible building

You need to install [Rust](https://www.rust-lang.org/tools/install)

Then, install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
cargo install wasm-pack
```

Finally, do a clean install and build

```bash
npm ci && npm run build
```

### Reproducible building

You can build the exact same bytecode using Docker, just be sure you're on a `linux/amd64` host

```bash
docker compose up --build
```

Then check that all the files are the same using `git status`

```bash
git status --porcelain
```

If the output is empty then the bytecode is the same as the one I commited

### Automated checks

Each time I commit to the repository, the GitHub's CI does the following:
- Clone the repository
- Reproduce the build using `docker compose up --build`
- Throw an error if the `git status --porcelain` output is not empty

Each time I release a new version tag on GitHub, the GitHub's CI does the following:
- Clone the repository
- Do not reproduce the build, as it's already checked by the task above
- Throw an error if there is a `npm diff` between the cloned repository and the same version tag on NPM

If a version is present on NPM but not on GitHub, do not use!
