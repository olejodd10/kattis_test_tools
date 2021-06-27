use crate::enums::RunConfig;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::fs::File;

mod interact;

pub fn run_test_cases(run_config: &RunConfig, test_cases_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {

    // https://doc.rust-lang.org/std/path/struct.Path.html#method.read_dir
    for entry in test_cases_dir.read_dir()?.filter_map(|dir_entry| dir_entry.ok()) {
        if let Some(extension) = entry.path().extension() {
            if let Some("in") = extension.to_str() {
                let in_file = File::open(entry.path())?;
                let out_path = entry.path().with_extension("out");
                let out_file = File::create(out_path)?;
                let mut in_file_handle = BufReader::new(in_file); 
                let mut out_file_handle = BufWriter::new(out_file); 
                println!("Running {}", entry.path().file_stem().unwrap().to_str().unwrap());
                interact::interact_remote(&mut in_file_handle, &mut out_file_handle, run_config)?;
                println!("");
            }
        }
    }


    Ok(())
}

