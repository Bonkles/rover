use clap::Parser;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli { 
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse(); 

    let file = File::open(&args.path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }

        line.clear();
    } 
    
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
