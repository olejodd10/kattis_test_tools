// https://rust-cli.github.io/book/index.html

use structopt::StructOpt;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, // PathBuf is like a String but for file system paths that work cross-platform.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let args = Cli::from_args();
    // println!("{:?}", args);

    let kattis_problem = "diplomacy";
    let tests_file_path = std::path::Path::new("src");
    let test_cases_path = std::path::Path::new("test_cases");

    kattis_test_system::fetch_test_cases(kattis_problem)?;
    kattis_test_system::run_test_cases(test_cases_path)?;
    kattis_test_system::generate_tests_rs_file(tests_file_path, test_cases_path)?;

    Ok(())
}
