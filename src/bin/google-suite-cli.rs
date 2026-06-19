use clap::Parser;

#[derive(Parser)]
#[command(name = "gsuite", version = "0.1.0")]
struct Cli {
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Cli::parse();
    println!("Executing command: {}", args.command);
}
