use crate::game::{Game};
use crate::common_traits::helpers::{input, sleep};
use super::searches::{search_name, search_producer, search_developer,
                      search_system, search_genre, search_date};

enum OptionMenu {
    N,
    P,
    D,
    G,
    S,
    T
}

fn menu() {
    println!("(n) Name");
    println!("(p) Producer");
    println!("(d) Developer");
    println!("(g) Genre");
    println!("(s) System");
    println!("(t) Date\n");
}

fn select(option: &OptionMenu, games: &Vec<Game>) -> Result<(), ()>{
    println!("Input: ");

    match option {
        OptionMenu::N => {
            let name = input();
            println!("Searching for games with name: {} ", &name);
            match search_name(name, games) {
                Ok(msg) => println!("Games: {:?}", msg),
                Err(msg) => println!("{}", msg)
            };
            return Ok(())
        },
        OptionMenu::P => {
            let producer = input();
            println!("Searching for games with producer: {}", &producer); 
            // search_producer(producer, game);
            return Ok(())
        },
        OptionMenu::D => {
            let developer = input();
            println!("Searching for games with developer: {}", &developer);
            // search_developer(developer, game);
            return Ok(())
        },
        OptionMenu::G => {
            let genre = input();
            println!("Searching for games with genre: {}", &genre);
            // search_genre(genre, game);
            return Ok(())
        },
        OptionMenu::S => {
            let system = input();
            println!("Searching for games with system: {}", &system);
            // search_system(system, game);
            return Ok(())
        },
        OptionMenu::T => {
            let date = input();
            println!("Searching for games with date: {}...", &date);
            // search_date(date, game);
            return Ok(())
        }
    }
}

fn from_str(s: &String) -> Result<OptionMenu, String> {
    match s.to_uppercase().trim() {
        "N" => Ok(OptionMenu::N),
        "P" => Ok(OptionMenu::P),
        "G" => Ok(OptionMenu::G),
        "D" => Ok(OptionMenu::D),
        "S" => Ok(OptionMenu::S),
        "T" => Ok(OptionMenu::T),
        _ => Err(format!("{}", s)),
    }
}

pub fn find(game: &Vec<Game>) -> Result<String, String> {
    println!("Which field do you want to search?\n");
    sleep(1);
    menu();
    let option = input();

    if let Ok(option_enum) = from_str(&option) {
        println!("\nOption received!");

        match select(&option_enum, game) {
            Ok(_) => {
                sleep(1);
                return Ok(format!("Search finished successfully!"))
            },
            Err(_) => {
                sleep(1);
                return Err(format!("Error during search"))
            }
        }
    } else {
        sleep(1);
        return Err(format!("Invalid option. Try again"))
    }
}

pub enum CountData {
    StringData(Vec<(String, u32)>),
    NumberData(Vec<(u16, u32)>),
}

pub fn format_counts(data: CountData) -> String {
    match data {
        CountData::StringData(mut count_vec) => {
            count_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            let mut result = String::from("There are these systems:\n");
            result.push_str("------------------\n");
            
            for (system_name, quantity) in count_vec {
                result.push_str(&format!("{} - {}\n", system_name, quantity));
            }
            
            result 
        },
        
        CountData::NumberData(mut count_vec) => {
            count_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            let mut result = String::from("Games released per year:\n");
            result.push_str("------------------\n");
            
            for (year, quantity) in count_vec {
                if year == 0 {
                    result.push_str(&format!("0000 - {} games\n", quantity));
                } else {
                    result.push_str(&format!("{} - {} games\n", year, quantity));
                }
            }
            
            result
        }
    }
}