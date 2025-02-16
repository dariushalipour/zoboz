#!/usr/bin/env bash

mkdir -p $(pwd)/temp
echo '' >$(pwd)/temp/test.docker.sh.results

npm -w @zoboz/bam run build-binaries
./verdaccio/run.sh --start >/dev/null
PUBLISH_REGISTRY=http://localhost:4873 npm -w @zoboz/bam run publish-binaries
npm -w @zoboz/bam publish --registry http://localhost:4873
npm install --registry http://localhost:4873
npm -w @zoboz/core run build
./verdaccio/run.sh --stop >/dev/null

for platform in "linux/amd64" "linux/arm64"; do
  for node_version in 14 22; do
    for ts_version in 4 5; do
      log_env="platform: $platform, node: $node_version, ts: $ts_version"
      prepare_level_one="rm -rf node_modules package-lock.json && npm i && npm run build && npm publish"
      test_level_two="node_modules package-lock.json && npm i && npm run test && npm run typecheck"
      ./verdaccio/run.sh --start >/dev/null
      PUBLISH_REGISTRY=http://localhost:4873 npm -w @zoboz/bam run publish-binaries
      npm -w @zoboz/bam publish --registry http://localhost:4873
      npm -w @zoboz/core publish --registry http://localhost:4873
      inline_commands="
        export NPM_CONFIG_REGISTRY=http://verdaccio:4873 &&
        npm config set '//verdaccio:4873/:_authToken' 'OGQ5NDgxYjAwOWZkNzFhZmRlZGZiOTNjZGZhY2QyYTY6MTEwZDlkOTQ0YWVjYWViNjIzYzE5NTg4M2I1NzUx' &&
        npm config set registry http://verdaccio:4873 &&
        echo '⏳ Testing   ($log_env)' >> /app/temp/test.docker.sh.results &&
        npm install -g typescript@$ts_version &&
        cd /app/level-ones/tsc-bundler && eval \"$prepare_level_one\" &&
        cd /app/level-ones/tsc-node10 && eval \"$prepare_level_one\" &&
        cd /app/level-ones/tsc-node16 && eval \"$prepare_level_one\" &&
        cd /app/level-ones/esbuild-bundler && eval \"$prepare_level_one\" &&
        cd /app/level-ones/esbuild-node10 && eval \"$prepare_level_one\" &&
        cd /app/level-ones/esbuild-node16 && eval \"$prepare_level_one\" &&
        cd /app/level-twos/bundler && rm -rf eval \"$test_level_two\" &&
        cd /app/level-twos/node10 && rm -rf eval \"$test_level_two\" &&
        cd /app/level-twos/node16 && rm -rf eval \"$test_level_two\" &&
        echo '🍃 Done      ($log_env)' >> /app/temp/test.docker.sh.results ||
        echo '🌶️ Failed    ($log_env)' >> /app/temp/test.docker.sh.results
      "
      docker run -it --rm \
        --platform $platform \
        --network zoboz-net \
        -v $(pwd)/tests:/app \
        -v $(pwd)/temp:/app/temp \
        -w /app \
        node:$node_version \
        bash -c "$inline_commands"
      ./verdaccio/run.sh --stop >/dev/null
    done
  done
done
