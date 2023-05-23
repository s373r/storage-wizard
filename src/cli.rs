use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: String,

    #[arg(short, long)]
    pub no_file_index: bool,

    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    pub search: Vec<String>,

    #[arg(short, long, requires = "search")]
    pub ignore_file_index: bool,
}
