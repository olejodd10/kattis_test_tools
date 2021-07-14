// https://rust-cli.github.io/book/index.html

use structopt::StructOpt;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!
use kattis_test_tools::RunConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "Kattis Test Tools", about = "A tool for testing kattis problem solutions")]
enum Ktt {
    #[structopt(about="Initialize directory for test cases")]
    Init {
        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-dir", 
            help="Path to directory for test cases",
        )]
        test_cases_dir: std::path::PathBuf, 
    },

    #[structopt(about="Fetch sample test cases for Kattis problem")]
    Fetch {
        #[structopt(
            name="kattis-problem-name",
            help="Kattis problem to fetch sample test cases from",
        )]
        problem_name: String,

        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-dir", 
            help="Path to directory for test cases",
        )]
        test_cases_dir: std::path::PathBuf, 
    },

    #[structopt(about="Generate test case output using local environment")]
    Run {
        #[structopt(
            short,
            long,
            default_value="rust", 
            name="config", 
            help="Config to use to generate .out files from .in files in test_cases_dir",
        )]
        run_config: RunConfig,

        #[structopt(
            short,
            long,
            help="Generate .ans files instead of .out files",
        )]
        answer: bool,

        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-dir", 
            help="Path to directory for test cases",
        )]
        test_cases_dir: std::path::PathBuf, 
    
        #[structopt(
            parse(from_os_str), 
            short, 
            long, 
            name="test-case", 
            help="Limit run to single test case",
        )]
        test_case: Option<std::path::PathBuf>, 
    },

    #[structopt(about="Evaluate test case output")]
    Test {
        #[structopt(
            parse(from_os_str), 
            short="g",
            long,
            name="tests-file-dir",
            help="Generate integration_tests.rs",
        )]
        generate_rust_tests: Option<std::path::PathBuf>,

        #[structopt(
            short,
            long,
            help="Test existing .out files"
        )]
        existing: bool,

        #[structopt(
            short,
            long,
            default_value="rust", 
            name="config", 
            help="Config to use to generate .out files from .in files in test_cases_dir",
        )]
        run_config: RunConfig,

        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-dir", 
            help="Path to directory for test cases",
        )]
        test_cases_dir: std::path::PathBuf,
        
        #[structopt(
            parse(from_os_str), 
            short,
            long, 
            name="test-case", 
            help="Limit test to single test case",
        )]
        test_case: Option<std::path::PathBuf>, 
        
        // bench: bool,
    },

    //Submit!!!!
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Ktt::from_args();

    match args {
        Ktt::Init{test_cases_dir} => {
            std::fs::create_dir_all(&test_cases_dir)?;
            // TODO: Save this as an environment variable or something, so that it can be removed as an argument elsewhere
            // Same should be done for tests_file_dir 
        },
        Ktt::Fetch{problem_name, test_cases_dir} => {
            std::fs::create_dir_all(&test_cases_dir)?;
            kattis_test_tools::fetch_test_cases(&problem_name, &test_cases_dir)?;
        },
        Ktt::Run{run_config, answer, test_cases_dir, test_case} => {
            if answer {
                if let Some(test_case_path) = test_case {
                    let test_case_input_path = test_cases_dir.join(test_case_path.with_extension("in"));
                    kattis_test_tools::generate_ans_file(&run_config, &test_case_input_path)?;
                } else {
                    kattis_test_tools::generate_ans_files(&run_config, &test_cases_dir)?;
                }
            } else {
                if let Some(test_case_path) = test_case {
                    let test_case_input_path = test_cases_dir.join(test_case_path.with_extension("in"));
                    kattis_test_tools::generate_out_file(&run_config, &test_case_input_path)?;
                } else {
                    kattis_test_tools::generate_out_files(&run_config, &test_cases_dir)?;
                }
            }
        },
        Ktt::Test{generate_rust_tests, existing, run_config, test_cases_dir, test_case} => {
            if let Some(tests_file_dir) = generate_rust_tests {
                std::fs::create_dir_all(&tests_file_dir)?; 
                kattis_test_tools::generate_rust_tests(&tests_file_dir, &test_cases_dir)?;
            } 
            if let Some(test_case_path) = test_case {
                let test_case_input_path = test_cases_dir.join(test_case_path.with_extension("in"));
                if !existing {
                    kattis_test_tools::generate_out_file(&run_config, &test_case_input_path)?;
                }
                kattis_test_tools::evaluate_output(&test_case_input_path.with_extension("out"))?;
            } else {
                if !existing {
                    kattis_test_tools::generate_out_files(&run_config, &test_cases_dir)?;
                }
                kattis_test_tools::evaluate_outputs(&test_cases_dir)?;
            }
        },
    }

    Ok(())
}
