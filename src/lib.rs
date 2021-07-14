mod fetch;
mod run;
mod test;
mod enums;

pub use enums::RunConfig;
pub use fetch::fetch_test_cases;
pub use run::{generate_out_file, generate_out_files, generate_ans_file, generate_ans_files};
pub use test::{evaluate_outputs, evaluate_output, generate_rust_tests};