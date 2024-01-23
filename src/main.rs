mod counter;
mod files;
mod printer;

use clap::Parser;
use self_update::cargo_crate_version;

/// line counter program to count lines of code by file extension
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// directory with files
    #[arg(short, long)]
    path: Option<String>,

    /// update example 'latest'
    #[arg(short, long)]
    update: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.update != None {
        if let Err(e) = update() {
            println!("[ERROR] {}", e);
            ::std::process::exit(1);
        }
    } else if args.path != None {
        let path = args.path.unwrap();
        let files_list = files::get_files(&path);
        let result = counter::run(files_list);
        printer::print_hash(result);
    } else {
        println!("enter the command --help or any other command to use the program!");
    }
}

fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("leonovk")
        .repo_name("line-counter")
        .bin_name("line-counter")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());
    Ok(())
}
