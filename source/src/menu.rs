use std::io;

#[derive(Debug, PartialEq)] 
pub enum OptionMenu {
    Q, 
    S, 
    L, 
    E, 
}

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

pub fn from_str(s: &String) -> Result<OptionMenu, String> {
    match s.to_uppercase().trim() {
        "Q" => Ok(OptionMenu::Q),
        "S" => Ok(OptionMenu::S),
        "L" => Ok(OptionMenu::L),
        "E" => Ok(OptionMenu::E),
        _ => Err(format!("{}", s)),
    }
}

pub fn select_options(option: &Result<OptionMenu, String>) -> Result<bool, String> {
    match option {
        Ok(OptionMenu::Q) => { 
            println!("Processing quantity...");
            Ok(false)
        }
        Ok(OptionMenu::S) => {
            println!("Searching for games...");
            Ok(false)
        }
        Ok(OptionMenu::L) => {
            println!("Listing games...");
            Ok(false)
        }
        Ok(OptionMenu::E) => {
            println!("Leaving program...");
            Ok(true)
        }
        Err(msg) => {
            Err(format!("This option does not exist: {}", msg))
        }
    }
}