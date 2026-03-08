use std::process::Command;

#[test]
fn demo_cli_prints_sampled_validation_cases() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "demo", "--", "--sample", "5"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("demo binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("Sampled validation"));
    assert!(stdout.contains("sample 1"));
    assert!(stdout.contains("output:"));
}
