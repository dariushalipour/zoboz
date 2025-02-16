use std::path::Path;

use oxc_module_resolver::create_oxc_module_resolver;

use super::value_objects::AbsolutePackageDir;

mod oxc_module_resolver;

pub struct SimpleModuleResolver {
    resolver: oxc_resolver::Resolver,
    absolute_base_url: Option<String>,
}

impl SimpleModuleResolver {
    pub fn new(package_dir: &AbsolutePackageDir) -> Self {
        let resolver = create_oxc_module_resolver();

        Self {
            resolver,
            absolute_base_url: Some(package_dir.value().to_string_lossy().to_string()),
        }
    }

    pub fn resolve_package_json_path(
        &self,
        dependent_dir: &Path,
        dependency_package_name: &str,
    ) -> Result<String, String> {
        let resolution = self
            .resolver
            .resolve(dependent_dir, &dependency_package_name);

        match resolution {
            Err(_) => return Err("RESOLVE_FAILED".to_string()),
            Ok(resolution) => {
                if resolution.package_json().is_none() {
                    return Err("PACKAGE_JSON_NOT_FOUND".to_string());
                }

                let package_json_path = resolution
                    .package_json()
                    .unwrap()
                    .realpath
                    .to_string_lossy()
                    .to_string();

                Ok(package_json_path)
            }
        }
    }

    pub fn resolve(
        &self,
        dependent_path: &Path,
        specifier: &str,
        is_trying_base_url_already: bool,
    ) -> Result<String, String> {
        let dependent_dirname: Option<&str> = dependent_path.parent().and_then(|p| p.to_str());

        if dependent_dirname.is_none() {
            return Err("DIRNAME_NOT_FOUND".to_string());
        }

        let dependent_dirname = dependent_dirname.unwrap();

        let source_dirname = if is_trying_base_url_already {
            self.absolute_base_url.as_ref().unwrap().as_str()
        } else {
            dependent_dirname
        };

        let specifier_for_resolver = if is_trying_base_url_already {
            &format!("./{}", specifier)
        } else {
            specifier
        };

        let resolved = self
            .resolver
            .resolve(source_dirname, specifier_for_resolver);

        if resolved.is_err() {
            if !is_trying_base_url_already
                && self.absolute_base_url.is_some()
                && !specifier.starts_with("./")
                && !specifier.starts_with("../")
            {
                return self.resolve(dependent_path, specifier, true);
            }

            return Err("RESOLVE_FAILED".to_string());
        }

        let resolved = resolved.unwrap();

        return Ok(resolved.path().to_string_lossy().to_string());
    }
}
