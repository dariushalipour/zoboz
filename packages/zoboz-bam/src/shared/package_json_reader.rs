use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::value_objects::AbsolutePackageDir;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExportEntry {
    Single(String),
    Multi(HashMap<String, ExportEntry>),
}

#[derive(serde::Deserialize, Default)]
pub struct PackageJson {
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub main: Option<String>,
    pub module: Option<String>,
    pub types: Option<String>,
    pub exports: Option<HashMap<String, ExportEntry>>,
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

pub fn get_package_json_string_by_file_path(package_json_path: &str) -> String {
    let package_json_content = std::fs::read_to_string(package_json_path).unwrap_or_default();

    package_json_content
}

impl PackageJson {
    pub fn from_json_string(json_string: &str) -> PackageJson {
        serde_json::from_str(json_string).unwrap_or_default()
    }

    pub fn from_file_path(file_path: &str) -> PackageJson {
        let json_string = get_package_json_string_by_file_path(file_path);
        PackageJson::from_json_string(&json_string)
    }

    pub fn get_entry_points(&self) -> HashSet<String> {
        let mut entry_points = HashSet::new();

        if let Some(main) = &self.main {
            entry_points.insert(main.clone());
        }

        if let Some(module_) = &self.module {
            entry_points.insert(module_.clone());
        }

        if let Some(types) = &self.types {
            entry_points.insert(types.clone());
        }

        if let Some(exports) = &self.exports {
            for (_, entry) in exports.iter() {
                extract_export_entry_points(&mut entry_points, entry);
            }
        }

        entry_points
    }
}

fn extract_export_entry_points(entry_points: &mut HashSet<String>, entry: &ExportEntry) {
    let mut paths = HashSet::new();

    match entry {
        ExportEntry::Single(path) => {
            paths.insert(path.clone());
        }
        ExportEntry::Multi(sub_entries) => {
            for (_, sub_entry) in sub_entries.iter() {
                extract_export_entry_points(entry_points, sub_entry);
            }
        }
    };
}
