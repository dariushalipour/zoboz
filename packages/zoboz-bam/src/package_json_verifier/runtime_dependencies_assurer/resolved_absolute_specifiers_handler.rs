use std::collections::HashSet;

use crate::shared::{
    json_editor::{Change, ChangeSet, ChangeType},
    package_json_reader::PackageJson,
    simple_module_resolver::SimpleModuleResolver,
    value_objects::AbsolutePackageDir,
};

pub fn handle_resolved_absolute_specifiers(
    module_resolver: &SimpleModuleResolver,
    absolute_package_dir: &AbsolutePackageDir,
    package_json: &PackageJson,
    change_sets: &mut Vec<ChangeSet>,
    resolved_absolute_specifiers: HashSet<String>,
) {
    for specifier in resolved_absolute_specifiers {
        if !package_json.dependencies.contains_key(&specifier)
            && !package_json.peer_dependencies.contains_key(&specifier)
        {
            if package_json.dev_dependencies.contains_key(&specifier) {
                handle_misplaced_dependency(package_json, specifier, change_sets);
            } else {
                handle_unlisted_dependency(
                    module_resolver,
                    absolute_package_dir,
                    specifier,
                    change_sets,
                );
            }
        }
    }
}

fn handle_misplaced_dependency(
    package_json: &PackageJson,
    specifier: String,
    change_sets: &mut Vec<ChangeSet>,
) {
    let misplaced_dependency_message: String = format!(
      "Runtime dependency `{}` is listed in package.json field `devDependencies`. It should be moved to `dependencies` or get duplicated to `peerDependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md", specifier
  );

    let version = package_json
        .dev_dependencies
        .get(&specifier)
        .unwrap()
        .to_string();

    change_sets.push(ChangeSet {
        description: misplaced_dependency_message,
        changes: vec![
            Change {
                path: format!("dependencies.{}", specifier).to_string(),
                change_type: ChangeType::Add,
                value: Some(version),
            },
            Change {
                path: format!("devDependencies.{}", specifier).to_string(),
                change_type: ChangeType::Remove,
                value: None,
            },
        ],
    });
}

fn handle_unlisted_dependency(
    module_resolver: &SimpleModuleResolver,
    absolute_package_dir: &AbsolutePackageDir,
    specifier: String,
    change_sets: &mut Vec<ChangeSet>,
) {
    let unlisted_dependency_message = format!(
      "Runtime dependency `{}` is not listed in package.json field `dependencies` or `peerDependencies`. https://github.com/dariushalipour/zoboz/blob/main/packages/zoboz-bam/src/package_json_verifier/runtime_dependencies_assurer/README.md", specifier);

    let directory = absolute_package_dir.value();

    let package_json_path = module_resolver
        .resolve_package_json_path(directory, &specifier)
        .unwrap();

    let package_json = PackageJson::from_file_path(&package_json_path);

    match package_json.version {
        Some(version) => {
            change_sets.push(ChangeSet {
                description: unlisted_dependency_message,
                changes: vec![Change {
                    path: format!("dependencies.{}", specifier).to_string(),
                    change_type: ChangeType::Add,
                    value: Some(version),
                }],
            });
        }
        None => {
            change_sets.push(ChangeSet {
                description: unlisted_dependency_message,
                changes: vec![],
            });
        }
    }
}
