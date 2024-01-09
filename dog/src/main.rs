use clap::Parser;
use std::fs::File;
use std::io::Read;
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
    let mut file = File::open(path).expect("[Error] File not found.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("[Error] File not found.");
    print!("{}", data);
    ExitCode::SUCCESS
}
