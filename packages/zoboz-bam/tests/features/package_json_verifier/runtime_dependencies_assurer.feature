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
      const foo = require('package-not-available');
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
      const foo = require('package-not-available');
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --absolute-source-dir $scenario_dir/src --absolute-output-dir $scenario_dir/dist --can-update-package-json
      """
    Then the result is error and equals the following text:
      """
      Runtime dependency `package-not-available` is not listed in package.json field `dependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md
      """
  # Scenario: If a runtime dependency is directly listed in dependencies,
  # in validate-mode, since it is already listed, no action is needed
  # Scenario: If a runtime dependency is not directly listed at all,
  # but is resolved from node_modules,
  # in validate-mode, it will be requested to be added to dependencies
  # Scenario: If a runtime dependency is not directly listed at all,
  # but is resolved from node_modules,
  # in fix-mode, it will be added to dependencies
  # Scenario: If a runtime dependency is listed in devDependencies,
  # but is resolved from node_modules,
  # in validate-mode, it will be requested to be moved to dependencies
  # Scenario: If a runtime dependency is listed in devDependencies,
  # but is resolved from node_modules,
  # in fix-mode, it will be moved to dependencies
