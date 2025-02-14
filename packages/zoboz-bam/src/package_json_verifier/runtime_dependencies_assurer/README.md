# Why You Should Put Runtime Dependencies in `dependencies` and `peerDependencies`

Currently your package has some actual runtime dependencies that are not listed in package.json at all, or they are listed under `devDependencies`.

## Why Is It Harmful?

Relying on hoisted modules or devDependencies can result in missing or incorrect versions for consumers who install only the library’s published build. This ensures that all required packages are properly installed and grouped with the correct versions for the end user.

## Recommendations

Explicitly list the runtime dependencies under `dependencies` or `peerDependencies`.

### When use peerDependencies

- If you want the consumer of your package to share the exact copy of the dependency, so the runtime version of two would be a single instance in the memory.
- If your package's inputs/arguments ask for data structures from this 3rd-party dependency and difference in versions can cause conflicts.
- If you want to give more freedom to your package consumers in choosing the version for this dependency instead of pinning an exact version.

#### NOTICE
Package managers usually do not install packages listed in `peerDependencies` when `install` command is run on your own package. This can cause trouble for the development phase, since it would result in not having the dependency available to you during the development. To address this issue, simply duplicate the dependency from `peerDependencies` to `devDependencies`. Doing so while your benefitting from all the advantages mentioned above, you stay unblocked in your package development.

### When use dependencies

- Basically if `peerDependencies` does not fit the case, you should use `dependencies`.
