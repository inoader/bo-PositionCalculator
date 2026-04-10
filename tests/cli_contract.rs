use std::process::Command;

fn bo() -> Command {
    Command::new(env!("CARGO_BIN_EXE_bo"))
}

#[test]
fn invalid_json_input_returns_non_zero_with_json_error() {
    let output = bo().args(["--json", "2.0", "101"]).output().unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(r#""ok":false"#));
    assert!(output.stderr.is_empty());
}

#[test]
fn invalid_text_input_returns_non_zero_on_stderr() {
    let output = bo().args(["2.0", "101"]).output().unwrap();

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("胜率必须在 0-100 之间"));
}

#[test]
fn valid_standard_input_returns_zero() {
    let output = bo().args(["2.0", "60"]).output().unwrap();

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("计算结果"));
    assert!(output.stderr.is_empty());
}
