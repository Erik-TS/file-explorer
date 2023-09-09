use std::env;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::ops::Add;
use std::path::Path;

fn main() {
    let initial_path = Path::new("/");
}

fn create_file(path: &Path, file_name: &str) -> Result<String, Error> {
    let new_file_path = path.join(Path::new(file_name));

    if !new_file_path.exists() {
        return match File::create(new_file_path) {
            Ok(_) => Ok(String::from("File created with success.")),
            Err(err) => Err(err),
        };
    }
    Ok(String::from("File already exists."))
}

fn remove_file(path: &Path, file_name: &str) -> Result<String, Error> {
    let removed_file_path = path.join(Path::new(file_name));

    match fs::remove_file(removed_file_path) {
        Ok(_) => Ok(String::from("File removed with success.")),
        Err(err) => Err(err),
    }
}

fn create_folder(folder_path: &Path, folder_name: &str) -> Result<String, Error> {
    let new_folder_path = folder_path.join(Path::new(folder_name));

    match fs::create_dir(new_folder_path) {
        Ok(_) => Ok(String::from("Folder created with success.")),
        Err(err) => Err(err),
    }
}

fn remove_folder(folder_path: &Path, folder_name: &str) -> Result<String, Error> {
    let path = folder_path.join(Path::new(folder_name));

    match fs::remove_dir(path) {
        Ok(_) => Ok(String::from("Folder removed with success.")),
        Err(err) => Err(err),
    }
}

fn rename(path: &Path, name: &str, new_name: &str) -> Result<String, Error> {
    let mut i = 1;
    let mut final_name = new_name.to_string();

    while path.join(&final_name).exists() {
        i += 1;
        final_name = format!("{}({})", new_name, i);
    }

    match fs::rename(name, final_name) {
        Ok(_) => Ok(String::from("File renamed with success.")),
        Err(err) => Err(err),
    }
}

fn change_current_dir(dir: &Path) -> Result<String, Error> {
    match env::set_current_dir(dir) {
        Ok(_) => {
            let success_message: String;

            match dir.to_str() {
                Some(dir_str) => {
                    success_message = String::from("Current directory: ").clone().add(dir_str)
                }
                None => {
                    success_message = String::from("Current directory changed! Failed printing it.")
                }
            }

            Ok(success_message)
        }
        Err(err) => Err(err),
    }
}

fn get_dir_content(dir: &Path) -> Result<Vec<String>, Error> {
    let mut dir_content: Vec<String> = Vec::new();

    match fs::read_dir(dir) {
        Ok(read_dir) => {
            for file in read_dir {
                match file {
                    Ok(dir_entry) => match dir_entry.path().to_str() {
                        Some(name) => dir_content.push(name.to_string()),
                        None => println!("Couldn't get name."),
                    },
                    Err(msg) => println!("Error: {}", msg),
                }
            }

            Ok(dir_content)
        }
        Err(err) => Err(err),
    }
}
