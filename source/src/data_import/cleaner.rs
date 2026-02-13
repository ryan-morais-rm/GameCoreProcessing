use std::arch::x86_64::_MM_MANT_SIGN_ZERO;
use std::fs;
use std::path::{PathBuf};
use super::helpers::extract_game_name;

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
        match self.load_data() {
            Ok(msg) => println!("{}", msg),
            Err(e) => {
                println!("{}", e);
                return Err(()); 
            }
        }

        match self.clean_blank() {
            Ok(msg) => println!("{}", msg),
            Err(_) => {
                println!("It could not clean blanks from the .csv");
                return Err(()); 
            }
        }

        match self.clean_name() {
            Ok(msg) => {
                println!("{}", msg);
                Ok(())
            },
            Err(_) => {
                println!("It could not clean name column from the .csv");
                Err(())
            }
        }
        // self.clean_name();   
        // self.clean_date();   
        // self.clean_system(); 
        // self.save_data();
    }

    pub fn load_data(&mut self) -> Result<String, String> {
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

    fn save_data(&self) -> Result<bool, String> {
        // Save the changes on another clean computer_games_clean.csv
        todo!()
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

    fn clean_system(&self) -> Result<bool, String> {
        // This function is the fourth!
        // Token. So, "Microsoft Windows" will be only "Windows"...
        // MacOS or macOS will be macOS
        // Linux stays same
        // Macintosh stays same
        todo!()
    }

    fn clean_date(&self) -> Result<bool, String>{
        // This function is the third!
        // "November 28, 1928" will be only "1928"...
        todo!()
    }
    
    pub fn clean_blank(&mut self) -> Result<String, bool> {
        let len_before = self.content.len();

        self.content.retain(|line| {
            if line.trim().is_empty() { return false; }

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

            if fields.len() != 6 {
                println!("Removido (Colunas erradas: {}): {}", fields.len(), line);
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