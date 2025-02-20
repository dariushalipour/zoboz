use std::collections::HashSet;

use module_specifiers_dumper::dump_modules_specifiers;
use resolved_absolute_specifiers_handler::handle_resolved_absolute_specifiers;
use unresolved_absolute_specifiers_handler::handle_unresolved_absolute_specifiers;

use crate::shared::json_editor::ChangeSet;

use crate::shared::package_json_reader::PackageJson;
use crate::shared::simple_module_resolver::SimpleModuleResolver;
use crate::shared::value_objects::AbsolutePackageDir;

mod module_specifiers_dumper;
mod resolved_absolute_specifiers_handler;
mod unresolved_absolute_specifiers_handler;

pub(crate) fn run(
    module_resolver: &SimpleModuleResolver,
    absolute_package_dir: &AbsolutePackageDir,
    package_json: &PackageJson,
    change_sets: &mut Vec<ChangeSet>,
) {
    let entry_points = package_json.get_entry_points();

    let mut resolved_absolute_specifiers: HashSet<String> = HashSet::new();
    let mut unresolved_absolute_specifiers: HashSet<String> = HashSet::new();

    for entry_point in entry_points {
        dump_modules_specifiers(
            module_resolver,
            absolute_package_dir.value().join(entry_point).as_path(),
            &mut resolved_absolute_specifiers,
            &mut unresolved_absolute_specifiers,
        );
    }

    handle_resolved_absolute_specifiers(
        module_resolver,
        absolute_package_dir,
        package_json,
        change_sets,
        resolved_absolute_specifiers,
    );

    handle_unresolved_absolute_specifiers(change_sets, unresolved_absolute_specifiers);
}
