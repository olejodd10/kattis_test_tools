mod fetch;
mod run;
mod test;
mod enums;

pub use enums::RunConfig;
pub use fetch::fetch_test_cases;
pub use run::run_test_cases;
pub use test::{evaluate_output, generate_rust_tests};