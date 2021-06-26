
use std::io::{BufReader, BufWriter, Read};
use std::fs::File;
use std::path::{Path, PathBuf};

//Structopt bruker Clap, men har ekstra makroer for å lage clap code for struct definisjoner
mod solve;

pub fn run_test_cases() -> Result<(), Box<dyn std::error::Error>> {
    //FØRST NÅ er vi opptatt av effektivitet.
    // let stdin = io::stdin(); //"Global stdin entity"
    // let stdout = io::stdout(); //"Global stdout entity"
    // let mut stdin_handle = stdin.lock(); 
    // let mut stdout_handle = io::BufWriter::new(stdout.lock()); //stdout har ikke buffered writing, så vi lager en BufWriter oppå for effektivitet

    //stdin_handle og stdout_handle er nå buffered og immutably borrowed takket være locken


    // https://doc.rust-lang.org/std/path/struct.Path.html#method.read_dir

    let test_cases_path = Path::new("test_cases");

    for entry in test_cases_path.read_dir()?.filter_map(|dir_entry| dir_entry.ok()) {
        if let Some(extension) = entry.path().extension() {
            if let Some("in") = extension.to_str() {
                let in_file = File::open(entry.path())?;
                let out_path = entry.path().as_path().with_extension("out");
                let out_file = File::create(out_path)?;
                let mut in_file_handle = BufReader::new(in_file); 
                let mut out_file_handle = BufWriter::new(out_file); 
                solve::solve(&mut in_file_handle, &mut out_file_handle)?;
            }
        }
    }


    Ok(())
}

pub fn fetch_test_cases(problem_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use curl::easy::Easy;
    
    let test_cases_path = PathBuf::from("test_cases");

    let mut easy = Easy::new();
    easy.url(
        format!("https://open.kattis.com/problems/{}/file/statement/samples.zip", problem_name).as_str()
    )?;
    easy.write_function(move |data| { // zip_name moves
        zip_extract::extract(std::io::Cursor::new(data), &test_cases_path, true).expect("Unzipping error");
        Ok(data.len())
    })?;
    easy.perform()?;

    Ok(())
}

pub fn generate_tests() -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!()
}

fn evaluate_output(out_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    compare_file_tokens(out_path, &out_path.as_path().with_extension("ans"))
}

fn compare_file_tokens(file1: &PathBuf, file2: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let file1 = File::open(file1)?;
    let file2 = File::open(file2)?;
    let mut file1_handle = BufReader::new(file1); 
    let mut file2_handle = BufReader::new(file2); 

    //Jeg tror jeg lagde noe tokeniter greier for lenge siden... brukbart?
    let mut string1 = String::new();
    file1_handle.read_to_string(&mut string1)?;
    let tokens1: Vec<&str> = string1.split_whitespace().filter(|word| word.len() > 0).collect();

    let mut string2 = String::new();
    file2_handle.read_to_string(&mut string2)?;
    let tokens2: Vec<&str> = string2.split_whitespace().filter(|word| word.len() > 0).collect();

    Ok(tokens1==tokens2)
}