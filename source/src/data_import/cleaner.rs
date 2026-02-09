struct Cleaner {
    input_path: String,
    output_path: String,
    // Buffer temporário para segurar os dados enquanto limpa
    raw_data: Vec<String>
}

impl Cleaner {
    fn new(&self) -> Self {
        Self {
            input_path,
            output_path,
            raw_data: Vec::new(),
        }
    }
    // Main functions that invokes secondary functions... 
    pub fn run_cleaning_process(&mut self) -> Result<(), String> {
        self.load_data()?;
        self.clean_blank();  
        self.clean_name();   
        self.clean_date();   
        self.clean_system(); 
        self.save_data()
    }
    fn load_data(&mut self) -> Result<(), String> {
        // Read data from computer_games.csv
    }
    fn save_data(&self) -> Result<(), String> {
        // Save the changes on another clean computer_games_clean.csv
    }

    // Secondary functions
    fn clean_name(&self) -> Result<(), ()> {
        // This function is the second!
        // Iterate on which games names and remove duplicate games...
        todo!()
    }
    fn clean_system(&self) -> Result<(), ()> {
        // This function is the fourth!
        // Token. So, "Microsoft Windows" will be only "Windows"...
        // MacOS or macOS will be macOS
        // Linux stays same
        // Macintosh stays same
        todo!()
    }
    fn clean_date(&self) -> Result<(), ()>{
        // This function is the third!
        // "November 28, 1928" will be only "1928"...
        todo!()
    }
    fn clean_blank(&self) -> Result<(), ()>{
        // This function will be used first!
        // Remove games with blank lines...
        todo!()
    }
}