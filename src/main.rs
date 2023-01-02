use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser)]
#[command(name = "My RPN program")]
#[command(author = "Your name")]
#[command(version = "1.0.0")]
#[command(about = "Super awesome sample RPN calculator")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(value_name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let cli: Cli = Cli::parse();

    if let Some(path) = cli.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        println!("No file specified");
    }

    println!("Is verbosity specified?: {}", cli.verbose);
}
