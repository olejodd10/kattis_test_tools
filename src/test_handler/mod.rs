
use std::io::{self, BufReader, BufWriter};
use std::fs::File;

//Structopt bruker Clap, men har ekstra makroer for å lage clap code for struct definisjoner
mod solve;

pub fn run_test_cases() -> Result<(), Box<dyn std::error::Error>> {
    //FØRST NÅ er vi opptatt av effektivitet.
    // let stdin = io::stdin(); //"Global stdin entity"
    // let stdout = io::stdout(); //"Global stdout entity"
    // let mut stdin_handle = stdin.lock(); 
    // let mut stdout_handle = io::BufWriter::new(stdout.lock()); //stdout har ikke buffered writing, så vi lager en BufWriter oppå for effektivitet

    //stdin_handle og stdout_handle er nå buffered og immutably borrowed takket være locken

    let in_file = File::open("input.txt")?;
    let out_file = File::create("output.txt")?;
    let mut in_file_handle = BufReader::new(in_file); //BufReader implementerer BufRead
    let mut out_file_handle = BufWriter::new(out_file); //BufWriter implementerer Write (Altså er det egentlig ikke nødvendig å lage en out_file handle, fordi File implementerer Write)

    // solve::solve(&mut stdin_handle, &mut stdout_handle)?;
    // solve::solve(&mut in_file_handle, &mut stdout_handle)?;
    solve::solve(&mut in_file_handle, &mut out_file_handle)?;

    Ok(())
}

pub fn fetch_test_cases() -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!()
}

pub fn generate_comparison_tests() -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!()
}