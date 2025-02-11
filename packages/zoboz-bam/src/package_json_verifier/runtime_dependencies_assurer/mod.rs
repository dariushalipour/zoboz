use crate::shared::json_editor::{Change, ChangeSet, ChangeType};

use crate::shared::package_json_reader::PackageJson;

pub(crate) fn run(package_json: &PackageJson, change_sets: &mut Vec<ChangeSet>) {
    // if package_json.type_field.is_some() {
    //     change_sets.push(ChangeSet {
    //         description: "Runtime dependency `package-not-available` is not listed in package.json field `dependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md".to_string(),
    //         changes: vec![],
    //     });
    // }
}
