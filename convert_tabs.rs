use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

fn convert_tabs(line: &str) -> String {
    line.replace("\t", "    ")
}

fn process_line(line: &str) -> String {
    if let Some(comment_start) = line.find("//") {
        let rest_of_line = &line[comment_start..];
        let modified_line = convert_tabs(rest_of_line);
        format!("{}{}", &line[..comment_start], modified_line)
    } else {
        line.to_string()
    }
}

fn process_file(file_path: &Path) -> io::Result<()> {
    let temp_file_path = file_path.with_extension("tmp");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut temp_file = File::create(&temp_file_path)?;

    for line in reader.lines() {
        let line = line?;
        let modified_line = process_line(&line);
        writeln!(temp_file, "{}", modified_line)?;
    }

    fs::rename(temp_file_path, file_path)?;
    Ok(())
}

fn process_directory(dir_path: &Path) -> io::Result<()> {
    let dir_entries = fs::read_dir(dir_path)?;

    for entry in dir_entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_path = entry.path();

        if file_type.is_file() && file_path.extension().map(|ext| ext == "rs").unwrap_or(false) {
            process_file(&file_path)?;
        } else if file_type.is_dir() {
            process_directory(&file_path)?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Check if the folder argument is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the folder path as an argument.");
        std::process::exit(1);
    }

    let folder = &args[1];
    let dir_path = Path::new(folder);

    process_directory(dir_path)?;

    Ok(())
}
