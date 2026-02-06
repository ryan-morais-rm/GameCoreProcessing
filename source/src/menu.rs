use std::fmt;

#[derive(Debug, PartialEq)] 
pub enum OptionMenu {
    Q, 
    S, 
    L, 
    E, 
}

impl fmt::Display for OptionMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionMenu::Q => write!(f, "Option: Quantity of games"),
            OptionMenu::S => write!(f, "Option: Search for a game"),
            OptionMenu::L => write!(f, "Option: List games"),
            OptionMenu::E => write!(f, "Option: Exit"),
        }
    }
}

pub fn from_str_to_option_menu(s: &String) -> Result<OptionMenu, String> {
    match s.to_uppercase().trim() {
        "Q" => Ok(OptionMenu::Q),
        "S" => Ok(OptionMenu::S),
        "L" => Ok(OptionMenu::L),
        "E" => Ok(OptionMenu::E),
        _ => Err(format!("The option '{}' does not exist!", s)),
    }
}

pub fn show_menu() {
    println!("=========== MENU ==========="); 
    println!("(q) quantity of games"); 
    println!("(s) search game"); 
    println!("(l) list games"); 
    println!("(e) exit");  
}

pub fn select_options(option: &OptionMenu) -> Result<bool, String> {
    match option {
        OptionMenu::Q => { 
            println!("Processing quantity...");
            Ok(true)
        }
        OptionMenu::S => {
            println!("Searching for games...");
            Ok(true)
        }
        OptionMenu::L => {
            println!("Listing games...");
            Ok(true)
        }
        OptionMenu::E => {
            println!("Leaving...");
            Ok(false)
        }
    }
}