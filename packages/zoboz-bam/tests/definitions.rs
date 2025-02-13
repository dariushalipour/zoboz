use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use helpers::{
    get_dir_path, get_docstring, initiate_tempdir, read_file, reformat_json, write_file, TheWorld,
};
use std::fs;
use zoboz_bam::{handle_command, tokenize_input};

mod helpers;

fn main() {
    let features = [
        "tests/features/specifiers_reformatter/cjs_specifiers_reformatter.feature",
        "tests/features/specifiers_reformatter/esm_specifiers_reformatter.feature",
        "tests/features/specifiers_reformatter/dts_specifiers_reformatter.feature",
        "tests/features/package_json_verifier/type_field_remover.feature",
        "tests/features/package_json_verifier/runtime_dependencies_assurer.feature",
    ];

    for feature in features.iter() {
        futures::executor::block_on(TheWorld::run(feature));
    }
}

#[given(expr = "there is an npm package with:")]
fn there_is_a_npm_package_with(world: &mut TheWorld, step: &Step) {
    initiate_tempdir(world);
    a_json_file_named_with(world, step, "package.json".to_string());
}

#[given(expr = "there is a file named {string} with:")]
fn a_file_named_with(world: &mut TheWorld, step: &Step, file_name: String) {
    write_file(world, file_name.as_str(), &get_docstring(step));
}

#[given(expr = "there is a JSON file named {string} with:")]
fn a_json_file_named_with(world: &mut TheWorld, step: &Step, file_name: String) {
    let json_content = get_docstring(step);
    // If you find a better way to validate only, switch to that
    let _value: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    write_file(world, file_name.as_str(), &json_content);
}

#[then(expr = "the JS content for {string} should be:")]
fn the_js_content_for_should_be(world: &mut TheWorld, step: &Step, file_name: String) {
    assert_eq!(
        read_file(world, &file_name).trim(),
        get_docstring(step).trim()
    );
}

#[then(expr = "the DTS content for {string} should be:")]
fn the_dts_content_for_should_be(world: &mut TheWorld, step: &Step, file_name: String) {
    assert_eq!(
        read_file(world, &file_name).trim(),
        get_docstring(step).trim()
    );
}

#[then(expr = "the JSON content for {string} should be:")]
fn the_json_content_for_should_be(world: &mut TheWorld, step: &Step, file_name: String) {
    assert_eq!(
        reformat_json(read_file(world, &file_name).trim()),
        reformat_json(get_docstring(step).trim())
    );
}

#[given(expr = "the package has a directory named {string}")]
fn the_package_has_a_directory_named(world: &mut TheWorld, dir_path: String) {
    let path = get_dir_path(world).join(dir_path);
    fs::create_dir_all(path).unwrap();
}

#[then(expr = "the result is ok")]
fn the_result_is_ok(world: &mut TheWorld) {
    if let Some(result) = &world.command_result {
        assert_eq!(result.is_ok(), true);
    } else {
        panic!("The command result is not set yet.");
    }
}

#[then(expr = "the result is error and equals the following text:")]
fn the_result_is_error_and_equals_the_following_text(world: &mut TheWorld, step: &Step) {
    if let Some(result) = &world.command_result {
        if let Some(error) = result.clone().err() {
            let expected_error = get_docstring(step);
            assert_eq!(error.trim(), expected_error.trim());
            return;
        } else {
            panic!("The command result is not error.");
        }
    } else {
        panic!("The command result is not set yet.");
    }
}

#[when(expr = "the following command is executed:")]
fn the_following_command_is_executed(world: &mut TheWorld, step: &Step) {
    let command = get_docstring(step);
    let command = command.replace("$scenario_dir", get_dir_path(world).to_str().unwrap());
    world.command_result = Some(handle_command(&tokenize_input(&command)));
}
