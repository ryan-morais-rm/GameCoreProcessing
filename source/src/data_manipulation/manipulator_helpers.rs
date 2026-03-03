pub enum CountData {
    StringData(Vec<(String, u32)>),
    NumberData(Vec<(u16, u32)>),
}

pub fn format_counts(data: CountData) -> String {
    match data {
        CountData::StringData(mut count_vec) => {
            count_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            let mut result = String::from("There are these systems:\n");
            result.push_str("------------------\n");
            
            for (system_name, quantity) in count_vec {
                result.push_str(&format!("{} - {}\n", system_name, quantity));
            }
            
            result 
        },
        
        CountData::NumberData(mut count_vec) => {
            count_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            let mut result = String::from("Games released per year:\n");
            result.push_str("------------------\n");
            
            for (year, quantity) in count_vec {
                if year == 0 {
                    result.push_str(&format!("Year (0000) - {} games\n", quantity));
                } else {
                    result.push_str(&format!("{} - {} games\n", year, quantity));
                }
            }
            
            result
        }
    }
}