use std::{fs, io};

fn main() -> io::Result<()> {
    let dir = ".";

    let mut files = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| !name.starts_with('.'))
        })
        .filter_map(|path| {
            let path_str = path.strip_prefix(dir).unwrap_or(&path).to_str()?;
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
