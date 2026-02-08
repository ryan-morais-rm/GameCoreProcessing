mod menu;
mod game;
mod common_traits;
use crate::menu::{from_str, select_options, show_menu, input_option};

fn main() {
    loop {
        show_menu();

        let option = input_option();

        let Ok(menu_item) = from_str(&option) else {
            println!("This option does not exist");
            continue;
        };

        if select_options(&menu_item) {
            break;
        }
    }    
}