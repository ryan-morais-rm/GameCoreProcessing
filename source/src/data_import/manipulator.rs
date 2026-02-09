use std::collections::HashMap;
use crate::game::Game;

pub struct Manipulator {
    // Total quantity of games 
    pub total_games: usize,
    pub total_systems: u16, 
    pub games_year: HashMap<u16, u32>,
    // Vector with Game Structs
    pub games: Vec<Game>
}

pub impl Manipulator {
    fn new() -> Self {
        Self {
            total_games: 0, 
            total_systems: 0, 
            games_year: Vec::new(),
            games: Vec::new(), 
        }
    }

    fn load_data(&self) {
        // Load clean csv and update games vector with the Structs Game
    }
    pub fn count_games(&self) {
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
    pub fn find_game(&self) {
        // Search game
        todo!(); 
    }
}
