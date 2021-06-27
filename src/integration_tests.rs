#![cfg(test)]

#[test]
fn sample_01() {
	assert!(kattis_test_tools::evaluate_output(std::path::Path::new("test_cases/sample-01.out")).unwrap());
}

#[test]
fn sample_02() {
	assert!(kattis_test_tools::evaluate_output(std::path::Path::new("test_cases/sample-02.out")).unwrap());
}

