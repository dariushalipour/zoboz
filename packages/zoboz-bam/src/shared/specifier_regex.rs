use lazy_static::lazy_static;

lazy_static! {
    // NOTE: This regex won't work if 'require' is aliased or renamed using createRequire.
    pub static ref RE_REQUIRE_OR_IMPORT: regex::Regex =
        regex::Regex::new(r#"\b(require|import)(\s*\(\s*['"])(.+?)(['"]\s*\))"#).unwrap();
    pub static ref RE_FROM: regex::Regex =
        regex::Regex::new(r#"\b(from)(\s*['"])(.+?)(['"])"#).unwrap();
}
