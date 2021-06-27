use std::io::{Write, BufReader, BufWriter, Read};
use std::path::Path;
use std::fs::File;

pub fn generate_rust_tests(tests_file_dir: &Path, test_cases_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let tests_file = File::create(tests_file_dir.join("integration_tests.rs"))?;
    let mut tests_file_handle = BufWriter::new(tests_file); 
    writeln!(tests_file_handle, "#![cfg(test)]\n")?;

    for entry in test_cases_dir.read_dir()?.filter_map(|dir_entry| dir_entry.ok()) {
        if let Some(extension) = entry.path().extension() {
            if let Some("ans") = extension.to_str() { //Lager test for tilsvarende "out"-fil hver gang den finner en "ans"-fil
                let test_name = String::from(entry.path().file_stem().unwrap().to_str().unwrap())
                .replace('-', "_");
                let out_path = String::from(entry.path().with_extension("out").to_str().unwrap())
                .replace('\\', "/");

                writeln!(tests_file_handle, "{}",
                    format!("#[test]\nfn {}() {{\n\tassert!(kattis_test_tools::evaluate_output(std::path::Path::new(\"{}\")).unwrap());\n}}\n", 
                    test_name, 
                    out_path)
                )?;
            }
        }
    }

    Ok(())
}


fn compare_file_tokens(file1: &Path, file2: &Path) -> Result<bool, Box<dyn std::error::Error>> {
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

pub fn evaluate_output(out_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    compare_file_tokens(out_path, &out_path.with_extension("ans"))
}

pub fn evaluate_outputs(test_cases_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in test_cases_dir.read_dir()?.filter_map(|dir_entry| dir_entry.ok()) {
        if let Some(extension) = entry.path().extension() {
            if let Some("ans") = extension.to_str() { //Lager test for tilsvarende "out"-fil hver gang den finner en "ans"-fil
                let test_name = String::from(entry.path().file_stem().unwrap().to_str().unwrap());
                let out_path = entry.path().with_extension("out");

                if evaluate_output(&out_path)? {
                    println!("{} passed", test_name);
                } else {
                    println!("{} failed", test_name);
                }
            }
        }
    }

    Ok(())
}
