use std::fmt;

pub struct Game {
    name: String, 
    developer: String,
    producer: String, 
    genre: String, 
    system: String,
    date: u16, 
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Name: {}\nDeveloper: {}\nProducer: {}\nGenre: {}\nSystem: {}\nDate: {}", 
            self.name, self.developer, self.producer, self.genre, self.system, self.date
        )
    }
}

impl Game {
    pub fn new(name: String, developer: String, producer:String, 
        genre: String, system: String, date: u16,) -> Self {
        Self { 
            name, developer, producer, genre, system, date 
        }
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
    pub fn date(&self) -> &u16 {
        &self.date
    }
}