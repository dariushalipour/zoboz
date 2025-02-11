mod cli_flags;
mod runtime_dependencies_assurer;
mod type_field_remover;

use std::{fs, string::String};

use cli_flags::get_params;

use crate::shared::{
    json_editor::{apply_change_sets, ChangeSet},
    package_json_reader::{get_package_json_object, get_package_json_string},
    value_objects::AbsolutePackageDir,
};

pub fn run_by_args(args: &[String]) -> Result<(), String> {
    let (absolute_package_dir, can_update_package_json) = get_params(args)?;

    run_by_params(&absolute_package_dir, can_update_package_json)
}

pub fn run_by_params(
    absolute_package_dir: &str,
    can_update_package_json: bool,
) -> Result<(), String> {
    let absolute_package_dir = AbsolutePackageDir::new(absolute_package_dir)?;

    let mut package_json_content = get_package_json_string(&absolute_package_dir);
    let package_json = get_package_json_object(&package_json_content);

    let mut change_sets: Vec<ChangeSet> = vec![];

    type_field_remover::run(&package_json, &mut change_sets);
    runtime_dependencies_assurer::run(&package_json, &mut change_sets);

    if change_sets.is_empty() {
        return Result::Ok(());
    }

    if !can_update_package_json {
        return Result::Err(changesets_to_err_string(change_sets));
    }

    let (automatic_changesets, manual_changesets): (Vec<_>, Vec<_>) = change_sets
        .into_iter()
        .partition(|change_set| change_set.changes.len() > 0);

    package_json_content = apply_change_sets(&package_json_content, automatic_changesets);

    fs::write(
        absolute_package_dir.value().join("package.json"),
        package_json_content,
    )
    .expect("Failed to write package.json");

    if manual_changesets.is_empty() {
        return Result::Ok(());
    }

    return Result::Err(changesets_to_err_string(manual_changesets));
}

fn changesets_to_err_string(change_sets: Vec<ChangeSet>) -> String {
    change_sets
        .iter()
        .map(|change_set| change_set.description.clone())
        .collect::<Vec<String>>()
        .join("\n")
}
