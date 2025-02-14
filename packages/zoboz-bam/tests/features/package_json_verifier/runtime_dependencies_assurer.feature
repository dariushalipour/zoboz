Feature: Ensures runtime dependencies will be available for the consumers

  Scenario: If a runtime dependency is not directly listed at all,
  and could not be resolved either,
  in validate-mode, it will be requested to be added to field "dependencies"

    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js"
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/cjs"
    And there is a file named "dist/cjs/index.js" with:
      """
      require('package-not-available/xyz/abc');
      require('package-not-available/xyz');
      require('package-not-available');
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist
      """
    Then the result is error and equals the following text:
      """
      Runtime dependency `package-not-available` is not listed in package.json field `dependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md
      """

  Scenario: If a runtime dependency is not directly listed at all,
  and could not be resolved either,
  in fix-mode, since we do not know the exact version,
  it will be requested to be added to field "dependencies" manually

    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js"
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/cjs"
    And there is a file named "dist/cjs/index.js" with:
      """
      require('@package-not/available/xyz/abc');
      require('@package-not/available/xyz');
      require('@package-not/available');
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist --can-update-package-json
      """
    Then the result is error and equals the following text:
      """
      Runtime dependency `@package-not/available` is not listed in package.json field `dependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md
      """

  Scenario: If a runtime dependency is directly listed in dependencies,
  in validate-mode, since it is already listed, no action is needed

    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "package-available": "1.0.0",
          "@package/available": "1.0.0"
        }
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/cjs"
    And there is a file named "dist/cjs/index.js" with:
      """
      require('@package/available/xyz/abc');
      require('package-available/xyz/abc');
      require('@package/available');
      require('package-available');
      """
    And there is a JSON file named "node_modules/@package/available/package.json" with:
      """
      {
        "name": "@package/available",
        "version": "1.0.0",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/@package/available/index.js" with:
      """
      module.exports = {};
      """
    And there is a JSON file named "node_modules/package-available/package.json" with:
      """
      {
        "name": "package-available",
        "version": "1.0.0",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/package-available/index.js" with:
      """
      module.exports = {};
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist
      """
    Then the result is ok

  Scenario: If a runtime dependency is not directly listed in dependencies or peerDependencies,
  but is resolved from node_modules,
  in validate-mode, it will be requested to be added to dependencies

    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "package-available": "1.0.0"
        }
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/cjs"
    And there is a file named "dist/cjs/index.js" with:
      """
      require('@package-not/listed/xyz/abc');
      require('package-available/xyz/abc');
      require('package-available');
      """
    And there is a JSON file named "node_modules/@package-not/listed/package.json" with:
      """
      {
        "name": "@package-not/listed",
        "version": "1.0.0",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/@package-not/listed/index.js" with:
      """
      module.exports = {};
      """
    And there is a JSON file named "node_modules/package-available/package.json" with:
      """
      {
        "name": "package-available",
        "version": "1.0.0",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/package-available/index.js" with:
      """
      module.exports = {};
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist
      """
    Then the result is error and equals the following text:
      """
      Runtime dependency `@package-not/listed` is not listed in package.json field `dependencies` or `peerDependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md
      """

  Scenario: If a runtime dependency is not directly listed at all,
  but is resolved from node_modules,
  in fix-mode, it will be added to dependencies

    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "package-available": "1.0.0"
        }
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/cjs"
    And there is a file named "dist/cjs/index.js" with:
      """
      require('@package-not/listed/xyz/abc');
      require('package-available/xyz/abc');
      require('package-available');
      """
    And there is a JSON file named "node_modules/@package-not/listed/package.json" with:
      """
      {
        "name": "@package-not/listed",
        "version": "2.3.4",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/@package-not/listed/index.js" with:
      """
      module.exports = {};
      """
    And there is a JSON file named "node_modules/package-available/package.json" with:
      """
      {
        "name": "package-available",
        "version": "1.0.0",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/package-available/index.js" with:
      """
      module.exports = {};
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist --can-update-package-json
      """
    Then the result is ok
    And the JSON content for "package.json" should be:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "package-available": "1.0.0",
          "@package-not/listed": "2.3.4"
        }
      }
      """

  Scenario: If a runtime dependency is listed in devDependencies,
  but is resolved from node_modules,
  in validate-mode, it will be requested to be moved to dependencies

    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "package-available": "1.0.0"
        },
        "devDependencies": {
          "@package-not/listed": "2.3.4"
        }
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/cjs"
    And there is a file named "dist/cjs/index.js" with:
      """
      require('@package-not/listed/xyz/abc');
      require('package-available/xyz/abc');
      require('package-available');
      """
    And there is a JSON file named "node_modules/@package-not/listed/package.json" with:
      """
      {
        "name": "@package-not/listed",
        "version": "2.3.4",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/@package-not/listed/index.js" with:
      """
      module.exports = {};
      """
    And there is a JSON file named "node_modules/package-available/package.json" with:
      """
      {
        "name": "package-available",
        "version": "1.0.0",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/package-available/index.js" with:
      """
      module.exports = {};
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist
      """
    Then the result is error and equals the following text:
      """
      Runtime dependency `@package-not/listed` is listed in package.json field `devDependencies`. It should be moved to `dependencies` or get duplicated to `peerDependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md
      """

  Scenario: If a runtime dependency is listed in devDependencies,
  but is resolved from node_modules,
  in fix-mode, it will be moved to dependencies

    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "package-available": "1.0.0"
        },
        "devDependencies": {
          "@package-not/listed": "2.3.4"
        }
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/cjs"
    And there is a file named "dist/cjs/index.js" with:
      """
      require('@package-not/listed/xyz/abc');
      require('package-available/xyz/abc');
      require('package-available');
      """
    And there is a JSON file named "node_modules/@package-not/listed/package.json" with:
      """
      {
        "name": "@package-not/listed",
        "version": "2.3.4",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/@package-not/listed/index.js" with:
      """
      module.exports = {};
      """
    And there is a JSON file named "node_modules/package-available/package.json" with:
      """
      {
        "name": "package-available",
        "version": "1.0.0",
        "main": "index.js",
        "exports": {
          ".": "./index.js",
          "./xyz/abc": "./index.js"
        }
      }
      """
    And there is a file named "node_modules/package-available/index.js" with:
      """
      module.exports = {};
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist --can-update-package-json
      """
    Then the result is ok
    And the JSON content for "package.json" should be:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "package-available": "1.0.0",
          "@package-not/listed": "2.3.4"
        },
        "devDependencies": {}
      }
      """
