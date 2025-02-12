use super::value_objects::{AbsoluteOutputDir, AbsolutePackageDir, AbsoluteSourceDir};

pub fn get_absolute_package_dir(args: &[String]) -> Result<AbsolutePackageDir, String> {
    let absolute_package_dir = args.iter().position(|arg| arg == "--absolute-package-dir");
    let absolute_package_dir = match absolute_package_dir {
        Some(index) => args.get(index + 1),
        None => None,
    };

    match absolute_package_dir {
        Some(value) => {
            let absolute_package_dir = AbsolutePackageDir::new(value)?;
            Ok(absolute_package_dir)
        }
        None => Err(format!(
            "--absolute-package-dir not found; use --absolute-package-dir <path>"
        )),
    }
}

pub fn get_absolute_source_dir(args: &[String]) -> Result<AbsoluteSourceDir, String> {
    let absolute_source_dir = args.iter().position(|arg| arg == "--absolute-source-dir");
    let absolute_source_dir = match absolute_source_dir {
        Some(index) => args.get(index + 1),
        None => None,
    };

    match absolute_source_dir {
        Some(value) => {
            let absolute_source_dir = AbsoluteSourceDir::new(value)?;
            Ok(absolute_source_dir)
        }
        None => Err(format!(
            "--absolute-source-dir not found; use --absolute-source-dir <path>"
        )),
    }
}

pub fn get_absolute_output_dir(args: &[String]) -> Result<AbsoluteOutputDir, String> {
    let absolute_output_dir = args.iter().position(|arg| arg == "--absolute-output-dir");
    let absolute_output_dir = match absolute_output_dir {
        Some(index) => args.get(index + 1),
        None => None,
    };

    match absolute_output_dir {
        Some(value) => {
            let absolute_output_dir = AbsoluteOutputDir::new(value)?;
            Ok(absolute_output_dir)
        }
        None => Err(format!(
            "--absolute-output-dir not found; use --absolute-output-dir <path>"
        )),
    }
}
