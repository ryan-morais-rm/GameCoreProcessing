mod menu;
mod game;
mod common_traits;
mod data_import;
mod data_manipulation;
use crate::data_import::cleaner::{Cleaner};
use crate::common_traits::helpers::{sleep, clear_screen};
use crate::menu::{from_str, input, select_options, show_menu};

fn main() {
    clean_data();

    loop {
        sleep(2);
        clear_screen();
        show_menu();

        let option = input();

        let menu_item =  from_str(&option);

        match select_options(&menu_item) {
            Ok(true) => break, 
            Ok(false) => continue, 
            Err(msg) => println!("{}", msg),
        }
    }    
}

fn clean_data() {
    println!("Data cleaning...");
    let mut data_cleaner = Cleaner::new(
        "../info/computer_games.csv",  
        "../info/games_cleaned.csv"   
    );

    match data_cleaner.clean() {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }
}