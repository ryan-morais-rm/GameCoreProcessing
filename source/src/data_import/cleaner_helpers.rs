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

pub fn validate_system(raw_os: &str) -> Vec<&str>{
    let parts: Vec<&str> = raw_os.split(',').collect();
    let mut cleaned_systems = Vec::new();
    
    for part in parts {
        let p = part.trim();
        let p_lower = p.to_lowercase();

        if p_lower.contains("macintosh") {
            cleaned_systems.push("Macintosh");
        } else if p_lower.contains("mac") { 
            cleaned_systems.push("MacOS");
        } else if p_lower.contains("windows") {
            cleaned_systems.push("Windows");
        } else if p_lower.contains("linux") {
            cleaned_systems.push("Linux");
        } else if p_lower.contains("beos") {
            cleaned_systems.push("BeOS");
        } else if p_lower.contains("dos") {
            cleaned_systems.push("MS-DOS");
        } else {
            if !p.is_empty() {
                cleaned_systems.push(p);
            }
        }
    }
    cleaned_systems.sort(); 
    cleaned_systems.dedup();
    
    cleaned_systems
}

pub fn field_clean_blank(line: &String) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut inside_quotes = false;
    
    for c in line.chars() {
        match c {
            '"' => inside_quotes = !inside_quotes, 
            
            ',' if !inside_quotes => {
                fields.push(current_field.trim().to_string());
                current_field.clear();
            }
            
            _ => current_field.push(c),
        }
    }
    fields.push(current_field.trim().to_string());

    fields
}

pub fn check_dataset(content: &Vec<String>) {
    println!("\n=== DEBUG: Checking data integrity ===");        
    println!("--- HEAD (First 20 lines) ---");
    
    for line in content.iter().take(10) {
        println!("{}", line);
    }
    
    println!("\n--- TAIL (Last 20 lines) ---");
    if content.len() > 20 {
        for line in content.iter().skip(content.len() - 20) {
            println!("{}", line);
        }
    }
    println!("=========================================\n");
}