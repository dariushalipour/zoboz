use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::shared::specifier_regex;
use crate::shared::ultimate_module_resolver::UltimateModuleResolver;
use crate::shared::value_objects::AbsoluteOutputDir;

pub(super) fn dump_modules_specifiers(
    module_resolver: &UltimateModuleResolver,
    absolute_output_dir: &AbsoluteOutputDir,
    dependent_path: &Path,
    resolved_absolute_specifiers: &mut HashSet<String>,
    unresolved_absolute_specifiers: &mut HashSet<String>,
    resolved_relative_specifiers: &mut HashSet<String>,
    unresolved_relative_specifiers: &mut HashSet<String>,
) {
    let file_content = fs::read_to_string(&dependent_path).unwrap();
    let mut resolution_results: Vec<(String, Result<String, String>)> = vec![];
    specifier_regex::RE_FROM.replace_all(&file_content, |caps: &regex::Captures| {
        let specifier = &caps[3];
        let resolution_result = module_resolver.resolve(dependent_path, specifier, false);
        resolution_results.push((specifier.to_string(), resolution_result));

        // this is just to ensure we're not changing the file content
        // I could not find a "find" method that gives me captures
        return caps[0].to_string();
    });

    specifier_regex::RE_REQUIRE_OR_IMPORT.replace_all(&file_content, |caps: &regex::Captures| {
        let specifier = &caps[3];
        let resolution_result = module_resolver.resolve(dependent_path, specifier, false);
        resolution_results.push((specifier.to_string(), resolution_result));

        // this is just to ensure we're not changing the file content
        // I could not find a "find" method that gives me captures
        return caps[0].to_string();
    });

    for (specifier, resolution_result) in resolution_results {
        if specifier.starts_with("./") || specifier.starts_with("../") {
            match resolution_result {
                Ok(resolved) => {
                    let out_dir = absolute_output_dir.value().to_string_lossy().to_string();
                    if !resolved.starts_with(&out_dir) {
                        if resolved.starts_with(&out_dir) {
                            let relative_path = Path::new(&resolved);
                            dump_modules_specifiers(
                                module_resolver,
                                absolute_output_dir,
                                &relative_path,
                                resolved_absolute_specifiers,
                                unresolved_absolute_specifiers,
                                resolved_relative_specifiers,
                                unresolved_relative_specifiers,
                            );
                        } else {
                            // this is a problem, a relative specifier pointing at somewhere outside of the output dir
                        }
                    }
                }
                Err(_) => {
                    // this *is* a problem, but we're not going to fix it here. (probably)
                    // here we only focus on package.json dependencies and peerDependencies, not relative failing paths
                }
            };
        } else {
            let root_specifier = get_root_specifier(&specifier);

            match resolution_result {
                Ok(_) => {
                    resolved_absolute_specifiers.insert(root_specifier.to_string());
                }
                Err(_) => {
                    unresolved_absolute_specifiers.insert(root_specifier.to_string());
                }
            }
        }
    }
}

fn get_root_specifier(specifier: &str) -> String {
    let keys: Vec<&str> = specifier.split('/').collect();

    if keys.len() == 1 {
        return specifier.to_string();
    }

    if keys[0].starts_with('@') {
        return keys[0..2].join("/");
    }

    return keys[0].to_string();
}
