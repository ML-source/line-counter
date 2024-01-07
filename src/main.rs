#![warn(clippy::all, clippy::pedantic)]

mod files;
mod counter;
mod printer;

use clap::Parser;

/// line counter program to count lines of code by file extension
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// directory with files
    #[arg(short, long)]
    path: String
}

fn main() {
    let args = Args::parse();
    let files_list = files::get_files(&args.path);
    let result = counter::run(files_list);
    printer::print_hash(result);
}
