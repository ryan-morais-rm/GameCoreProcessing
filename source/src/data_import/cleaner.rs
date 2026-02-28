use std::fs;
use std::path::{PathBuf};
use crate::common_traits::data::{sleep};

use super::cleaner_helpers::{check_dataset, extract_game_name, 
                             validate_system, field_clean_blank,
                             show_path};

pub struct Cleaner {
    pub input_path: PathBuf, 
    pub output_path: PathBuf,
    pub content: Vec<String>, 
}

impl Cleaner {
    pub fn new(input: &str, output: &str) -> Self {
        Self {
            input_path: PathBuf::from(input),
            output_path: PathBuf::from(output), 
            content: Vec::new(),
        }
    }
    // Main functions that invokes secondary functions... 
    pub fn run_cleaning_process(&mut self) -> Result<(), ()> {
        sleep();
        match self.load_file() {
            Ok(msg) => println!("{}", msg),
            Err(e) => {
                println!("{}", e);
                return Err(()); 
            }
        }
        sleep();
        match self.clean_blank() {
            Ok(msg) => println!("{}", msg),
            Err(_) => {
                println!("It could not clean blanks from the .csv");
                return Err(()); 
            }
        }
        sleep();
        match self.clean_name() {
            Ok(msg) => println!("{}", msg),
            Err(_) => {
                println!("It could not clean name column from the .csv");
                return Err(());
            }
        }
        sleep();
        match self.clean_date() {
            Ok(msg) => println!("{}", msg),
            Err(_) => {
                println!("It could not clean date column from the .csv");
                return Err(());
            }
        }
        sleep();
        match self.clean_system() {
            Ok(msg) => println!("{}", msg),
            Err(_) => {
                println!("It could not clean system column from the .csv");
                return Err(());
            }
        }
        sleep();
        check_dataset(&self.content);  
        sleep();
        match self.save_file() {
            Ok(msg) => println!("{}", msg), 
            Err(_) => {
                println!("It could not save the file!");
                return Err(());
            }
        }
        sleep();

        Ok(())
    }

    fn load_file(&mut self) -> Result<String, String> {
        if !self.input_path.exists() {
            return Err(format!("There is no file in this path!")); 
        }
        if self.input_path.is_dir() {
            return Err(format!("It is a directory! not a File!"));
        }
        let raw_text = fs::read_to_string(&self.input_path)
        .map_err(|e| e.to_string())?;
        
        self.content = raw_text.lines().map(|s| s.to_string()).collect();
        
        Ok(format!("File stored and prepared to be used!"))
    }

    fn save_file(&self) -> Result<String, String> {
        if self.output_path.exists() {
            return Err("The file already exists!".to_string());
        }

        let text_to_save = self.content.join("\n");

        if let Err(e) = fs::write(&self.output_path, text_to_save) {
            return Err(format!("Error saving the file: {}", e));
        }

        Ok(format!(
            "File {} has been cleaned and it is saved as {}", 
            show_path(&self.input_path), 
            show_path(&self.output_path)
        ))
    }

    // Secondary functions
    fn clean_name(&mut self) -> Result<String, ()> {
        let len_before = self.content.len();

        self.content.dedup_by(|a, b| {
            let name_a = extract_game_name(a);
            let name_b = extract_game_name(b);

            name_a.eq_ignore_ascii_case(&name_b)
        });

        let len_after = self.content.len();
        
        Ok(format!(
            "Duplicated games: {} deleted games.", 
            len_before - len_after
        ))
    }

    pub fn clean_system(&mut self) -> Result<String, ()> {
        let mut changed = 0;

        for line in self.content.iter_mut() {
            let mut fields = Vec::new();
            let mut current_field = String::new();
            let mut inside_quotes = false;

            for c in line.chars() {
                match c {
                    '"' => inside_quotes = !inside_quotes, 
                    ',' if !inside_quotes => {
                        fields.push(current_field.trim().to_string());
                        current_field.clear();
                    }
                    _ => current_field.push(c),
                }
            }
            fields.push(current_field.trim().to_string()); 

            if fields.len() != 6 { continue; }
            
            let raw_os = &fields[4];
            
            let cleaned_systems = validate_system(raw_os);
            
            let new_os_field = cleaned_systems.join(", ");
            
            fields[4] = new_os_field;

            let new_line: Vec<String> = fields.iter()
                .map(|f| {
                    if f.contains(',') {
                        format!("\"{}\"", f)
                    } else {
                        f.to_string()
                    }
                })
                .collect();
            *line = new_line.join(",");
            changed += 1;
        }

        Ok(format!("Systems cleaned. {} processed lines.", changed))
    }

    fn clean_date(&mut self) -> Result<String, ()> {
        let mut modifieds = 0;

        for line in self.content.iter_mut() {
            let mut comma_count = 0;
            let mut start_index = 0;
            let mut inside_quotes = false;

            for (i, c) in line.chars().enumerate() {
                if i == 0 { continue; }
                if c == '"' { inside_quotes = !inside_quotes; }
                if c == ',' && !inside_quotes {
                    comma_count += 1;
                    if comma_count == 5 {
                        start_index = i + 1; 
                        break;
                    }
                }
            }

            if comma_count != 5 { continue; }
            let raw_date = &line[start_index..];
            let clean_str = raw_date.trim().trim_matches('"');
            
            let year = if clean_str.len() >= 4 {
                let last_4 = &clean_str[clean_str.len()-4..];
                if last_4.chars().all(|c| c.is_numeric()) {
                    last_4
                } else {
                    "0000" 
                }
            } else {
                "0000"
            };

            let nova_linha = format!("{}{}", &line[..start_index], year);
            *line = nova_linha;
            modifieds += 1;
        }

        Ok(format!("Dates are cleaned. {} lines has been adjusted 
        to format AAAA.", modifieds))
    }

    fn clean_blank(&mut self) -> Result<String, bool> {
        let len_before = self.content.len();

        self.content.retain(|line| {
            if line.trim().is_empty() { return false; }
            
            let fields = field_clean_blank(line);

            if fields.len() != 6 {
                println!("Removed (Wrong columns: {}): {}", fields.len(), line);
                return false; 
            }

            for field in fields {
                let limpo = field.trim_matches('"'); 
                if limpo.trim().is_empty() {
                    return false;
                }
            }

            true
        });

        let len_then = self.content.len();

        Ok(format!("Cleaning of blank lines has ended!: {} removed lines. Rest: {}", 
        len_before - len_then, len_then))
    }
}