use crate::shared::{tsconfig_reader::TsConfig, value_objects::AbsoluteOutputDir};

use super::value_objects::AbsoluteSourceDir;

pub(super) fn create_resolver(
    tsconfig: &TsConfig,
    src_dir: &AbsoluteSourceDir,
    out_dir: &AbsoluteOutputDir,
) -> oxc_resolver::Resolver {
    let aliases = get_aliases(tsconfig, src_dir, out_dir);

    let mut resolve_options = oxc_resolver::ResolveOptions::default();
    resolve_options.alias = aliases;
    resolve_options.extensions = vec![
        ".js".to_string(),
        ".jsx".to_string(),
        ".cjs".to_string(),
        ".mjs".to_string(),
        ".ts".to_string(),
        ".tsx".to_string(),
        ".json".to_string(),
        ".ts".to_string(),
        ".tsx".to_string(),
        ".d.ts".to_string(),
    ];
    resolve_options.extension_alias = vec![
        (
            ".js".to_string(),
            vec![
                ".js".to_string(),
                ".ts".to_string(),
                ".d.ts".to_string(),
                ".tsx".to_string(),
            ],
        ),
        (
            ".jsx".to_string(),
            vec![".jsx".to_string(), ".tsx".to_string()],
        ),
        (
            ".ts".to_string(),
            vec![".ts".to_string(), ".tsx".to_string(), ".js".to_string()],
        ),
        (
            ".tsx".to_string(),
            vec![".tsx".to_string(), ".jsx".to_string()],
        ),
    ];

    oxc_resolver::Resolver::new(resolve_options)
}

fn get_aliases(
    tsconfig: &TsConfig,
    src_dir: &AbsoluteSourceDir,
    out_dir: &AbsoluteOutputDir,
) -> Vec<(String, Vec<oxc_resolver::AliasValue>)> {
    if tsconfig.compiler_options.base_url.is_empty() {
        return vec![];
    }

    let base_url = std::path::Path::new(&tsconfig.compiler_options.base_url);

    let aliases: Vec<(String, Vec<oxc_resolver::AliasValue>)> = tsconfig
        .compiler_options
        .paths
        .iter()
        .map(|(k, v)| {
            let alias_key = k.trim_end_matches("/*").to_string();
            let alias_value = v
                .iter()
                .map(|x| {
                    let url = base_url.join(x.trim_end_matches("/*"));
                    url.to_string_lossy()
                        // turn src_dir based absolute paths to out_dir based absolute paths
                        .replace(
                            src_dir.value().to_str().unwrap(),
                            out_dir.value().to_str().unwrap(),
                        )
                        .into()
                })
                .collect::<Vec<_>>();
            (alias_key, alias_value)
        })
        .collect::<Vec<_>>();

    aliases
}
