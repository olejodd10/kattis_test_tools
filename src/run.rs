use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::{Command,Stdio};
use std::fs::File;

fn pipe_through_program(program: &str, input: &mut dyn BufRead, output: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
    // https://doc.rust-lang.org/std/process/index.html

    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec)?;

    let mut program_tokens = program.split_whitespace();
    let command = program_tokens.next().ok_or("No program supplied")?;
    let args: Vec<&str> = program_tokens.collect();

    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute child");

    let mut input_pipe = child.stdin.take().expect("Failed to get stdin");

    std::thread::spawn(move || {
        input_pipe.write_all(&input_vec).expect("Failed to write to stdin");
    });

    let output_pipe = child
        .wait_with_output()
        .expect("Failed to wait on child");

    output.write_all(output_pipe.stdout.as_slice())?;

    Ok(())
}

pub fn run_input(program: &str, test_case_input: &Path, answer: bool) -> Result<(), Box<dyn std::error::Error>> {
    let extension = if answer { "ans" } else { "out" };
    let in_file = File::open(test_case_input)?;
    let out_path = test_case_input.with_extension(extension);
    let out_file = File::create(out_path)?;
    let mut in_file_handle = BufReader::new(in_file);
    let mut out_file_handle = BufWriter::new(out_file);
    println!("Running {}", test_case_input.file_stem().unwrap().to_str().unwrap());
    pipe_through_program(program, &mut in_file_handle, &mut out_file_handle)?;
    println!("");
    Ok(())
}

pub fn run_inputs(program: &str, test_cases_dir: &Path, answer: bool) -> Result<(), Box<dyn std::error::Error>> {

    // https://doc.rust-lang.org/std/path/struct.Path.html#method.read_dir
    for entry in test_cases_dir.read_dir()?.filter_map(|dir_entry| dir_entry.ok()) {
        if let Some(extension) = entry.path().extension() {
            if let Some("in") = extension.to_str() {
                run_input(program, &entry.path(), answer)?;
            }
        }
    }

    Ok(())
}
