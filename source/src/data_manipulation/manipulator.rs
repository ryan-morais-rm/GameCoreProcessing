use std::path::PathBuf;
use crate::game::Game;
use crate::common_traits::helpers::{extract_column, load_file, sleep};
use super::manipulator_helpers::{format_counts, find, CountData};

pub struct Manipulator {
    // Total quantity of games 
    pub total_games: usize,
    pub games: Vec<Game>,
}

impl Manipulator {
    pub fn new() -> Self {
        Self {
            total_games: 0, 
            games: Vec::new(), 
        }
    }

    pub fn load_data(&mut self, file_path: &PathBuf) -> Result<(), String> {
        let raw_lines = load_file(file_path)?;

        for (i, line) in raw_lines.iter().enumerate() {
            if i == 0 { continue; } 

            let columns = extract_column(line); 

            if columns.len() != 6 { continue; } 

            let date_u16 = columns[5].parse::<u16>().unwrap_or(0);

            let game = Game::new(
                columns[0].clone(),
                columns[1].clone(),
                columns[2].clone(),
                columns[3].clone(),
                columns[4].clone(),
                date_u16,
            );

            self.games.push(game);
        }

        self.total_games = self.games.len();
        
        sleep(1);
        
        Ok(())
    }

    pub fn count_games(&self) {
        println!("Counting games...\n");
        sleep(1);
        println!("There are {} games!", &self.total_games);
    }

    pub fn count_systems(&self) {
        println!("Counting systems...\n");
        sleep(1);

        let mut counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
        
        for game in &self.games {
            let systems_list: Vec<&str> = game.system().split(',').collect();
            
            for sys in systems_list {
                let clean_sys = sys.trim();
                if !clean_sys.is_empty() {
                    *counts.entry(clean_sys.to_string()).or_insert(0) += 1;
                }
            }
        }

        let vec_systems = counts.into_iter().collect();

        let count_data = CountData::StringData(vec_systems);

        let result = format_counts(count_data);

        println!("{}", result);

        sleep(10);
    }

    pub fn games_per_year(&self) {
        println!("Counting games by release year...\n");
        sleep(1);

        let mut counts: std::collections::HashMap<u16, u32> = std::collections::HashMap::new();
        
        for game in &self.games {
            let year = game.date();
            *counts.entry(*year).or_insert(0) += 1;
        }

        let vec_years = counts.into_iter().collect();

        let count_data = CountData::NumberData(vec_years);

        let result = format_counts(count_data);

        println!("{}", result);

        sleep(10);
    }

    pub fn find_game(&self) /*-> Result<bool, ()>*/ {
        match find(&self.games) {
            Ok(msg) => println!("{}", msg),
            Err(msg) => println!("{}", msg)
        }
        sleep(10);
    }
}