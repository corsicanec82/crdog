use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {}

pub fn run() {
    let cli = Cli::parse();

    dbg!(cli);
}
