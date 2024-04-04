use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn get_input(query: &str) -> std::io::Result<String> {
    print!("{}",query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}


pub fn create_dir_if_not_exists(dir: &PathBuf) {
    if !dir.exists() {
        if let Err(err) = fs::create_dir(dir) {
            println!("failed to created dir: {}",dir.display());
        }
    }
}


pub fn move_file(from: &PathBuf, to: &PathBuf) {
    if let Err(err) = fs::rename(from,to) {
        println!("failed to move file due to err: {}",err);
    }
}


pub fn organize_dir(dir_path: PathBuf) {
    if !dir_path.exists() {
        println!("Directory {} doesn't exist",dir_path.display());
        return
    }
    let dir_files = match dir_path.read_dir() {
        Ok(dir_files) => dir_files,
        Err(err) => {
            println!("Error opening directory {}: {}",dir_path.display(),err);
            return;
        }
    };
    for file in dir_files {
        if let Ok(file) = file {
            if file.path().is_dir() {
                println!("Path {} is directory, skipped", file.path().display());
                continue;
            }
            let file_extension = match file.path().extension() {
                None => {
                    println!("file hasn't extension: {}",file.path().display());
                    continue;
                }
                Some(extension) => match extension.to_str() {
                    None => continue,
                    Some(_) => extension.to_ascii_lowercase()
                }
            };
            let extension_dir = PathBuf::from(dir_path.join(file_extension));
            create_dir_if_not_exists(&extension_dir);
            move_file(&file.path(), &extension_dir.join(file.file_name()));
        }
    }


}