# Winter

A personal finance application

## Development workflows

### Complete App (Rust + React)

#### Commands
* full validation (formatting + linting + tests): `pnpm validate`
* all tests: `pnpm test`
* launch app: `tauri dev`

### Rust (backend)
* rust tests: `pnpm test:tauri`
* rust test coverage: `pnpm test:coverage:tauri`
  * generated report: `./src-tauri/target/llvm-cov/html/index.html`

### React (frontend)
* Typescript tests: `pnpm test:frontend`


## Release workflows
* Create distributable app: `tauri build`
  * Creates AppImage (frontend + backend + native wrapper)

## First time setup

```
cargo install cargo-llvm-cov
```
