use super::value_objects::AbsolutePackageDir;

#[derive(serde::Deserialize, Default)]
pub struct PackageJson {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub main: Option<String>,
    pub module: Option<String>,
    pub types: Option<String>,
    pub exports: Option<serde_json::Value>,
}

pub fn get_package_json_string(package_dir: &AbsolutePackageDir) -> String {
    let package_json_path = package_dir.value().join("package.json");
    let package_json_content = std::fs::read_to_string(package_json_path).unwrap_or_default();

    package_json_content
}

pub fn get_package_json(package_dir: &AbsolutePackageDir) -> PackageJson {
    let package_json_content = get_package_json_string(package_dir);
    get_package_json_object(&package_json_content)
}

pub fn get_package_json_object(package_json_content: &str) -> PackageJson {
    let package_json: PackageJson = serde_json::from_str(package_json_content).unwrap_or_default();

    package_json
}
