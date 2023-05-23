use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Finds duplicate files super fast")]
pub struct Cli {
    #[arg(default_value = ".", help = "Root path for scanning")]
    pub path: String,

    #[arg(short, long, help = "Do not generate a file index in filesystem")]
    pub no_file_index: bool,

    #[arg(short, long, value_delimiter = ' ', num_args = 1.., help = "Search for duplicates of specific files", value_name = "FILE_PATH")]
    pub search: Vec<String>,

    #[arg(
        short,
        long,
        help = "Do not use the generated file index to search for duplicates"
    )]
    #[arg(short, long, requires = "search")]
    pub ignore_file_index: bool,
}
