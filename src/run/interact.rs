use std::io::{BufRead,Write};
use std::process::{Command,Stdio};

use super::RunConfig;

pub fn interact_remote(input: &mut dyn BufRead, output: &mut dyn Write, run_config: &RunConfig) -> Result<(), Box<dyn std::error::Error>> {
    // https://doc.rust-lang.org/std/process/index.html

    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec)?;
    
    let mut child = match run_config {
        RunConfig::Rust => {
            Command::new("cargo")
                .arg("run")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute child")
        },
        RunConfig::Haskell => {
            Command::new("ghc")
                .arg("main.hs")
                .output() //Status can also be used
                .expect("Failed to compile main.hs");
            Command::new("main.exe")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute child")            
        },
        RunConfig::Python => {
            Command::new("python")
                .arg("main.py")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute child")
        },
    };

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