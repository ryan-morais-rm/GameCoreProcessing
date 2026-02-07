use std::io;
mod menu;
use crate::menu::{from_str, select_options, show_menu};

fn main() {
    loop {
        let mut option = String::new(); 

        show_menu();

        if io::stdin().read_line(&mut option).is_err() {
            println!("Could not read input");
            continue; 
        }

        let Ok(menu_item) = from_str(&option) else {
            println!("Option does not exist!");
            continue; 
        };

        match select_options(&menu_item) {
            Ok(true) => break, 
            Ok(false) => continue, 
            Err(msg) => println!("Could not continue: {}", msg), 
        }
    }    
}