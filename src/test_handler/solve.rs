use std::io::{BufRead,Write};
use std::process::{Command,Stdio};

// Solve skal kunne alle typer inputstreams som har lines() og read_line() (aka en BufRead)
// BufWrite finnes ikke, og det er ikke noe farlig
// input og output vil typisk være BufReader og BufWriter
// &mut fordi read() og write() krever det
pub fn interact(input: &mut dyn BufRead, output: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
    
    for line in input.lines().map(|l| l.expect("Parse error")) {
        writeln!(output, "{}", line)?;
    }

    Ok(())
}

pub fn interact_remote(input: &mut dyn BufRead, output: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
    // https://doc.rust-lang.org/std/process/index.html

    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec)?;

    let mut child = Command::new("cargo run")
        .stdin(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute child");

    let mut input_pipe = child.stdin.take().expect("Failed to et stdin");

    std::thread::spawn(move || {
        input_pipe.write_all(&input_vec).expect("Failed to write to stdin");
    });

    let output_pipe = child
        .wait_with_output()
        .expect("Faile to wait on child");

    output.write_all(output_pipe.stdout.as_slice())?;

    Ok(())
}