use std::fs;
use std::path::PathBuf;
use crate::common_traits::helpers::{load_file, sleep};

use super::cleaner_helpers::{
    check_dataset, extract_game_name, validate_system, 
    field_clean_blank, show_path
};

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

    pub fn clean(&mut self) -> Result<String, String> {
        self.content = load_file(&self.input_path)?;
        println!("{}", self.blank()?);
        println!("{}", self.name()?);
        println!("{}", self.date()?);
        println!("{}", self.system()?);
        check_dataset(&self.content);
        println!("{}", self.save_file()?);

        Ok("Cleaning has been done!".to_string())
    }

    fn save_file(&self) -> Result<String, String> {
        if self.output_path.exists() {
            return Err("The file already exists!".to_string());
        }

        let text_to_save = self.content.join("\n");
        
        fs::write(&self.output_path, text_to_save)
            .map_err(|e| format!("Error saving the file: {}", e))?;

        sleep(1);

        Ok(format!(
            "File {} has been cleaned and it is saved as {}",
            show_path(&self.input_path), show_path(&self.output_path)
        ))
    }

    fn blank(&mut self) -> Result<String, String> {
        let len_before = self.content.len();

        self.content.retain(|line| {
            if line.trim().is_empty() { return false; }
            let fields = field_clean_blank(line);
            
            if fields.len() != 6 {
                return false; 
            }

            fields.into_iter().all(|field| !field.trim_matches('"').trim().is_empty())
        });

        sleep(1);

        let len_then = self.content.len();

        Ok(format!("Cleaning of blank lines has ended!: {} removed lines. Rest: {}", len_before - len_then, len_then))
    }

    fn name(&mut self) -> Result<String, String> {
        let len_before = self.content.len();

        self.content.dedup_by(|a, b| {
            extract_game_name(a).eq_ignore_ascii_case(&extract_game_name(b))
        });

        let len_after = self.content.len();
        
        sleep(1);

        Ok(format!("Duplicated games: {} deleted games.", len_before - len_after))
    }

    fn system(&mut self) -> Result<String, String> {
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
            fields[4] = cleaned_systems.join(", ");

            let new_line: Vec<String> = fields.iter()
                .map(|f| if f.contains(',') { format!("\"{}\"", f) } else { f.to_string() })
                .collect();
                
            *line = new_line.join(",");
            changed += 1;
        }

        sleep(1);

        Ok(format!("Systems cleaned. {} processed lines.", changed))
    }

    fn date(&mut self) -> Result<String, String> {
        let mut qtd_mods = 0;

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
            
            let clean_str = line[start_index..].trim().trim_matches('"');
            let year = if clean_str.len() >= 4 {
                let last_4 = &clean_str[clean_str.len()-4..];
                if last_4.chars().all(|c| c.is_numeric()) { last_4 } else { "0000" }
            } else { "0000" };

            *line = format!("{}{}", &line[..start_index], year);
            qtd_mods += 1;
        }

        sleep(1);

        Ok(format!("Dates are cleaned. {} lines has been adjusted to format AAAA.", qtd_mods))
    }
}