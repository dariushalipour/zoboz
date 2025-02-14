use std::collections::HashSet;

use crate::shared::json_editor::ChangeSet;

pub fn handle_unresolved_absolute_specifiers(
    change_sets: &mut Vec<ChangeSet>,
    unresolved_absolute_specifiers: HashSet<String>,
) {
    for specifier in unresolved_absolute_specifiers {
        change_sets.push(ChangeSet {
          description: format!("Runtime dependency `{}` is not listed in package.json field `dependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md", specifier),
          changes: vec![],
      });
    }
}
