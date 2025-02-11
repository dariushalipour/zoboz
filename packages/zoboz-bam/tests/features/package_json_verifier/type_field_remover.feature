Feature: Ensure field "type" is not present

  Scenario: When field "type" is not present, in validate-mode, no change is requested
    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js"
      }
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir
      """
    Then the result is ok

  Scenario: When field "type" is not present, in fix-mode, no change will happen to package.json
    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "main": "dist/cjs/index.js"
      }
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --can-update-package-json
      """
    Then the JSON content for "package.json" should be:
      """
        {
          "name": "test",
          "version": "1.0.0",
          "main": "dist/cjs/index.js"
        }
      """

  Scenario: When field "type" is not present, in validate-mode, removal is requested
    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "type": "whatever",
        "main": "dist/cjs/index.js"
      }
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir
      """
    Then the result is error and equals the following text:
      """
      Field `type` in package.json should not exist. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/type_field_remover/README.md
      """

  Scenario: When field "type" is present, in fix-mode, it will get removed from package.json
    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "type": "whatever",
        "main": "dist/cjs/index.js",
        "dependencies": {
          "dep1": "1.0.0"
        }
      }
      """
    When the following command is executed:
      """
      verify-package-json --absolute-package-dir $scenario_dir --can-update-package-json
      """
    Then the JSON content for "package.json" should be:
      """
        {
          "name": "test",
          "version": "1.0.0",
          "main": "dist/cjs/index.js",
          "dependencies": {
            "dep1": "1.0.0"
          }
        }
      """
