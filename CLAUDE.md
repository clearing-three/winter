## Dependencies

All dependencies must be pinned to exact versions.

### Version Constraints

- `@eslint-react/eslint-plugin`: Pinned to 3.0.0 until @antfu/eslint-config adds support for 4.x. Dependabot is configured to ignore major updates for this package.

## Tech Stack

### Frontend
- Node.js: 24
- pnpm: 10
- React: 19
- TypeScript: 5 (target: ES2022)
- Vite: 6
- ESLint: 9

### Desktop/Backend
- Rust: 1.95
- Tauri: 2
- SQLite with migrations

### Target Platform
- Debian Trixie
- WebKitGTK: webkit2gtk-4.1

## Configuration


## Code Style

Prefer self-documenting code.

## Validation

**REQUIRED**: After making any code changes, you MUST run `pnpm validate` before considering the task complete. This is not optional.

## Development Workflow


### Git Branch Naming

- use short names, do not prefix with 'feature/', 'bugfix/', etc


### Branch Protection
**IMPORTANT**: Never make code changes when on the `main` branch. If the user requests changes while on `main`:
1. Check current branch with `git branch --show-current`
2. If on `main`, do NOT make any file changes
3. Inform the user: "Currently on main branch. Should I create a feature branch or would you like to switch branches first?"
4. Wait for user instructions before proceeding


