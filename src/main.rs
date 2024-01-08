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
    #[arg(short, long, default_value_t = String::from("./"))]
    path: String,

    /// update
    #[arg(short, long, default_value_t = String::from("non"))]
    update: String,
}

fn main() {
    let args = Args::parse();

    if args.update != "non" {
        let _ = update();
        return;
    }

    let files_list = files::get_files(&args.path);
    let result = counter::run(files_list);
    printer::print_hash(result);
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
