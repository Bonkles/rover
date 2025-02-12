use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli { 
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse(); 

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let lines = content.lines();
    for line in lines {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
