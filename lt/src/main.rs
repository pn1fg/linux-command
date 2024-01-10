use clap::Parser;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[clap(default_value = ".")]
    name: String,
    #[arg(short = 'a')]
    all: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut files = fs::read_dir(&args.name)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| !name.starts_with('.') || args.all)
        })
        .filter_map(|path| {
            let path_str = path.strip_prefix(&args.name).unwrap_or(&path).to_str()?;
            let mut display_str = path_str.to_string();
            if path.is_dir() {
                display_str.push('/');
            }
            Some(display_str)
        })
        .collect::<Vec<_>>();

    files.sort();

    let joined_files = files.join("\n");
    println!("{}", joined_files);

    Ok(())
}
