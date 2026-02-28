use std::fmt;
use super::common_traits::data::Metadata;

pub struct Game {
    name: String, 
    developer: String,
    producer: String, 
    genre: String, 
    system: String,
    date: String, 
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, 
                   Developer: {}, 
                   Producer: {}, 
                   Genre: {}, 
                   System: {}, 
                   Date: {}", 
        self.name, self.developer, self.producer, self.genre, self.system, self.date)
    }
}

impl Metadata for Game {
    fn common(&self) -> String {
        format!("name: {}, 
                 date: {}", 
        self.name, self.date)
    }
    fn specific(&self) -> String {
        format!("producer: {}, 
                 developer: {}", 
        self.producer, self.developer)
    }
}

impl Game {
    pub fn new(name: String, developer: String, producer:String, 
        genre: String, system: String, date: String,) -> Self {
        Self { name, developer, producer, genre, system, date }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn producer(&self) -> &str {
        &self.producer
    }
    pub fn developer(&self) -> &str {
        &self.developer
    }
    pub fn genre(&self) -> &str {
        &self.genre 
    }
    pub fn system(&self) -> &str {
        &self.system
    }
    pub fn date(&self) -> &str {
        &self.date
    }
}