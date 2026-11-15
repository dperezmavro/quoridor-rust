use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct QuoridorCli {
    /// Load game state from file
    #[arg(short, long, default_value = None)]
    pub load_file: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value = None)]
    pub save_file: Option<String>,
}

pub fn parse() -> QuoridorCli {
    QuoridorCli::parse()
}
