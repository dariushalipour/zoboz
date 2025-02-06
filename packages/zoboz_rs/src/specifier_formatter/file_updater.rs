use std::path::Path;

use lazy_static::lazy_static;

use super::specifier_formatter::SpecifierFormatter;

lazy_static! {
    // NOTE: This regex won't work if 'require' is aliased or renamed using createRequire.
    static ref RE_REQUIRE_OR_IMPORT: regex::Regex =
        regex::Regex::new(r#"\b(require|import)(\s*\(\s*['"])(.+?)(['"]\s*\))"#).unwrap();
    static ref RE_FROM: regex::Regex =
        regex::Regex::new(r#"\b(from)(\s*['"])(.+?)(['"])"#).unwrap();
}

pub(super) fn update_cjs(
    specifier_formatter: &SpecifierFormatter,
    file_path: &Path,
    file_content: &str,
) -> Option<String> {
    let new_content = update_requires_and_imports(specifier_formatter, file_path, file_content);
    if new_content != file_content {
        Some(new_content.into_owned())
    } else {
        None
    }
}

pub(super) fn update_esm(
    specifier_formatter: &SpecifierFormatter,
    file_path: &Path,
    file_content: &str,
) -> Option<String> {
    let new_content = update_requires_and_imports(specifier_formatter, file_path, file_content);
    let new_content = update_froms(specifier_formatter, file_path, &new_content);

    if new_content != file_content {
        Some(new_content.into_owned())
    } else {
        None
    }
}

pub(super) fn update_dts(
    specifier_formatter: &SpecifierFormatter,
    file_path: &Path,
    file_content: &str,
) -> Option<String> {
    let new_content = update_requires_and_imports(specifier_formatter, file_path, file_content);
    let new_content = update_froms(specifier_formatter, file_path, &new_content);

    if new_content != file_content {
        Some(new_content.into_owned())
    } else {
        None
    }
}

fn update_requires_and_imports<'a>(
    specifier_formatter: &'a SpecifierFormatter,
    file_path: &'a Path,
    file_content: &'a str,
) -> std::borrow::Cow<'a, str> {
    let new_content = RE_REQUIRE_OR_IMPORT.replace_all(&file_content, |caps: &regex::Captures| {
        format!(
            "{}{}{}{}",
            &caps[1],
            &caps[2],
            specifier_formatter.format(&file_path, &caps[3], false),
            &caps[4]
        )
    });

    new_content
}

fn update_froms<'a>(
    specifier_formatter: &'a SpecifierFormatter,
    file_path: &'a Path,
    file_content: &'a str,
) -> std::borrow::Cow<'a, str> {
    let new_content = RE_FROM.replace_all(&file_content, |caps: &regex::Captures| {
        format!(
            "{}{}{}{}",
            &caps[1],
            &caps[2],
            specifier_formatter.format(&file_path, &caps[3], false),
            &caps[4]
        )
    });

    new_content
}
