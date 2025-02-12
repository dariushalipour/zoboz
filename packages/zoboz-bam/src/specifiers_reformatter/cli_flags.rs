use crate::shared::{
    cli_flags::{get_absolute_output_dir, get_absolute_package_dir, get_absolute_source_dir},
    value_objects::{AbsoluteOutputDir, AbsolutePackageDir, AbsoluteSourceDir, OutputFormat},
};

pub(super) fn get_params(
    args: &[String],
) -> Result<
    (
        OutputFormat,
        AbsolutePackageDir,
        AbsoluteSourceDir,
        AbsoluteOutputDir,
    ),
    String,
> {
    let output_format = get_output_format(&args)?;

    let absolute_package_dir = get_absolute_package_dir(&args)?;
    let absolute_source_dir = get_absolute_source_dir(&args)?;
    let absolute_output_dir = get_absolute_output_dir(&args)?;

    Ok((
        output_format,
        absolute_package_dir,
        absolute_source_dir,
        absolute_output_dir,
    ))
}

fn get_output_format(args: &[String]) -> Result<OutputFormat, String> {
    let output_format = args.iter().position(|arg| arg == "--output-format");
    let output_format = match output_format {
        Some(index) => args.get(index + 1),
        None => None,
    };

    match output_format {
        Some(value) => {
            let output_format = OutputFormat::new(value)?;
            Ok(output_format)
        }
        None => Err(format!(
            "OutputFormat not found; use --output-format <cjs|esm|dts>"
        )),
    }
}
