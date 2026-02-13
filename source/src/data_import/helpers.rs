// Cleaner game functions
pub fn extract_game_name(line: &str) -> String {
    let mut name = String::new();
    let mut inside_quotes = false;

    for c in line.chars() {
        match c {
            '"' => inside_quotes = !inside_quotes, 
            ',' if !inside_quotes => break,        
            _ => name.push(c),                     
        }
    }
    name.trim().trim_matches('"').to_string()
}