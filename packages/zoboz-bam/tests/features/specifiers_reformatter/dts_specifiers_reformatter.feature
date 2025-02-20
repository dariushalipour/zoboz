Feature: Declaration Module Specifier Formatter

  Scenario: Importing a declaration module with a specifier
    Given there is an npm package with:
      """
      {
        "name": "test",
        "version": "1.0.0",
        "types": "dist/dts/index.d.ts"
      }
      """
    And the package has a directory named "src"
    And the package has a directory named "dist/dts"
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
    And there is a file named "dist/dts/index.d.ts" with:
      """
      import foo from './foo';
      import fooPrime from './foo.js';
      import { bar } from "./bar";
      """
    And there is a file named "dist/dts/foo.d.ts" with:
      """
      export default 'foo';
      """
    And there is a file named "dist/dts/bar/index.d.ts" with:
      """
      import { createRequire } from 'module';
      // absolute path imports based on on tsconfig.json baseUrl
      import baz from 'baz';
      // alias imports based on tsconfig.json paths
      import utils from
        '@utils';
      
      import('../lazy');
      import
        ('../lazy');
      
      const require = createRequire(import.meta.url);
      
      // alias imports based on tsconfig.json paths
      require('@utils/uniq.js');
      require(
        '@utils/uniq'
      );
      
      export const bar = 'bar';
      """
    And there is a file named "dist/dts/baz.d.ts" with:
      """
      export default 'baz';
      """
    And there is a file named "dist/dts/lazy.d.ts" with:
      """
      export default 'lazy';
      """
    And there is a file named "dist/dts/utils/uniq.d.ts" with:
      """
      export default 'uniq';
      """
    And there is a file named "dist/dts/utils/filter.d.ts" with:
      """
      export default 'filter';
      """
    And there is a file named "dist/dts/utils/index.d.ts" with:
      """
      export { default as uniq } from './uniq';
      export { default as filter } from "./filter";
      """
    When the following command is executed:
      """
      reformat-specifiers \
        --absolute-package-dir $scenario_dir \
        --absolute-source-dir $scenario_dir/src \
        --absolute-output-dir $scenario_dir/dist/dts \
        --output-format dts
      """
    Then the DTS content for "dist/dts/index.d.ts" should be:
      """
      import foo from './foo.js';
      import fooPrime from './foo.js';
      import { bar } from "./bar/index.js";
      """
    And the DTS content for "dist/dts/foo.d.ts" should be:
      """
      export default 'foo';
      """
    And the DTS content for "dist/dts/bar/index.d.ts" should be:
      """
      import { createRequire } from 'module';
      // absolute path imports based on on tsconfig.json baseUrl
      import baz from '../baz.js';
      // alias imports based on tsconfig.json paths
      import utils from
        '../utils/index.js';
      
      import('../lazy.js');
      import
        ('../lazy.js');
      
      const require = createRequire(import.meta.url);
      
      // alias imports based on tsconfig.json paths
      require('../utils/uniq.js');
      require(
        '../utils/uniq.js'
      );
      
      export const bar = 'bar';
      """
    And the DTS content for "dist/dts/lazy.d.ts" should be:
      """
      export default 'lazy';
      """
