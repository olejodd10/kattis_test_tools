// https://rust-cli.github.io/book/index.html

use structopt::StructOpt;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!
mod test_handler;
mod tests;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, // PathBuf is like a String but for file system paths that work cross-platform.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {


    // let args = Cli::from_args();
    // println!("{:?}", args);

    test_handler::fetch_test_cases("diplomacy")?;
    test_handler::run_test_cases()?;
    // test_handler::generate_tests()?;

    Ok(())
}
