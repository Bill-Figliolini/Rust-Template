use std::process::Command;

#[test]
fn binary_prints_greeting() {
    let output = Command::new(env!("CARGO_BIN_EXE_{{project-name}}"))
        .output()
        .expect("binary should run");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
}
