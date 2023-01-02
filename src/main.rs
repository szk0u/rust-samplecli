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

    match cli.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }

    println!("Is verbosity specified?: {}", cli.verbose);
}
