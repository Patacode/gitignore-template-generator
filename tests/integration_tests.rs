use test_bin::get_test_bin;

#[test]
fn it_works() {
    let mut cmd = get_test_bin("gitignore-template-generator");

    cmd.arg("rust");

    let output = cmd.output().expect("Failed to execute command");

    print!("{}", String::from_utf8_lossy(&output.stdout));

    assert!(output.status.success());
}
