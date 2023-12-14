use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String, // pattern to search for
    path: std::path::PathBuf, // file to read from
}

fn main() {
    // Get the command line arguments
    let args = Cli::parse();

    println!("Searching for {:?} in {:?}", args.pattern, args.path);
}
