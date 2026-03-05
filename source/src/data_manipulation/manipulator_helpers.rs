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
                Ok(msg) => {
                    println!("Games: {:?}", msg);
                    return Ok(())
                },
                Err(msg) => println!("{}", msg)
            };
            return Err(())
        },
        OptionMenu::P => {
            let producer = input();
            println!("Searching for games with producer: {}", &producer); 
            
            match search_producer(producer, games) {
                Ok(msg) => {
                    println!("Games: {:?}", msg);
                    return Ok(())
                },
                Err(msg) => println!("{}", msg)
            };
            return Err(())
        },
        OptionMenu::D => {
            let developer = input();
            println!("Searching for games with developer: {}", &developer);

            match search_developer(developer, games) {
                Ok(msg) => {
                    println!("Games: {:?}", msg);
                    return Ok(())
                },
                Err(msg) => println!("{}", msg)
            };
            return Err(())
        },
        OptionMenu::G => {
            let genre = input();
            println!("Searching for games with genre: {}", &genre);

            match search_genre(genre, games) {
                Ok(msg) => {
                    println!("Games: {:?}", msg);
                    return Ok(())
                },
                Err(msg) => println!("{}", msg)
            };
            return Err(())
        },
        OptionMenu::S => {
            let system = input();
            println!("Searching for games with system: {}", &system);
            
            match search_system(system, games) {
                Ok(msg) => {
                    println!("Games: {:?}", msg);
                    return Ok(())
                },
                Err(msg) => println!("{}", msg)
            };
            return Err(())
        },
        OptionMenu::T => {
            let date = input();
            println!("Searching for games with date: {}...", &date);
            
            match search_date(date, games) {
                Ok(msg) => {
                    println!("Games: {:?}", msg);
                    return Ok(())
                },
                Err(msg) => println!("{}", msg)
            };
            return Err(())
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
                return Ok(format!("Search finished!"))
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