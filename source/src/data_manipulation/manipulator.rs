use std::collections::HashMap;
use std::path::PathBuf;
use crate::game::Game;
use crate::common_traits::helpers::{extract_column, load_file, sleep};

pub struct Manipulator {
    // Total quantity of games 
    pub total_games: usize,
    pub systems_count: HashMap<String, u32>, 
    pub games_year: HashMap<u16, u32>,
    pub games: Vec<Game>,
}

impl Manipulator {
    pub fn new() -> Self {
        Self {
            total_games: 0, 
            systems_count: HashMap::new(), 
            games_year: HashMap::new(),
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
        
        sleep();
        
        Ok(())
    }

    pub fn count_games(&self) -> Result<u16, ()>{
        println!("Counting games...");
        sleep();
        todo!(); 
    }
    pub fn count_systems(&self) {
        // Operating system most used.
        println!("Counting systems...");
        todo!(); 
    }
    pub fn games_released_year(&self) {
        // Games released from each year.
        println!("Listing games per year...");
        sleep();
        todo!(); 
    }
    pub fn find_game(&self) -> Result<bool, ()>{
        println!("Searching for games...");
        sleep();
        todo!(); 
    }
}