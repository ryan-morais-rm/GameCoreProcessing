use std::path::PathBuf;
use std::thread;
use std::fs;
use std::time::Duration;

pub trait Metadata {
    fn common(&self) -> String;
    fn specific(&self) -> String; 
}

pub fn sleep() {
    let secs: u64 = 1;
    thread::sleep(Duration::from_secs(secs));
}

pub fn load_file(input_path: &PathBuf) -> Result<Vec<String>, String> {
    let mut content: Vec<String> = Vec::new();

    if !input_path.exists() {
        return Err("There is no file in this path!".to_string());
    }
    
    if input_path.is_dir() {
        return Err("It is a directory! not a File!".to_string());
    }
        
    let raw_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    
    content = raw_text.lines().map(|s| s.to_string()).collect();

    sleep();
    
    // println!("File has been loaded and prepared to be used!");

    Ok(content)
}

pub fn extract_column(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut inside_quotes = false;

    for c in line.chars() {
        match c {
            '"' => inside_quotes = !inside_quotes,
            ',' if !inside_quotes => {
                fields.push(current_field.trim().trim_matches('"').to_string());
                current_field.clear();
            }
            _ => current_field.push(c),
        }
    }

    fields.push(current_field.trim().trim_matches('"').to_string());

    fields
}