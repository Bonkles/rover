use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli { 
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse(); 
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
