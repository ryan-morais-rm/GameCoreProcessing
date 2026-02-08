use std::io;

pub fn show_menu() {
    println!("=========== MENU ==========="); 
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

#[derive(Debug, PartialEq)] 
pub enum OptionMenu {
    Q, 
    S, 
    L, 
    E, 
}

pub fn from_str(s: &String) -> Result<OptionMenu, ()> {
    match s.to_uppercase().trim() {
        "Q" => Ok(OptionMenu::Q),
        "S" => Ok(OptionMenu::S),
        "L" => Ok(OptionMenu::L),
        "E" => Ok(OptionMenu::E),
        _ => Err(()),
    }
}

pub fn select_options(option: &OptionMenu) -> bool {
    match option {
        OptionMenu::Q => { 
            println!("Processing quantity...");
            false
        }
        OptionMenu::S => {
            println!("Searching for games...");
            false
        }
        OptionMenu::L => {
            println!("Listing games...");
            false
        }
        OptionMenu::E => {
            println!("Leaving program...");
            true
        }
    }
}