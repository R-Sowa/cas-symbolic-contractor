use symbolic_mr::fixtures::{
    build_matrix_element_suite, build_reference_one_body_suite, build_reference_two_body_suite,
    write_fixture_suite, write_matrix_element_fixture_suite,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write_fixture_suite(&build_reference_one_body_suite())?;
    write_fixture_suite(&build_reference_two_body_suite())?;
    write_matrix_element_fixture_suite(&build_matrix_element_suite())?;
    Ok(())
}
