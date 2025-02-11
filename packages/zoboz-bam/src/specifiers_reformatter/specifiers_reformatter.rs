use std::path::Path;

use crate::shared::{
    ultimate_module_resolver::UltimateModuleResolver, utils, value_objects::AbsoluteOutputDir,
};

pub(super) struct SpecifiersReformatter {
    ultimate_module_resolver: UltimateModuleResolver,
    out_dir: String,
}

impl SpecifiersReformatter {
    pub(super) fn new(
        ultimate_module_resolver: UltimateModuleResolver,
        out_dir: &AbsoluteOutputDir,
    ) -> Self {
        Self {
            ultimate_module_resolver,
            out_dir: out_dir.value().to_string_lossy().to_string(),
        }
    }

    pub(super) fn format(&self, dependent_path: &Path, specifier: &str) -> String {
        match self
            .ultimate_module_resolver
            .resolve(dependent_path, specifier, false)
        {
            Ok(resolved_path) => {
                if !resolved_path.starts_with(self.out_dir.as_str()) {
                    return specifier.to_string();
                }

                let relative_path =
                    utils::relative(dependent_path.parent().unwrap(), resolved_path);

                let relative_path = utils::ensure_relative_prefix(relative_path);

                if relative_path.ends_with(".d.ts") {
                    relative_path.replace(".d.ts", ".js")
                } else {
                    relative_path
                }
            }
            Err(_) => return specifier.to_string(),
        }
    }
}
