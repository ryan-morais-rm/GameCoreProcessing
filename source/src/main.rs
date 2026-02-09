mod menu;
mod game;
mod common_traits;
use crate::menu::{from_str, input_option, select_options, show_menu};

fn main() {
    // Process of data cleaning and import of data from "computer_games.csv" to main.rs
    clean_data();
    import_data();

    // Main program
    loop {
        show_menu();

        let option = input_option();

        let menu_item =  from_str(&option);

        match select_options(&menu_item) {
            Ok(true) => break, 
            Ok(false) => continue, 
            Err(msg) => println!("{}", msg),
        }
    }    
}

fn clean_data() {
    todo!()
}

fn import_data() {
    todo!()
}