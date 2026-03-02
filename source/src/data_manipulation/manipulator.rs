use std::path::PathBuf;
use std::collections::HashMap;
use crate::game::Game;
use crate::common_traits::data::sleep;

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