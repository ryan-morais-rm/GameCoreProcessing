use std::io;
use std::path::PathBuf;
use super::data_manipulation::manipulator::Manipulator;

#[derive(Debug, PartialEq)] 
pub enum OptionMenu {
    O,
    Q, 
    S, 
    L, 
    E, 
}

pub fn show_menu() {
    println!("=========== MENU ==========="); 
    println!("(o) quantity of systems"); 
    println!("(q) quantity of games");
    println!("(s) search game"); 
    println!("(l) list games"); 
    println!("(e) exit");  
}

pub fn input_option() -> String {
    let mut option = String::new(); 
    io::stdin().read_line(&mut option).expect("Option was not received");
    option
}

pub fn from_str(s: &String) -> Result<OptionMenu, String> {
    match s.to_uppercase().trim() {
        "O" => Ok(OptionMenu::O),
        "Q" => Ok(OptionMenu::Q),
        "S" => Ok(OptionMenu::S),
        "L" => Ok(OptionMenu::L),
        "E" => Ok(OptionMenu::E),
        _ => Err(format!("{}", s)),
    }
}

pub fn select_options(option: &Result<OptionMenu, String>) -> Result<bool, String> {
    let mut data_manipulator = Manipulator::new();

    let file = PathBuf::from("../info/games_cleaned.csv");

    data_manipulator.load_data(&file);

    match option {
        Ok(OptionMenu::O) => {
            data_manipulator.count_systems();
            Ok(false)
        },
        Ok(OptionMenu::Q) => { 
            println!("{}", data_manipulator.count_games());
            Ok(false)
        },
        Ok(OptionMenu::S) => {
            data_manipulator.find_game();
            Ok(false)
        },
        Ok(OptionMenu::L) => {
            data_manipulator.games_per_year();
            Ok(false)
        },
        Ok(OptionMenu::E) => {
            println!("Leaving program...");
            Ok(true)
        },
        Err(msg) => {
            Err(format!("This option does not exist: {}", msg))
        }
    }
}