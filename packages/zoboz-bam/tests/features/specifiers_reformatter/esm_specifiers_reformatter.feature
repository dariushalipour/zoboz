Feature: ES Module Specifier Formatter

  Scenario: Importing an ES Module with a specifier
    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "module": "dist/esm/index.js"
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/esm"
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
    And there is a file named "dist/esm/index.js" with:
      """
      import foo from './foo';
      import { bar } from "./bar";
      """
    And there is a file named "dist/esm/foo.js" with:
      """
      export default 'foo';
      """
    And there is a file named "dist/esm/bar/index.js" with:
      """
      import { createRequire } from 'module';
      // absolute path imports based on on tsconfig.json baseUrl
      import baz from 'baz';
      // alias imports based on tsconfig.json paths
      import utils from '@utils';
      
      import('../lazy');
      
      const require = createRequire(import.meta.url);
      
      // alias imports based on tsconfig.json paths
      require('@utils/uniq');
      
      export const bar = 'bar';
      """
    And there is a file named "dist/esm/baz.js" with:
      """
      export default 'baz';
      """
    And there is a file named "dist/esm/lazy.js" with:
      """
      export default 'lazy';
      """
    And there is a file named "dist/esm/utils/uniq.js" with:
      """
      export default 'uniq';
      """
    And there is a file named "dist/esm/utils/filter.js" with:
      """
      export default 'filter';
      """
    And there is a file named "dist/esm/utils/index.js" with:
      """
      export { default as uniq } from './uniq';
      export { default as filter } from "./filter";
      """
    When the following command is executed:
      """
      reformat-specifiers \
        --absolute-package-dir $scenario_dir \
        --absolute-source-dir $scenario_dir/src \
        --absolute-output-dir $scenario_dir/dist/esm \
        --output-format esm
      """
    Then the JS content for "dist/esm/index.js" should be:
      """
      import foo from './foo.js';
      import { bar } from "./bar/index.js";
      """
    And the JS content for "dist/esm/foo.js" should be:
      """
      export default 'foo';
      """
    And the JS content for "dist/esm/bar/index.js" should be:
      """
      import { createRequire } from 'module';
      // absolute path imports based on on tsconfig.json baseUrl
      import baz from '../baz.js';
      // alias imports based on tsconfig.json paths
      import utils from '../utils/index.js';
      
      import('../lazy.js');
      
      const require = createRequire(import.meta.url);
      
      // alias imports based on tsconfig.json paths
      require('../utils/uniq.js');
      
      export const bar = 'bar';
      """
    And the JS content for "dist/esm/lazy.js" should be:
      """
      export default 'lazy';
      """
