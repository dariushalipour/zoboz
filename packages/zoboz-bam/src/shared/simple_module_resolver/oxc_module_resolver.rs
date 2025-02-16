pub(super) fn create_oxc_module_resolver() -> oxc_resolver::Resolver {
    let mut resolve_options = oxc_resolver::ResolveOptions::default();
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
