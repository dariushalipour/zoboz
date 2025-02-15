# Why Runtime Dependencies Belong in `dependencies` or `peerDependencies`

Your package has runtime dependencies that are either missing from `package.json` or incorrectly listed under `devDependencies`.

## Why Is This a Problem?

Relying on hoisted modules or `devDependencies` can lead to missing or incorrect versions when consumers install only the published build. Explicitly declaring runtime dependencies ensures all required packages are installed with correct versions.

## Recommendations

List runtime dependencies under `dependencies` or `peerDependencies` based on usage.

### When to Use `peerDependencies`

- When you want consumers to share the same dependency instance in memory.
- When your package accepts data structures from the dependency, and version mismatches could cause conflicts.
- When you want to allow consumers flexibility in choosing the dependency version instead of pinning an exact one.

#### Important Note

Package managers do not install `peerDependencies` by default. This can cause issues during development if the dependency is unavailable. To avoid this, duplicate the dependency in `devDependencies` while still benefiting from the flexibility of `peerDependencies`.

### When to Use `dependencies`

- If `peerDependencies` is not suitable, use `dependencies` instead.
