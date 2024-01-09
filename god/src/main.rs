use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: String,
}
fn main() -> ExitCode {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("[Error] No such file or directory.");
        return ExitCode::FAILURE;
    }
    let file = File::open(path).expect("[Error] File not found.");
    let data = BufReader::new(file);

    let mut lines: Vec<String> = data
        .lines()
        .map(|line| line.expect("[Error] Cannot read file"))
        .collect();

    lines.reverse();

    println!("{}", lines.join("\n"));

    ExitCode::SUCCESS
}
