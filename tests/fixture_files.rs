use std::fs;

#[test]
fn committed_fixture_files_exist() {
    assert!(fs::metadata("tests/fixtures/reference_one_body.json").is_ok());
    assert!(fs::metadata("tests/fixtures/reference_two_body.json").is_ok());
    assert!(fs::metadata("tests/fixtures/matrix_element.json").is_ok());
}
