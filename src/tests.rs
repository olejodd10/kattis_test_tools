#![cfg(test)]

#[test]
fn sample_01() {
	assert!(super::test_handler::evaluate_output(std::path::Path::new("test_cases/sample-01.out")).unwrap());
}

#[test]
fn sample_02() {
	assert!(super::test_handler::evaluate_output(std::path::Path::new("test_cases/sample-02.out")).unwrap());
}

