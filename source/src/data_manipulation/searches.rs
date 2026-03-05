use crate::game::Game;
use crate::common_traits::helpers::levenshtein;

pub fn search_name(search_term: String, games: &Vec<Game>) -> Result<Vec<&str>, String> {
    let mut matches: Vec<&str> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let game_name: &str = game.name(); 
        let name_lower = game_name.to_lowercase();

        if name_lower.contains(&target_lower) {
            matches.push(game_name);
            continue;
        }

        let distance = levenshtein(&target_lower, &name_lower);
        
        let tolerance = if search_len > 5 { 3 } else { 1 };

        if distance <= tolerance {
            matches.push(game_name);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_producer(search_term: String, games: &Vec<Game>) -> Result<Vec<&str>, String> {
    let mut matches: Vec<&str> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let game_name: &str = game.producer(); 
        let name_lower = game_name.to_lowercase();

        if name_lower.contains(&target_lower) {
            matches.push(game_name);
            continue;
        }

        let distance = levenshtein(&target_lower, &name_lower);
        
        let tolerance = if search_len > 5 { 3 } else { 1 };

        if distance <= tolerance {
            matches.push(game_name);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_developer(search_term: String, games: &Vec<Game>) -> Result<Vec<&str>, String> {
    let mut matches: Vec<&str> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let game_name: &str = game.name(); 
        let name_lower = game_name.to_lowercase();

        if name_lower.contains(&target_lower) {
            matches.push(game_name);
            continue;
        }

        let distance = levenshtein(&target_lower, &name_lower);
        
        let tolerance = if search_len > 5 { 3 } else { 1 };

        if distance <= tolerance {
            matches.push(game_name);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_system(search_term: String, games: &Vec<Game>) -> Result<Vec<&str>, String> {
    let mut matches: Vec<&str> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let game_name: &str = game.name(); 
        let name_lower = game_name.to_lowercase();

        if name_lower.contains(&target_lower) {
            matches.push(game_name);
            continue;
        }

        let distance = levenshtein(&target_lower, &name_lower);
        
        let tolerance = if search_len > 5 { 3 } else { 1 };

        if distance <= tolerance {
            matches.push(game_name);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_genre(search_term: String, games: &Vec<Game>) -> Result<Vec<&str>, String> {
    let mut matches: Vec<&str> = Vec::new();
        
    let target_lower = search_term.to_lowercase();
    let search_len = target_lower.len();

    for game in games {
        let game_name: &str = game.name(); 
        let name_lower = game_name.to_lowercase();

        if name_lower.contains(&target_lower) {
            matches.push(game_name);
            continue;
        }

        let distance = levenshtein(&target_lower, &name_lower);
        
        let tolerance = if search_len > 5 { 3 } else { 1 };

        if distance <= tolerance {
            matches.push(game_name);
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", search_term))
    } else {
        Ok(matches)
    }
}

pub fn search_date(date: String, game: &Vec<Game>) -> Result<Vec<u16>, String> {
    let target_year: u16 = match date.trim().parse() {
        Ok(year) => year,
        Err(_) => return Err(format!("It is not a possible year")),
    };

    let mut matches: Vec<u16> = Vec::new();

    for g in game {
        if *g.date() == target_year {
            matches.push(*g.date()); 
        }
    }

    if matches.is_empty() {
        Err(format!("No game with this pattern found '{}'.", date))
    } else {
        Ok(matches) 
    }
}