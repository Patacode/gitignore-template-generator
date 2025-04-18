use std::fs;

use test_bin::get_test_bin;

#[test]
fn it_outputs_correct_template_from_api() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    cmd.arg("rust");

    let output = cmd.output().expect("Failed to execute command");
    let expected = fs::read_to_string("tests/expected/rust_template.txt")
        .expect("Failed to read expected output file");
    let actual = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert_eq!(actual, expected);
}
