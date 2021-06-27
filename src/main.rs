// https://rust-cli.github.io/book/index.html

use structopt::StructOpt;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!
use kattis_test_tools::enums::RunConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "Kattis Test Tools", about = "A tool for testing kattis problem solutions")]
enum Ktt {
    #[structopt(about="Initialize directory for test cases")]
    Init {
        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-path", 
            help="Path to directory for test cases",
        )]
        test_cases_path: std::path::PathBuf, 
    },

    #[structopt(about="Fetch sample test cases for Kattis problem")]
    Fetch {
        #[structopt(
            name="kattis-problem-name",
            help="Kattis problem to fetch sample test cases from.",
        )]
        problem_name: String,

        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-path", 
            help="Path to directory for test cases",
        )]
        test_cases_path: std::path::PathBuf, 
    },

    #[structopt(about="Generate test case output using local environment")]
    Run {
        #[structopt(
            default_value="rust", 
            name="config", 
            help="Config to use to generate .out files from .in files in test_cases_path.",
        )]
        run_config: RunConfig,

        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-path", 
            help="Path to directory for test cases",
        )]
        test_cases_path: std::path::PathBuf, 
    },

    #[structopt(about="Evaluate test case output")]
    Test {
        #[structopt(
            parse(from_os_str), 
            short="gr",
            long,
            name="tests-file-path",
            help="Generate integration_tests.rs",
        )]
        generate_rust_tests: Option<std::path::PathBuf>,

        #[structopt(
            short,
            long,
            help="Execute tests and print verdicts"
        )]
        execute: bool,

        #[structopt(
            parse(from_os_str), 
            long, 
            default_value = "test_cases", 
            name="test-cases-path", 
            help="Path to directory for test cases",
        )]
        test_cases_path: std::path::PathBuf, 
        
        // bench: bool,
    },

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Ktt::from_args();

    match args {
        Ktt::Init{test_cases_path} => {
            std::fs::create_dir_all(&test_cases_path)?;
            // TODO: Save this as an environment variable or something, so that it can be removed as an argument elsewhere
            // Same should be done for tests_file_path 
        },
        Ktt::Fetch{problem_name, test_cases_path} => {
            std::fs::create_dir_all(&test_cases_path)?;
            kattis_test_tools::fetch_test_cases(&problem_name, &test_cases_path)?;
        },
        Ktt::Run{run_config, test_cases_path} => {
            // std::fs::create_dir_all(&test_cases_path)?; //Should obviously exist
            kattis_test_tools::run_test_cases(&run_config, &test_cases_path)?;
        },
        Ktt::Test{generate_rust_tests, execute, test_cases_path} => {
            // std::fs::create_dir_all(&test_cases_path)?; //Should obviously exist
            if let Some(tests_file_path) = generate_rust_tests {
                kattis_test_tools::generate_rust_tests(&tests_file_path, &test_cases_path)?;
            }
            if execute {
                //ASSUMES RUST ENVIRONMENT
                std::process::Command::new("cargo test")
                    .spawn()
                    .expect("Failed to execute cargo test");
            }
        },
    }

    // let tests_file_path = std::path::Path::new("src");
    // let test_cases_path = std::path::Path::new("test_cases");
    // kattis_test_tools::run_test_cases(test_cases_path)?;
    // kattis_test_tools::generate_integration_tests(tests_file_path, test_cases_path)?;

    Ok(())
}
