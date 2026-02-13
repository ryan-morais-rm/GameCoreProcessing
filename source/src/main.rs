mod menu;
mod game;
mod common_traits;
mod data_import;
use crate::data_import::cleaner::Cleaner;
use crate::menu::{from_str, input_option, select_options, show_menu};

fn main() {
    // Process of data cleaning and import of data from "computer_games.csv" to main.rs
    clean_data();
    // import_data();

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
    let mut data_cleaner = Cleaner::new(
        "../info/computer_games.csv",  
        "../info/games_cleaned.csv"   
    );

    match data_cleaner.run_cleaning_process() {
        Ok(_) => println!("Everything went well!"),
        Err(_) => println!("Ops!,something bad has ocurred..."),
    }
}

/*
fn import_data() {
    todo!()
}
*/