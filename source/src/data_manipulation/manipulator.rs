use std::collections::HashMap;
use crate::game::Game;

pub struct Manipulator {
    // Total quantity of games 
    pub total_games: usize,
    pub systems_count: HashMap<String, u32>, 
    pub games_year: HashMap<u16, u32>,
    pub games: Vec<Game>,
}

impl Manipulator {
    fn new() -> Self {
        Self {
            total_games: 0, 
            total_systems: HashMap::new(), 
            games_year: HashMap::new(),
            games: Vec::new(), 
        }
    }

    fn load_data(&mut self, file_path: &str) -> Result<(), ()>{
        // Load clean csv and update games vector with the Structs Game
    }
    pub fn count_games(&self) -> Result<u16, ()>{
        // Quantity of games loaded.
        todo!(); 
    }
    pub fn count_systems(&self) {
        // Operating system most used.
        todo!(); 
    }
    pub fn games_released_year(&self) {
        // Games released from each year.
        todo!(); 
    }
    pub fn find_game(&self) -> Result<bool, ()>{
        // Search game
        todo!(); 
    }
}