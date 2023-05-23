use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: String,
}
