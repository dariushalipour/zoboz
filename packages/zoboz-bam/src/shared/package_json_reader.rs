use std::collections::HashSet;

use super::value_objects::AbsolutePackageDir;

#[derive(serde::Deserialize, Default)]
pub struct PackageJson {
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub main: Option<String>,
    pub module: Option<String>,
    pub types: Option<String>,
    pub exports: Option<serde_json::Value>,
    #[serde(default)]
    pub dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    pub dev_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "peerDependencies")]
    pub peer_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "optionalDependencies")]
    pub _optional_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "bundledDependencies")]
    pub _bundled_dependencies: Vec<String>,
    #[serde(default, rename = "bundleDependencies")]
    pub _bundle_dependencies: Vec<String>,
}

pub fn get_package_json_string(package_dir: &AbsolutePackageDir) -> String {
    let package_json_path = package_dir.value().join("package.json");
    get_package_json_string_by_file_path(package_json_path.to_str().unwrap())
}

pub fn get_package_json(package_dir: &AbsolutePackageDir) -> PackageJson {
    let package_json_content = get_package_json_string(package_dir);
    get_package_json_object(&package_json_content)
}

pub fn get_package_json_by_file_path(package_json_path: &str) -> PackageJson {
    let package_json_content = get_package_json_string_by_file_path(package_json_path);
    get_package_json_object(&package_json_content)
}

pub fn get_package_json_string_by_file_path(package_json_path: &str) -> String {
    let package_json_content = std::fs::read_to_string(package_json_path).unwrap_or_default();

    package_json_content
}

pub fn get_package_json_object(package_json_content: &str) -> PackageJson {
    let package_json: PackageJson = serde_json::from_str(package_json_content).unwrap_or_default();

    package_json
}

pub fn get_package_json_entry_points(package_json: &PackageJson) -> HashSet<String> {
    let mut entry_points = HashSet::new();

    if let Some(main) = &package_json.main {
        entry_points.insert(main.clone());
    }

    if let Some(module_) = &package_json.module {
        entry_points.insert(module_.clone());
    }

    if let Some(types) = &package_json.types {
        entry_points.insert(types.clone());
    }

    if let Some(exports) = &package_json.exports {
        if let serde_json::Value::Object(exports) = exports {
            for (_, value) in exports {
                if let serde_json::Value::String(value) = value {
                    entry_points.insert(value.clone());
                }
            }
        }
    }

    entry_points
}
