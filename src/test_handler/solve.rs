use std::io::{BufRead,Write};

// Solve skal kunne alle typer inputstreams som har lines() og read_line() (aka en BufRead)
// BufWrite finnes ikke, og det er ikke noe farlig
// input og output vil typisk være BufReader og BufWriter
// &mut fordi read() og write() krever det
pub fn solve(input: &mut dyn BufRead, output: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
    
    for line in input.lines().map(|l| l.expect("Parse error")) {
        writeln!(output, "{}", line)?;
    }

    Ok(())
}