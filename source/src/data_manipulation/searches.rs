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

pub fn search_producer(producer: String, game: &Vec<Game>) -> Result<Vec<String>, ()> {
    todo!()
}

pub fn search_developer(developer: String, game: &Vec<Game>) -> Result<Vec<String>, ()> {
    todo!()
}

pub fn search_system(system: String, game: &Vec<Game>) -> Result<Vec<String>, ()> {
    todo!()
}

pub fn search_genre(genre: String, game: &Vec<Game>) -> Result<Vec<String>, ()> {
    todo!()
}

pub fn search_date(date: String, game: &Vec<Game>) -> Result<Vec<u16>, ()> {
    todo!()
}