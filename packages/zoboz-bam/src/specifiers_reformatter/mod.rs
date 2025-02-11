use cli_flags::get_params;
use file_updater::{update_cjs, update_dts, update_esm};
use file_walker::walk_files_recursively;
use specifiers_reformatter::SpecifiersReformatter;

use crate::shared::{
    tsconfig_reader,
    value_objects::{self, AbsoluteOutputDir, AbsolutePackageDir, AbsoluteSourceDir, OutputFormat},
};

mod cli_flags;
mod file_updater;
mod file_walker;
mod oxc_module_resolver;
mod specifiers_reformatter;
mod ultimate_module_resolver;

pub fn run_by_args(args: &[String]) -> Result<(), String> {
    let (output_format, absolute_package_dir, absolute_source_dir, absolute_output_dir) =
        get_params(args)?;

    run_by_params(
        &output_format,
        &absolute_package_dir,
        &absolute_source_dir,
        &absolute_output_dir,
    )
}

pub fn run_by_params(
    output_format: &str,
    absolute_package_dir: &str,
    absolute_source_dir: &str,
    absolute_output_dir: &str,
) -> Result<(), String> {
    let output_format = OutputFormat::new(output_format)?;
    let package_dir = AbsolutePackageDir::new(absolute_package_dir)?;
    let absolute_source_dir = AbsoluteSourceDir::new(absolute_source_dir)?;
    let absolute_output_dir = AbsoluteOutputDir::new(absolute_output_dir)?;

    let extensions: &[&str] = match output_format.value() {
        "esm" => &["js", "jsx", "mjs", "mjsx"],
        "cjs" => &["js", "jsx", "cjs", "cjsx"],
        "dts" => &["ts", "tsx"],
        _ => &["js", "jsx"],
    };

    if !absolute_source_dir.is_package_dir_child(&package_dir) {
        panic!("Source directory must be inside the package directory");
    }

    if !absolute_output_dir.is_package_dir_child(&package_dir) {
        panic!("Output directory must be inside the package directory");
    }

    let resolver = ultimate_module_resolver::UltimateModuleResolver::new(
        &package_dir,
        &absolute_source_dir,
        &absolute_output_dir,
    );

    let specifiers_reformatter = SpecifiersReformatter::new(resolver, &absolute_output_dir);

    walk_files_recursively(
        &absolute_output_dir.value(),
        extensions,
        &|file_path, file_content| match output_format.value() {
            "esm" => update_esm(&specifiers_reformatter, file_path, file_content),
            "cjs" => update_cjs(&specifiers_reformatter, file_path, file_content),
            "dts" => update_dts(&specifiers_reformatter, file_path, file_content),
            _ => panic!("Invalid format"),
        },
    )
    .unwrap();

    Ok(())
}
