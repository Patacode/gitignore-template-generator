use std::fs;

use regex::Regex;
use test_bin::get_test_bin;

#[test]
fn it_outputs_correct_single_template_from_api() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    cmd.arg("rust");

    let output = cmd.output().expect("Failed to execute command");
    let expected = fs::read_to_string("tests/expected/rust_template.txt")
        .expect("Failed to read expected output file");
    let actual = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert_eq!(actual, expected);
}

#[test]
fn it_outputs_correct_combined_templates_from_api() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    cmd.args(["rust", "python"]);

    let output = cmd.output().expect("Failed to execute command");
    let expected =
        fs::read_to_string("tests/expected/rust_python_template.txt")
            .expect("Failed to read expected output file");
    let actual = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert_eq!(actual, expected);
}

#[test]
fn it_outputs_correct_error_when_no_pos_args_provided() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    let output = cmd.output().expect("Failed to execute command");
    let expected = fs::read_to_string("tests/expected/no_pos_args_error.txt")
        .expect("Failed to read expected output file");
    let actual = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(actual, expected);
}

#[test]
fn it_outputs_correct_error_when_commas_in_pos_args() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    cmd.args(["rust", "python,java"]);

    let output = cmd.output().expect("Failed to execute command");
    let expected =
        fs::read_to_string("tests/expected/comma_pos_args_error.txt")
            .expect("Failed to read expected output file");
    let actual = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(actual, expected);
}

#[test]
fn it_outputs_correct_error_when_template_not_found() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    cmd.args(["foo"]);

    let output = cmd.output().expect("Failed to execute command");
    let expected = "An error occurred during the API call: http status: 404\n";
    let actual = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(actual, expected);
}

#[test]
fn it_outputs_correct_infos_for_version_option() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    cmd.arg("-V");

    let output = cmd.output().expect("Failed to execute command");
    let pattern = r"^gitignore-template-generator [0-9]+\.[0-9]+\.[0-9]+\n$";
    let re = Regex::new(pattern).unwrap();
    let actual = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(re.is_match(&actual), "String did not match pattern: {}", pattern);
}
