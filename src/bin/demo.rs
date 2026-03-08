#[path = "demo/demo_exact.rs"]
mod demo_exact;

use demo_exact::{
    ExactIndexAssignment, ExactState, ExactSystem, evaluate_tensor_term,
    exact_expectation_for_product,
};
use symbolic_mr::{
    CasReference, Expectation, MatrixElement, OperatorProduct, active, annihilate, core, create,
    expectation, matrix_element, simplify_tensor_form, trace_expectation, trace_matrix_element,
    virtual_,
};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    match args.as_slice() {
        [] => run_curated_demo(),
        [flag] if flag == "--trace" => run_curated_demo_trace(),
        [flag] if flag == "--trace-compact" => run_curated_demo_trace_compact(),
        [flag] if flag == "--cross-check" => run_curated_demo_cross_check(),
        [flag, count] if flag == "--sample" => {
            let count = count
                .parse::<usize>()
                .expect("--sample requires a positive integer");
            run_sampled_validation(count);
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  cargo run --bin demo");
            eprintln!("  cargo run --bin demo -- --trace");
            eprintln!("  cargo run --bin demo -- --trace-compact");
            eprintln!("  cargo run --bin demo -- --cross-check");
            eprintln!("  cargo run --bin demo -- --sample <count>");
            std::process::exit(2);
        }
    }
}

fn run_curated_demo() {
    print_header("One-body");
    demo_expectation(
        "core-core expectation",
        "⟨Ψ_CAS| a†(i) a(j) |Ψ_CAS⟩",
        expectation(
            create(core("i")) * annihilate(core("j")),
            CasReference::new(),
        ),
    );
    demo_expectation(
        "active-active expectation",
        "⟨Ψ_CAS| a†(u) a(v) |Ψ_CAS⟩",
        expectation(
            create(active("u")) * annihilate(active("v")),
            CasReference::new(),
        ),
    );
    demo_expectation(
        "mixed one-body expectation",
        "⟨Ψ_CAS| a†(u) a(a) |Ψ_CAS⟩",
        expectation(
            create(active("u")) * annihilate(virtual_("a")),
            CasReference::new(),
        ),
    );

    print_header("Two-body");
    demo_expectation(
        "non-normal active expectation",
        "⟨Ψ_CAS| a(x) a†(u) a†(v) a(w) |Ψ_CAS⟩",
        expectation(
            annihilate(active("x"))
                * create(active("u"))
                * create(active("v"))
                * annihilate(active("w")),
            CasReference::new(),
        ),
    );
    demo_expectation(
        "core-active mixed expectation",
        "⟨Ψ_CAS| a†(i) a†(u) a(j) a(v) |Ψ_CAS⟩",
        expectation(
            create(core("i"))
                * create(active("u"))
                * annihilate(core("j"))
                * annihilate(active("v")),
            CasReference::new(),
        ),
    );

    print_header("Matrix element");
    demo_matrix_element(
        "one-body Hamiltonian in middle",
        "⟨Ψ_CAS| a†(u)a(a) (a†(x)a(y)) a†(b)a(v) |Ψ_CAS⟩",
        matrix_element(
            create(active("u")) * annihilate(virtual_("a")),
            Some(create(active("x")) * annihilate(active("y"))),
            create(virtual_("b")) * annihilate(active("v")),
            CasReference::new(),
        ),
    );

    print_header("Higher-body");
    demo_expectation(
        "active higher-body expectation",
        "⟨Ψ_CAS| a†(u)a†(v)a†(w)a(x)a(y)a(z) |Ψ_CAS⟩",
        expectation(
            create(active("u"))
                * create(active("v"))
                * create(active("w"))
                * annihilate(active("x"))
                * annihilate(active("y"))
                * annihilate(active("z")),
            CasReference::new(),
        ),
    );
}

fn run_curated_demo_trace() {
    print_header("One-body");
    demo_expectation_trace(
        "core-core expectation",
        "⟨Ψ_CAS| a†(i) a(j) |Ψ_CAS⟩",
        expectation(
            create(core("i")) * annihilate(core("j")),
            CasReference::new(),
        ),
    );
    demo_expectation_trace(
        "active-active expectation",
        "⟨Ψ_CAS| a†(u) a(v) |Ψ_CAS⟩",
        expectation(
            create(active("u")) * annihilate(active("v")),
            CasReference::new(),
        ),
    );
    demo_expectation_trace(
        "mixed one-body expectation",
        "⟨Ψ_CAS| a†(u) a(a) |Ψ_CAS⟩",
        expectation(
            create(active("u")) * annihilate(virtual_("a")),
            CasReference::new(),
        ),
    );

    print_header("Two-body");
    demo_expectation_trace(
        "non-normal active expectation",
        "⟨Ψ_CAS| a(x) a†(u) a†(v) a(w) |Ψ_CAS⟩",
        expectation(
            annihilate(active("x"))
                * create(active("u"))
                * create(active("v"))
                * annihilate(active("w")),
            CasReference::new(),
        ),
    );
    demo_expectation_trace(
        "core-active mixed expectation",
        "⟨Ψ_CAS| a†(i) a†(u) a(j) a(v) |Ψ_CAS⟩",
        expectation(
            create(core("i"))
                * create(active("u"))
                * annihilate(core("j"))
                * annihilate(active("v")),
            CasReference::new(),
        ),
    );

    print_header("Matrix element");
    demo_matrix_element_trace(
        "one-body Hamiltonian in middle",
        "⟨Ψ_CAS| a†(u)a(a) (a†(x)a(y)) a†(b)a(v) |Ψ_CAS⟩",
        matrix_element(
            create(active("u")) * annihilate(virtual_("a")),
            Some(create(active("x")) * annihilate(active("y"))),
            create(virtual_("b")) * annihilate(active("v")),
            CasReference::new(),
        ),
    );

    print_header("Higher-body");
    demo_expectation_trace(
        "active higher-body expectation",
        "⟨Ψ_CAS| a†(u)a†(v)a†(w)a(x)a(y)a(z) |Ψ_CAS⟩",
        expectation(
            create(active("u"))
                * create(active("v"))
                * create(active("w"))
                * annihilate(active("x"))
                * annihilate(active("y"))
                * annihilate(active("z")),
            CasReference::new(),
        ),
    );
}

fn run_curated_demo_trace_compact() {
    demo_expectation_trace_compact(
        "one-body core-core",
        "⟨Ψ_CAS| a†(i) a(j) |Ψ_CAS⟩",
        expectation(
            create(core("i")) * annihilate(core("j")),
            CasReference::new(),
        ),
        false,
    );
    demo_expectation_trace_compact(
        "one-body active-active",
        "⟨Ψ_CAS| a†(u) a(v) |Ψ_CAS⟩",
        expectation(
            create(active("u")) * annihilate(active("v")),
            CasReference::new(),
        ),
        false,
    );
    demo_expectation_trace_compact(
        "one-body mixed zero",
        "⟨Ψ_CAS| a†(u) a(a) |Ψ_CAS⟩",
        expectation(
            create(active("u")) * annihilate(virtual_("a")),
            CasReference::new(),
        ),
        false,
    );
    demo_expectation_trace_compact(
        "non-normal active two-body",
        "⟨Ψ_CAS| a(x) a†(u) a†(v) a(w) |Ψ_CAS⟩",
        expectation(
            annihilate(active("x"))
                * create(active("u"))
                * create(active("v"))
                * annihilate(active("w")),
            CasReference::new(),
        ),
        true,
    );
    demo_matrix_element_trace_compact(
        "matrix element with one-body Hamiltonian",
        "⟨Ψ_CAS| a†(u)a(a) (a†(x)a(y)) a†(b)a(v) |Ψ_CAS⟩",
        matrix_element(
            create(active("u")) * annihilate(virtual_("a")),
            Some(create(active("x")) * annihilate(active("y"))),
            create(virtual_("b")) * annihilate(active("v")),
            CasReference::new(),
        ),
        false,
    );
    demo_expectation_trace_compact(
        "active higher-body expectation",
        "⟨Ψ_CAS| a†(u)a†(v)a†(w)a(x)a(y)a(z) |Ψ_CAS⟩",
        expectation(
            create(active("u"))
                * create(active("v"))
                * create(active("w"))
                * annihilate(active("x"))
                * annihilate(active("y"))
                * annihilate(active("z")),
            CasReference::new(),
        ),
        false,
    );
}

fn run_curated_demo_cross_check() {
    let system = ExactSystem::default();
    let state = ExactState::demo_reference(&system);

    for case in curated_cross_check_cases(&system) {
        let (ordered, output, exact_product) = case.trace_and_product();
        let exact = exact_expectation_for_product(&state, &case.assignment, &exact_product);
        let symbolic = evaluate_tensor_term(&state, &case.assignment, output.term());

        assert!(
            (exact - symbolic).abs() < 1.0e-9,
            "cross-check mismatch in {}: exact={exact}, symbolic={symbolic}",
            case.label
        );

        print_cross_check_field("case:", case.label);
        print_cross_check_field("in:", case.input);
        print_cross_check_field("ordered:", &format_cross_check_expression(&ordered));
        print_cross_check_field("out:", &format_cross_check_expression(&output.to_string()));
        print_cross_check_field(
            "check:",
            &format!(
                "exact = {}, symbolic = {}",
                format_numeric_value(exact),
                format_numeric_value(symbolic)
            ),
        );
        println!();
    }
}

fn run_sampled_validation(count: usize) {
    print_header("Sampled validation");

    let mut rng = Lcg::new(0x5EED_2026);
    for sample_index in 0..count {
        let case = sampled_case(rng.next_usize(7));
        println!("sample {}:", sample_index + 1);
        println!("category: {}", case.category);
        println!("input: {}", case.input);
        println!("output: {}", case.output());
        println!();
    }
}

fn print_header(title: &str) {
    println!("{title}");
    println!("{}", "-".repeat(title.len()));
}

fn demo_expectation(label: &str, input: &str, expr: Expectation) {
    let simplified = simplify_tensor_form(expr, Default::default())
        .expect("curated expectation example should simplify");
    println!("case: {label}");
    println!("input: {input}");
    println!("output: {simplified}");
    println!();
}

fn demo_matrix_element(label: &str, input: &str, expr: MatrixElement) {
    let simplified = simplify_tensor_form(expr, Default::default())
        .expect("curated matrix-element example should simplify");
    println!("case: {label}");
    println!("input: {input}");
    println!("output: {simplified}");
    println!();
}

fn demo_expectation_trace(label: &str, input: &str, expr: Expectation) {
    let trace = trace_expectation(expr, Default::default())
        .expect("curated expectation example should simplify");
    println!("case: {label}");
    println!("input: {input}");
    println!("normal ordered: {}", trace.normal_ordered());
    println!("reference reduction:");
    print_reference_reduction(trace.reduced_terms());
    println!("final output: {}", trace.final_form());
    println!();
}

fn demo_matrix_element_trace(label: &str, input: &str, expr: MatrixElement) {
    let trace = trace_matrix_element(expr, Default::default())
        .expect("curated matrix-element example should simplify");
    println!("case: {label}");
    println!("input: {input}");
    println!("normal ordered: {}", trace.normal_ordered());
    println!("reference reduction:");
    print_reference_reduction(trace.reduced_terms());
    println!("final output: {}", trace.final_form());
    println!();
}

fn demo_expectation_trace_compact(label: &str, input: &str, expr: Expectation, show_ordered: bool) {
    let trace = trace_expectation(expr, Default::default())
        .expect("curated expectation example should simplify");
    print_compact_trace_block(
        label,
        input,
        &trace.normal_ordered().to_string(),
        &trace.final_form().to_string(),
        show_ordered,
    );
}

fn demo_matrix_element_trace_compact(
    label: &str,
    input: &str,
    expr: MatrixElement,
    show_ordered: bool,
) {
    let trace = trace_matrix_element(expr, Default::default())
        .expect("curated matrix-element example should simplify");
    print_compact_trace_block(
        label,
        input,
        &trace.normal_ordered().to_string(),
        &trace.final_form().to_string(),
        show_ordered,
    );
}

fn print_compact_trace_block(
    label: &str,
    input: &str,
    ordered: &str,
    output: &str,
    show_ordered: bool,
) {
    println!("case: {label}");
    println!("in: {input}");
    if show_ordered {
        println!("ordered: {ordered}");
    }
    println!("out: {output}");
    println!();
}

fn print_reference_reduction(reduced_terms: &[symbolic_mr::SignedTensorTerm]) {
    if reduced_terms.is_empty() {
        println!("  0");
        return;
    }

    for term in reduced_terms {
        println!("  {term}");
    }
}

struct CrossCheckCase {
    label: &'static str,
    input: &'static str,
    expr: CrossCheckExpr,
    assignment: ExactIndexAssignment,
}

enum CrossCheckExpr {
    Expectation(Expectation),
    MatrixElement(MatrixElement),
}

impl CrossCheckCase {
    fn trace_and_product(&self) -> (String, symbolic_mr::SimplifiedTensorForm, OperatorProduct) {
        match &self.expr {
            CrossCheckExpr::Expectation(expr) => {
                let trace = trace_expectation(expr.clone(), Default::default())
                    .expect("cross-check expectation should simplify");
                (
                    trace.normal_ordered().to_string(),
                    trace.final_form().clone(),
                    expr.product().clone(),
                )
            }
            CrossCheckExpr::MatrixElement(expr) => {
                let trace = trace_matrix_element(expr.clone(), Default::default())
                    .expect("cross-check matrix element should simplify");
                (
                    trace.normal_ordered().to_string(),
                    trace.final_form().clone(),
                    matrix_element_product(expr),
                )
            }
        }
    }
}

fn curated_cross_check_cases(system: &ExactSystem) -> Vec<CrossCheckCase> {
    vec![
        CrossCheckCase {
            label: "diagonal active one-body",
            input: "⟨Ψ_CAS| a†(u) a(u) |Ψ_CAS⟩",
            expr: CrossCheckExpr::Expectation(expectation(
                create(active("u")) * annihilate(active("u")),
                CasReference::new(),
            )),
            assignment: ExactIndexAssignment::new().active("u", system.active_orbital(0)),
        },
        CrossCheckCase {
            label: "non-normal active two-body",
            input: "⟨Ψ_CAS| a(x) a†(u) a†(v) a(w) |Ψ_CAS⟩",
            expr: CrossCheckExpr::Expectation(expectation(
                annihilate(active("x"))
                    * create(active("u"))
                    * create(active("v"))
                    * annihilate(active("w")),
                CasReference::new(),
            )),
            assignment: ExactIndexAssignment::new()
                .active("x", system.active_orbital(0))
                .active("u", system.active_orbital(0))
                .active("v", system.active_orbital(1))
                .active("w", system.active_orbital(1)),
        },
        CrossCheckCase {
            label: "matrix element with one-body Hamiltonian",
            input: "⟨Ψ_CAS| a†(u)a(a) (a†(x)a(y)) a†(b)a(v) |Ψ_CAS⟩",
            expr: CrossCheckExpr::MatrixElement(matrix_element(
                create(active("u")) * annihilate(virtual_("a")),
                Some(create(active("x")) * annihilate(active("y"))),
                create(virtual_("b")) * annihilate(active("v")),
                CasReference::new(),
            )),
            assignment: ExactIndexAssignment::new()
                .active("u", system.active_orbital(0))
                .active("x", system.active_orbital(1))
                .active("y", system.active_orbital(0))
                .active("v", system.active_orbital(1))
                .virtual_("a", system.virtual_orbital(0))
                .virtual_("b", system.virtual_orbital(0)),
        },
        CrossCheckCase {
            label: "active higher-body",
            input: "⟨Ψ_CAS| a†(u)a†(v)a†(w)a(x)a(y)a(z) |Ψ_CAS⟩",
            expr: CrossCheckExpr::Expectation(expectation(
                create(active("u"))
                    * create(active("v"))
                    * create(active("w"))
                    * annihilate(active("x"))
                    * annihilate(active("y"))
                    * annihilate(active("z")),
                CasReference::new(),
            )),
            assignment: ExactIndexAssignment::new()
                .active("u", system.active_orbital(0))
                .active("v", system.active_orbital(1))
                .active("w", system.active_orbital(2))
                .active("x", system.active_orbital(0))
                .active("y", system.active_orbital(1))
                .active("z", system.active_orbital(2)),
        },
    ]
}

fn matrix_element_product(expr: &MatrixElement) -> OperatorProduct {
    let mut ops = expr.left().ops().to_vec();
    if let Some(hamiltonian) = expr.hamiltonian() {
        ops.extend_from_slice(hamiltonian.ops());
    }
    ops.extend_from_slice(expr.right().ops());
    OperatorProduct::new(ops)
}

fn format_numeric_value(value: f64) -> String {
    let sanitized = if value.abs() < 1.0e-12 { 0.0 } else { value };
    format!("{sanitized:.3}")
}

const CROSS_CHECK_LABEL_WIDTH: usize = 10;

fn print_cross_check_field(label: &str, content: &str) {
    let prefix = format!("{label:<CROSS_CHECK_LABEL_WIDTH$}");
    let continuation = " ".repeat(CROSS_CHECK_LABEL_WIDTH);
    let mut lines = content.lines();

    if let Some(first) = lines.next() {
        println!("{prefix}{first}");
        for line in lines {
            println!("{continuation}{line}");
        }
    } else {
        println!("{label}");
    }
}

fn format_cross_check_expression(content: &str) -> String {
    if content.starts_with("HigherRDM(") {
        return content.replace(", fragment=", ",\nfragment=");
    }

    split_sum_terms(content).join("\n")
}

fn split_sum_terms(content: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let bytes = content.as_bytes();
    let mut i = 0usize;

    while i + 2 < bytes.len() {
        if bytes[i] == b' '
            && (bytes[i + 1] == b'+' || bytes[i + 1] == b'-')
            && bytes[i + 2] == b' '
        {
            let term = content[start..i].trim();
            if !term.is_empty() {
                parts.push(term.to_string());
            }
            start = i + 1;
            i += 3;
        } else {
            i += 1;
        }
    }

    let tail = content[start..].trim();
    if !tail.is_empty() {
        parts.push(tail.to_string());
    }

    if parts.is_empty() {
        vec![content.to_string()]
    } else {
        parts
    }
}

struct SampleCase {
    category: &'static str,
    input: &'static str,
    expr: SampleExpr,
}

enum SampleExpr {
    Expectation(Expectation),
    MatrixElement(MatrixElement),
}

impl SampleCase {
    fn output(&self) -> String {
        match &self.expr {
            SampleExpr::Expectation(expr) => simplify_tensor_form(expr.clone(), Default::default())
                .expect("sampled expectation should simplify")
                .to_string(),
            SampleExpr::MatrixElement(expr) => {
                simplify_tensor_form(expr.clone(), Default::default())
                    .expect("sampled matrix element should simplify")
                    .to_string()
            }
        }
    }
}

fn sampled_case(choice: usize) -> SampleCase {
    match choice {
        0 => SampleCase {
            category: "one-body",
            input: "⟨Ψ_CAS| a†(i) a(j) |Ψ_CAS⟩",
            expr: SampleExpr::Expectation(expectation(
                create(core("i")) * annihilate(core("j")),
                CasReference::new(),
            )),
        },
        1 => SampleCase {
            category: "one-body",
            input: "⟨Ψ_CAS| a†(u) a(v) |Ψ_CAS⟩",
            expr: SampleExpr::Expectation(expectation(
                create(active("u")) * annihilate(active("v")),
                CasReference::new(),
            )),
        },
        2 => SampleCase {
            category: "one-body",
            input: "⟨Ψ_CAS| a†(u) a(a) |Ψ_CAS⟩",
            expr: SampleExpr::Expectation(expectation(
                create(active("u")) * annihilate(virtual_("a")),
                CasReference::new(),
            )),
        },
        3 => SampleCase {
            category: "two-body",
            input: "⟨Ψ_CAS| a†(i) a†(u) a(j) a(v) |Ψ_CAS⟩",
            expr: SampleExpr::Expectation(expectation(
                create(core("i"))
                    * create(active("u"))
                    * annihilate(core("j"))
                    * annihilate(active("v")),
                CasReference::new(),
            )),
        },
        4 => SampleCase {
            category: "two-body",
            input: "⟨Ψ_CAS| a†(i) a†(j) a(k) a(l) |Ψ_CAS⟩",
            expr: SampleExpr::Expectation(expectation(
                create(core("i"))
                    * create(core("j"))
                    * annihilate(core("k"))
                    * annihilate(core("l")),
                CasReference::new(),
            )),
        },
        5 => SampleCase {
            category: "matrix-element",
            input: "⟨Ψ_CAS| a†(u)a(a) a†(b)a(v) |Ψ_CAS⟩",
            expr: SampleExpr::MatrixElement(matrix_element(
                create(active("u")) * annihilate(virtual_("a")),
                None,
                create(virtual_("b")) * annihilate(active("v")),
                CasReference::new(),
            )),
        },
        _ => SampleCase {
            category: "matrix-element",
            input: "⟨Ψ_CAS| a†(u)a(a) (a†(x)a(y)) a†(b)a(v) |Ψ_CAS⟩",
            expr: SampleExpr::MatrixElement(matrix_element(
                create(active("u")) * annihilate(virtual_("a")),
                Some(create(active("x")) * annihilate(active("y"))),
                create(virtual_("b")) * annihilate(active("v")),
                CasReference::new(),
            )),
        },
    }
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_usize(&mut self, modulo: usize) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.state >> 32) as usize) % modulo
    }
}
