use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::api::{
    active, annihilate, core, create, expectation, general, matrix_element, operator_string,
    virtual_,
};
use crate::ast::{FermionOp, Index};
use crate::reference::{CasReference, Expectation, MatrixElement};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureSuite {
    pub version: u32,
    pub suite: String,
    pub cases: Vec<FixtureCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureCase {
    pub name: String,
    pub input: Vec<String>,
    pub expected: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixElementFixtureSuite {
    pub version: u32,
    pub suite: String,
    pub cases: Vec<MatrixElementFixtureCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixElementFixtureCase {
    pub name: String,
    pub left: Vec<String>,
    pub hamiltonian: Option<Vec<String>>,
    pub right: Vec<String>,
    pub expected: String,
}

pub fn build_reference_one_body_suite() -> FixtureSuite {
    FixtureSuite {
        version: 1,
        suite: "reference_one_body".to_string(),
        cases: vec![
            FixtureCase {
                name: "active_to_gamma".to_string(),
                input: vec![
                    "create(u:active)".to_string(),
                    "annihilate(v:active)".to_string(),
                ],
                expected: "gamma(u,v)".to_string(),
            },
            FixtureCase {
                name: "virtual_to_zero".to_string(),
                input: vec![
                    "create(a:virtual)".to_string(),
                    "annihilate(b:virtual)".to_string(),
                ],
                expected: "0".to_string(),
            },
            FixtureCase {
                name: "core_to_delta".to_string(),
                input: vec![
                    "create(i:core)".to_string(),
                    "annihilate(j:core)".to_string(),
                ],
                expected: "delta(i,j)".to_string(),
            },
            FixtureCase {
                name: "core_active_mixed_to_zero".to_string(),
                input: vec![
                    "create(i:core)".to_string(),
                    "annihilate(u:active)".to_string(),
                ],
                expected: "0".to_string(),
            },
            FixtureCase {
                name: "active_virtual_mixed_to_zero".to_string(),
                input: vec![
                    "create(u:active)".to_string(),
                    "annihilate(a:virtual)".to_string(),
                ],
                expected: "0".to_string(),
            },
        ],
    }
}

pub fn build_reference_two_body_suite() -> FixtureSuite {
    FixtureSuite {
        version: 1,
        suite: "reference_two_body".to_string(),
        cases: vec![
            FixtureCase {
                name: "active_to_gamma2".to_string(),
                input: vec![
                    "create(u:active)".to_string(),
                    "create(v:active)".to_string(),
                    "annihilate(w:active)".to_string(),
                    "annihilate(x:active)".to_string(),
                ],
                expected: "Gamma(u,v;w,x)".to_string(),
            },
            FixtureCase {
                name: "core_to_delta_delta".to_string(),
                input: vec![
                    "create(i:core)".to_string(),
                    "create(j:core)".to_string(),
                    "annihilate(k:core)".to_string(),
                    "annihilate(l:core)".to_string(),
                ],
                expected: "delta(i,l) * delta(j,k) - delta(i,k) * delta(j,l)".to_string(),
            },
            FixtureCase {
                name: "core_active_to_signed_delta_gamma".to_string(),
                input: vec![
                    "create(i:core)".to_string(),
                    "create(u:active)".to_string(),
                    "annihilate(j:core)".to_string(),
                    "annihilate(v:active)".to_string(),
                ],
                expected: "- delta(i,j) * gamma(u,v)".to_string(),
            },
            FixtureCase {
                name: "virtual_containing_two_body_to_zero".to_string(),
                input: vec![
                    "create(a:virtual)".to_string(),
                    "create(u:active)".to_string(),
                    "annihilate(v:active)".to_string(),
                    "annihilate(b:virtual)".to_string(),
                ],
                expected: "0".to_string(),
            },
        ],
    }
}

pub fn build_matrix_element_suite() -> MatrixElementFixtureSuite {
    MatrixElementFixtureSuite {
        version: 1,
        suite: "matrix_element".to_string(),
        cases: vec![
            MatrixElementFixtureCase {
                name: "canonical_overlap_like".to_string(),
                left: vec![
                    "create(u:active)".to_string(),
                    "annihilate(a:virtual)".to_string(),
                ],
                hamiltonian: None,
                right: vec![
                    "create(b:virtual)".to_string(),
                    "annihilate(v:active)".to_string(),
                ],
                expected: "delta(a,b) * gamma(u,v)".to_string(),
            },
            MatrixElementFixtureCase {
                name: "right_requires_normal_ordering".to_string(),
                left: vec![
                    "create(u:active)".to_string(),
                    "annihilate(a:virtual)".to_string(),
                ],
                hamiltonian: None,
                right: vec![
                    "annihilate(v:active)".to_string(),
                    "create(b:virtual)".to_string(),
                ],
                expected: "- delta(a,b) * gamma(u,v)".to_string(),
            },
            MatrixElementFixtureCase {
                name: "one_body_hamiltonian_in_middle".to_string(),
                left: vec![
                    "create(u:active)".to_string(),
                    "annihilate(a:virtual)".to_string(),
                ],
                hamiltonian: Some(vec![
                    "create(x:active)".to_string(),
                    "annihilate(y:active)".to_string(),
                ]),
                right: vec![
                    "create(b:virtual)".to_string(),
                    "annihilate(v:active)".to_string(),
                ],
                expected: "- delta(a,b) * Gamma(u,x;v,y)".to_string(),
            },
        ],
    }
}

pub fn fixture_path(suite_name: &str) -> PathBuf {
    Path::new("tests/fixtures").join(format!("{suite_name}.json"))
}

pub fn write_fixture_suite(suite: &FixtureSuite) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("tests/fixtures")?;
    let path = fixture_path(&suite.suite);
    let json = serde_json::to_string_pretty(suite)?;
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

pub fn load_fixture_suite(suite_name: &str) -> Result<FixtureSuite, Box<dyn std::error::Error>> {
    let path = fixture_path(suite_name);
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub fn write_matrix_element_fixture_suite(
    suite: &MatrixElementFixtureSuite,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("tests/fixtures")?;
    let path = fixture_path(&suite.suite);
    let json = serde_json::to_string_pretty(suite)?;
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

pub fn load_matrix_element_fixture_suite(
    suite_name: &str,
) -> Result<MatrixElementFixtureSuite, Box<dyn std::error::Error>> {
    let path = fixture_path(suite_name);
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub fn build_expectation_from_fixture_case(
    case: &FixtureCase,
    reference: CasReference,
) -> Result<Expectation, Box<dyn std::error::Error>> {
    let ops = parse_ops(&case.input)?;

    Ok(expectation(operator_string(ops), reference))
}

pub fn build_matrix_element_from_fixture_case(
    case: &MatrixElementFixtureCase,
    reference: CasReference,
) -> Result<MatrixElement, Box<dyn std::error::Error>> {
    let left = operator_string(parse_ops(&case.left)?);
    let hamiltonian = case
        .hamiltonian
        .as_ref()
        .map(|ops| parse_ops(ops).map(operator_string))
        .transpose()?;
    let right = operator_string(parse_ops(&case.right)?);

    Ok(matrix_element(left, hamiltonian, right, reference))
}

fn parse_ops(encoded_ops: &[String]) -> Result<Vec<FermionOp>, Box<dyn std::error::Error>> {
    encoded_ops
        .iter()
        .map(|encoded| parse_op(encoded))
        .collect()
}

fn parse_op(encoded: &str) -> Result<FermionOp, Box<dyn std::error::Error>> {
    let (kind, rest) = encoded
        .split_once('(')
        .ok_or_else(|| format!("invalid fixture op: {encoded}"))?;
    let payload = rest
        .strip_suffix(')')
        .ok_or_else(|| format!("invalid fixture op: {encoded}"))?;
    let (symbol, space) = payload
        .split_once(':')
        .ok_or_else(|| format!("invalid fixture op payload: {encoded}"))?;

    let index = parse_index(symbol, space)?;
    match kind {
        "create" => Ok(create(index)),
        "annihilate" => Ok(annihilate(index)),
        _ => Err(format!("unsupported fixture op kind: {kind}").into()),
    }
}

fn parse_index(symbol: &str, space: &str) -> Result<Index, Box<dyn std::error::Error>> {
    let index = match space {
        "active" => active(symbol),
        "core" => core(symbol),
        "virtual" => virtual_(symbol),
        "general" => general(symbol),
        _ => return Err(format!("unsupported fixture space: {space}").into()),
    };

    Ok(index)
}
