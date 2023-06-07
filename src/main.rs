use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use chrono::Local;

fn main() -> io::Result<()> {
    let user_dir = match env::var("USERPROFILE") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            return Err(Error::new(
                ErrorKind::Other,
                "Failed to retrieve user directory path.",
            ));
        }
    };

    let source_dir = user_dir
        .join("AppData")
        .join("LocalLow")
        .join("Southpaw Games")
        .join("Skul");

    if !source_dir.exists() {
        return Err(Error::new(
            ErrorKind::Other,
            "Source directory does not exist.",
        ));
    }

    let current_time = Local::now().format("%Y%m%d%H%M%S").to_string();
    println!("Current time: {}", current_time);

    let destination_dir = env::current_dir()?.join(format!("Southpaw Games {}", current_time));

    if destination_dir.exists() {
        fs::remove_dir_all(&destination_dir)?;
    }

    fs::create_dir_all(&destination_dir)?;

    copy_directory(&source_dir, &destination_dir)?;

    println!("Directory copied successfully.");
    Ok(())
}

fn copy_directory(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry_path.file_name().ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                "Failed to retrieve file name from entry path.",
            )
        })?;

        let destination_path = destination.join(file_name);
        if entry_path.is_dir() {
            fs::create_dir_all(&destination_path)?;
            copy_directory(&entry_path, &destination_path)?;
        } else {
            fs::copy(&entry_path, &destination_path)?;
        }
    }
    Ok(())
}
