// https://rust-cli.github.io/book/index.html

use structopt::StructOpt;
use std::io::{self,Write,BufReader, BufRead, BufWriter};
//Structopt bruker Clap, men har ekstra makroer for å lage clap code for struct definisjoner

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, // PathBuf is like a String but for file system paths that work cross-platform.
}

//Erstatt med io::Result<()>
// Solve skal kunne alle typer inputstreams som har lines() og read_line() (aka en BufRead)
// BufWrite finnes ikke, og det er ikke noe farlig
// input og output vil typisk være BufReader og BufWriter
fn solve(input: Box<dyn BufRead>, output: Box<dyn Write>) -> Result<(), Box<dyn std::error::Error>> {
    for line in input.lines().filter_map(|l| l.ok()) {
        writeln!(output, "{}", line)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    //FØRST NÅ er vi opptatt av effektivitet.

    let stdin = io::stdin(); //"Global stdin entity"
    let stdout = io::stdout(); //"Global stdout entity"

    //Locking er aldri nødvendig, for det gjøres implisitt hver gang vi aksesserer en impl Read/impl Write. Men det er raskere å låse én gang for hele prosessen

    let mut in_handle = stdin.lock(); // stdin implementerer BufRead allerede, så vi trenger ikke lage en BufReader!
    // If you have something that implements Read, you can use the BufReader type to turn it into a BufRead.
    // For example, File implements Read, but not BufRead. BufReader to the rescue!
    //ALTSÅ: Trenger ikke å lage bufreader fordi stdin allerede er implementerer BufRead

    let mut out_handle = io::BufWriter::new(stdout.lock()); //stdout har ikke buffered writing, så vi lager en BufWriter oppå for effektivitet

    solve(Box::new(in_handle), Box::new(out_handle));

    // let args = Cli::from_args();
    // println!("{:?}", args);
    Ok(())
}
