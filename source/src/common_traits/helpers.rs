use std::process::Command;
use std::path::PathBuf;
use std::{thread, fs, io};
use std::time::Duration;

pub fn input() -> String {
    let mut option = String::new(); 
    io::stdin().read_line(&mut option).expect("Option was not received");
    option
}

pub fn clear_screen() {
    Command::new("clear").status().unwrap();
}

pub fn sleep(secs: u64) {
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

    sleep(1);
    
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

pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();

    if len_a == 0 { return len_b; }
    if len_b == 0 { return len_a; }

    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a { matrix[i][0] = i; }
    for j in 0..=len_b { matrix[0][j] = j; }

    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1) 
                .min(matrix[i][j - 1] + 1)    
                .min(matrix[i - 1][j - 1] + cost); 
        }
    }

    matrix[len_a][len_b]
}