use std::process::Command;

#[test]
fn demo_cli_prints_curated_sections() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "demo"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("demo binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("One-body"));
    assert!(stdout.contains("Two-body"));
    assert!(stdout.contains("Matrix element"));
}

#[test]
fn demo_cli_trace_prints_stage_sections() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "demo", "--", "--trace"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("demo binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("input:"));
    assert!(stdout.contains("normal ordered:"));
    assert!(stdout.contains("reference reduction:"));
    assert!(stdout.contains("final output:"));
}

#[test]
fn demo_cli_trace_shows_non_normal_reduction_example() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "demo", "--", "--trace"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("demo binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("case: non-normal active expectation"));
    assert!(stdout.contains(
        "normal ordered: delta(x,u) a†(v) a(w) - delta(x,v) a†(u) a(w) - a†(u) a†(v) a(w) a(x)"
    ));
    assert!(stdout.contains("  delta(x,u) * gamma(v,w)"));
    assert!(stdout.contains("  - delta(x,v) * gamma(u,w)"));
    assert!(stdout.contains("  - Gamma(u,v;w,x)"));
    assert!(stdout.contains(
        "final output: delta(x,u) * gamma(v,w) - delta(x,v) * gamma(u,w) - Gamma(u,v;w,x)"
    ));
}

#[test]
fn demo_cli_trace_compact_prints_curated_runtime_blocks() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "demo", "--", "--trace-compact"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("demo binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("case: one-body core-core"));
    assert!(stdout.contains("case: matrix element with one-body Hamiltonian"));
    assert!(stdout.contains("in: ⟨Ψ_CAS| a†(i) a(j) |Ψ_CAS⟩"));
    assert!(stdout.contains("out:"));
    assert!(!stdout.contains("reference reduction:"));
}

#[test]
fn demo_cli_trace_compact_shows_higher_rdm_case() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "demo", "--", "--trace-compact"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("demo binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("case: active higher-body expectation"));
    assert!(stdout.contains("HigherRDM(order=3"));
}

#[test]
fn demo_cli_cross_check_prints_exact_comparison_blocks() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "demo", "--", "--cross-check"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("demo binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains(concat!(
        "case:     diagonal active one-body\n",
        "in:       ⟨Ψ_CAS| a†(u) a(u) |Ψ_CAS⟩\n",
        "ordered:  a†(u) a(u)\n",
        "out:      gamma(u,u)\n",
        "check:    exact = 0.500, symbolic = 0.500\n"
    )));
    assert!(stdout.contains(concat!(
        "case:     non-normal active two-body\n",
        "in:       ⟨Ψ_CAS| a(x) a†(u) a†(v) a(w) |Ψ_CAS⟩\n",
        "ordered:  delta(x,u) a†(v) a(w)\n",
        "          - delta(x,v) a†(u) a(w)\n",
        "          - a†(u) a†(v) a(w) a(x)\n",
        "out:      delta(x,u) * gamma(v,w)\n",
        "          - delta(x,v) * gamma(u,w)\n",
        "          - Gamma(u,v;w,x)\n",
        "check:    exact = 0.250, symbolic = 0.250\n"
    )));
    assert!(stdout.contains(concat!(
        "case:     matrix element with one-body Hamiltonian\n",
        "in:       ⟨Ψ_CAS| a†(u)a(a) (a†(x)a(y)) a†(b)a(v) |Ψ_CAS⟩\n",
        "ordered:  delta(a,x) delta(y,b) a†(u) a(v)\n",
        "          - delta(a,b) a†(u) a†(x) a(v) a(y)\n",
        "          - delta(a,x) a†(b) a†(u) a(v) a(y)\n",
        "          - delta(y,b) a†(u) a†(x) a(a) a(v)\n",
        "          + a†(b) a†(u) a†(x) a(a) a(v) a(y)\n",
        "out:      - delta(a,b) * Gamma(u,x;v,y)\n",
        "check:    exact = -0.250, symbolic = -0.250\n"
    )));
    assert!(stdout.contains(concat!(
        "case:     active higher-body\n",
        "in:       ⟨Ψ_CAS| a†(u)a†(v)a†(w)a(x)a(y)a(z) |Ψ_CAS⟩\n",
        "ordered:  a†(u) a†(v) a†(w) a(x) a(y) a(z)\n",
        "out:      HigherRDM(order=3,\n",
        "          fragment=a†(u) a†(v) a†(w) a(x) a(y) a(z))\n",
        "check:    exact = -0.125, symbolic = -0.125\n"
    )));
}
