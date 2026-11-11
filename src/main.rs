// https://rust-cli.github.io/book/index.html

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Kattis Test Tools", about = "A tool for testing kattis problem solutions")]
enum Ktt {
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

    #[structopt(about="Run program with inputs")]
    Run {
        #[structopt(
            short,
            long,
            default_value = "cargo run",
            name="program",
            help="Program to run to generate outputs",
        )]
        program: String,

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

    #[structopt(about="Run program with inputs and verify output")]
    Test {
        #[structopt(
            short,
            long,
            default_value = "cargo run",
            name="program",
            help="Program to run to generate outputs",
        )]
        program: String,

        #[structopt(
            short,
            long,
            help="Test existing .out files"
        )]
        existing: bool,

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
        Ktt::Fetch{problem_name, test_cases_dir} => {
            std::fs::create_dir_all(&test_cases_dir)?;
            kattis_test_tools::fetch_test_cases(&problem_name, &test_cases_dir)?;
        },
        Ktt::Run{program, answer, test_cases_dir, test_case} => {
            if answer {
                if let Some(test_case_path) = test_case {
                    let test_case_input_path = test_cases_dir.join(test_case_path.with_extension("in"));
                    kattis_test_tools::run_input(&program, &test_case_input_path, true)?;
                } else {
                    kattis_test_tools::run_inputs(&program, &test_cases_dir, true)?;
                }
            } else {
                if let Some(test_case_path) = test_case {
                    let test_case_input_path = test_cases_dir.join(test_case_path.with_extension("in"));
                    kattis_test_tools::run_input(&program, &test_case_input_path, false)?;
                } else {
                    kattis_test_tools::run_inputs(&program, &test_cases_dir, false)?;
                }
            }
        },
        Ktt::Test{program, existing, test_cases_dir, test_case} => {
            if let Some(test_case_path) = test_case {
                let test_case_input_path = test_cases_dir.join(test_case_path.with_extension("in"));
                if !existing {
                    kattis_test_tools::run_input(&program, &test_case_input_path, false)?;
                }
                kattis_test_tools::evaluate_output(&test_case_input_path.with_extension("out"))?;
            } else {
                if !existing {
                    kattis_test_tools::run_inputs(&program, &test_cases_dir, false)?;
                }
                kattis_test_tools::evaluate_outputs(&test_cases_dir)?;
            }
        },
    }

    Ok(())
}
