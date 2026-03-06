use crate::game::{Game};
use crate::common_traits::helpers::{levenshtein};

pub fn search_name(search_term: String, games: &Vec<Game>) -> Result<Vec<&str>, String> {
    let mut matches: Vec<&str> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let name: &str = game.name(); 
        let name_lower = name.to_lowercase();

        if name_lower.contains(&target_lower) {
            matches.push(name);
            continue;
        }

        let distance = levenshtein(&target_lower, &name_lower);
        let tolerance = if search_len > 5 { 3 } else { 1 };
        if distance <= tolerance {
            matches.push(name);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_producer(search_term: String, games: &Vec<Game>) -> Result<Vec<String>, String> {
    let mut matches: Vec<String> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let producer: &str = game.producer(); 
        let name: &str = game.name();
        let game_info: String = format!("Name: {} - Producer: {} ", name, producer);
        let producer_lower = producer.to_lowercase();

        if producer_lower.contains(&target_lower) {
            matches.push(game_info);
            continue;
        }

        let distance = levenshtein(&target_lower, &producer_lower);
        let tolerance = if search_len > 5 { 3 } else { 1 };
        if distance <= tolerance {
            matches.push(game_info);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_developer(search_term: String, games: &Vec<Game>) -> Result<Vec<String>, String> {
    let mut matches: Vec<String> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let name: &str = game.name(); 
        let developer: &str = game.developer();
        let developer_lower = developer.to_lowercase();
        let game_info = format!("Name: {} - Developer: {}", name, developer);

        if developer_lower.contains(&target_lower) {
            matches.push(game_info);
            continue;
        }

        let distance = levenshtein(&target_lower, &developer_lower);
        let tolerance = if search_len > 5 { 3 } else { 1 };
        if distance <= tolerance {
            matches.push(game_info);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_system(search_term: String, games: &Vec<Game>) -> Result<Vec<String>, String> {
    let mut matches: Vec<String> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let name: &str = game.name(); 
        let system: &str = game.system();
        let system_lower = system.to_lowercase();
        let game_info = format!("Name: {} - System: {}", name, system);

        if system_lower.contains(&target_lower) {
            matches.push(game_info);
            continue;
        }

        let distance = levenshtein(&target_lower, &system_lower);
        let tolerance = if search_len > 5 { 3 } else { 1 };
        if distance <= tolerance {
            matches.push(game_info);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_genre(search_term: String, games: &Vec<Game>) -> Result<Vec<String>, String> {
    let mut matches: Vec<String> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let name: &str = game.name();
        let genre: &str = game.genre();
        let genre_lower = genre.to_lowercase();
        let game_info = format!("Name: {} - Genre: {}", name, genre);

        if genre_lower.contains(&target_lower) {
            matches.push(game_info);
            continue;
        }

        let distance = levenshtein(&target_lower, &genre_lower);
        let tolerance = if search_len > 5 { 3 } else { 1 };
        if distance <= tolerance {
            matches.push(game_info);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_date(date: String, game: &Vec<Game>) -> Result<Vec<String>, String> {
    let target_year: u16 = match date.trim().parse() {
        Ok(year) => year,
        Err(_) => return Err(format!("It is not a possible year")),
    };

    let mut matches: Vec<String> = Vec::new();

    for g in game {
        if *g.date() == target_year {
            let name: &str = g.name();
            matches.push(format!("Name: {} - Date: {}", name, *g.date())); 
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", date))
    } else {
        Ok(matches) 
    }
}