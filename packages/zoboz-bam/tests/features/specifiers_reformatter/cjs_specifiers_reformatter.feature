Feature: CommonJS Specifier Formatter

  Scenario: Importing a CommonJS module with a specifier
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
    And there is a file named "tsconfig.json" with:
      """
      {
        "compilerOptions": {
          "baseUrl": "src",
          "paths": {
            "@utils/*": ["utils/*"]
          }
        }
      }
      """
    And there is a file named "dist/cjs/index.js" with:
      """
      const foo = require('./foo');
      const bar = require("./bar");
      """
    And there is a file named "dist/cjs/foo.js" with:
      """
      module.exports = 'foo';
      """
    And there is a file named "dist/cjs/bar/index.js" with:
      """
      const baz = require('../baz');
      import('../lazy');
      
      // absolute path imports based on on tsconfig.json baseUrl
      require('baz');
      
      // alias imports based on tsconfig.json paths
      require('@utils');
      require('@utils/uniq');
      
      module.exports = 'bar';
      """
    And there is a file named "dist/cjs/baz.js" with:
      """
      module.exports = 'baz';
      """
    And there is a file named "dist/cjs/lazy.js" with:
      """
      module.exports = 'lazy';
      """
    And there is a file named "dist/cjs/utils/uniq.js" with:
      """
      module.exports = 'uniq';
      """
    And there is a file named "dist/cjs/utils/filter.js" with:
      """
      module.exports = 'filter';
      """
    And there is a file named "dist/cjs/utils/index.js" with:
      """
      module.exports.uniq = require('./uniq');
      module.exports.filter = require('./filter');
      """
    When the following command is executed:
      """
      reformat-specifiers \
        --absolute-package-dir $scenario_dir \
        --absolute-source-dir $scenario_dir/src \
        --absolute-output-dir $scenario_dir/dist/cjs \
        --output-format cjs
      """
    Then the JS content for "dist/cjs/index.js" should be:
      """
      const foo = require('./foo.js');
      const bar = require("./bar/index.js");
      """
    And the JS content for "dist/cjs/foo.js" should be:
      """
      module.exports = 'foo';
      """
    And the JS content for "dist/cjs/bar/index.js" should be:
      """
      const baz = require('../baz.js');
      import('../lazy.js');
            
      // absolute path imports based on on tsconfig.json baseUrl
      require('../baz.js');
      
      // alias imports based on tsconfig.json paths
      require('../utils/index.js');
      require('../utils/uniq.js');
      
      module.exports = 'bar';
      """
    And the JS content for "dist/cjs/lazy.js" should be:
      """
      module.exports = 'lazy';
      """
