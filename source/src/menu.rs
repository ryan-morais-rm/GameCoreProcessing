#[derive(Debug, PartialEq)] 
pub enum OptionMenu {
    Q, 
    S, 
    L, 
    E, 
}

pub fn from_str(s: &String) -> Result<OptionMenu, String> {
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
            Ok(false)
        }
        OptionMenu::S => {
            println!("Searching for games...");
            Ok(false)
        }
        OptionMenu::L => {
            println!("Listing games...");
            Ok(false)
        }
        OptionMenu::E => {
            println!("Leaving...");
            Ok(true)
        }
    }
}