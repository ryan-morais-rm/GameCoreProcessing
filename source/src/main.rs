use std::io;
mod menu;
use crate::menu::{from_str_to_option_menu, select_options, show_menu};

fn main() {
    loop {
        let mut option = String::new(); 

        show_menu();

        if io::stdin().read_line(&mut option).is_err() {
            println!("Could not read input");
            continue; 
        }

        let conversion_result = from_str_to_option_menu(&option);

        match conversion_result {
            Ok(menu_item) => {
                match select_options(&menu_item) {
                    Ok(should_continue) => {
                        if !should_continue {
                            break; 
                        }
                    },
                    Err(msg) => println!("Could not continue: {}", msg),
                }
            },
            Err(msg) => {
                println!("This option does not exist in menu: {}", msg);
            }
        }
    }
    
    println!("Program has ended!");
}