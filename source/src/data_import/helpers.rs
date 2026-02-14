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

pub fn check_dataset(content: &Vec<String>) {
    println!("\n=== DEBUG: Checking data integrity ===");        
    println!("--- HEAD (First 5 lines) ---");
    
    for line in content.iter().take(5) {
        println!("{}", line);
    }
    
    println!("\n--- TAIL (Last 5 lines) ---");
    if content.len() > 5 {
        for line in content.iter().skip(content.len() - 5) {
            println!("{}", line);
        }
    }
    println!("=========================================\n");
}