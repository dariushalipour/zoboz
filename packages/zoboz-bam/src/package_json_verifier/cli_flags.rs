use crate::shared::{
    cli_flags::{get_absolute_output_dir, get_absolute_package_dir, get_absolute_source_dir},
    value_objects::{AbsoluteOutputDir, AbsolutePackageDir, AbsoluteSourceDir},
};

pub(super) fn get_params(
    args: &[String],
) -> Result<
    (
        AbsolutePackageDir,
        AbsoluteSourceDir,
        AbsoluteOutputDir,
        bool,
    ),
    String,
> {
    let absolute_package_dir = get_absolute_package_dir(&args)?;
    let absolute_source_dir = get_absolute_source_dir(&args)?;
    let absolute_output_dir = get_absolute_output_dir(&args)?;

    let can_update_package_json = get_can_update_package_json(args);

    Ok((
        absolute_package_dir,
        absolute_source_dir,
        absolute_output_dir,
        can_update_package_json,
    ))
}

fn get_can_update_package_json(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--can-update-package-json")
}
