use std::env;
use std::fs;
use std::io::Error;
use std::ops::Add;
use std::path::Path;

fn main() {
    let start_dir = Path::new("/");
}

fn create_new_folder(folder_path: &Path, folder_name: &str) -> Result<String, Error> {
    let new_folder_path_string = format!("{}/{}", folder_path.to_string_lossy(), folder_name);

    match fs::create_dir(new_folder_path_string) {
        Ok(_) => Ok(String::from("Folder created with success.")),
        Err(err) => Err(err),
    }
}

fn change_current_dir(dir: &Path) -> Result<String, Error> {
    match env::set_current_dir(dir) {
        Ok(_) => {
            let success_message: String;

            match dir.to_str() {
                Some(dir_str) => success_message = String::from("Current directory: ").clone().add(dir_str),
                None => success_message = String::from("Current directory changed! Failed printing it.")
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
