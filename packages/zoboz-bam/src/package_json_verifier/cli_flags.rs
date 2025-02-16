use crate::shared::{cli_flags::get_absolute_package_dir, value_objects::AbsolutePackageDir};

pub(super) fn get_params(args: &[String]) -> Result<(AbsolutePackageDir, bool), String> {
    let absolute_package_dir = get_absolute_package_dir(&args)?;

    let can_update_package_json = get_can_update_package_json(args);

    Ok((absolute_package_dir, can_update_package_json))
}

fn get_can_update_package_json(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--can-update-package-json")
}
