//use serde::{Serialize, Deserialize};
use serde_json;

use std::io::Write;
use std::fs::OpenOptions;

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// JournalCategory
////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
struct JournalCategory {
    name: String,
    thoughts: Vec<String>
}

////////////////////////////////////////////////////////////////////////////////
// JournalEntry
////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
struct JournalEntry {
    user: String,
    categories: HashMap<String, JournalCategory>
}


////////////////////////////////////////////////////////////////////////////////
// Journal that keeps track of a users thought via JSON structures
////////////////////////////////////////////////////////////////////////////////
pub struct Journal {
    user_ : String,
    data_ : JournalEntry
}

#[allow(dead_code)]
impl Journal {

////////////////////////////////////////////////////////////////////////////////
    pub fn new(user : &str) -> Journal {

        Journal {

            user_ : user.to_string(),
            data_ : JournalEntry{user: user.to_string(),
                categories: HashMap::new()}

        }

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn close(&mut self) {

        let file_op = OpenOptions::new().
            write(true).
            create(true).
            open(format!("./users/{}.json", self.user_.as_str()));
        
        if file_op.is_ok()
        {
            let mut file = file_op.unwrap();

            // Erase content of file
            if let Ok(())  = file.set_len(0)
            {
                
                let j = serde_json::to_string_pretty(&self.data_);
            

                if let Err(e) = file.write(j.unwrap().as_bytes())
                {
                    println!("Sorry, but we could not backup your thoughts due to: {:?}", e)
                }
            }
        }

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn init(&mut self) -> std::result::Result<(), String> {

        let file_op = OpenOptions::new().
            read(true).
            create(false).
            open(format!("./users/{}.json", self.user_.as_str()));
        
        if file_op.is_ok()
        {
            // Result of opening file was OK
            use std::io::Read;
            let mut file = std::io::BufReader::new(file_op.unwrap());
            let mut content = String::new();
            if let Ok(_size) = file.read_to_string(&mut content)
            {

                let parse_result: serde_json::Result<JournalEntry> = serde_json::from_str(&content);

                if parse_result.is_err()
                {
                    let e = parse_result.unwrap_err();
                    println!("Journal found, but could not parse content due to {}", e);
                }
                else
                {
                    self.data_ = parse_result.unwrap();
                }
            }

        }
        else
        {
            let e = file_op.unwrap_err();
            if e.kind() != std::io::ErrorKind::NotFound
            {
                return Err(format!("Could not read present journal of {} due to error {}", self.user_, e));
            }
        }

        Ok(())
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn get_username(&self) -> &str {
        return self.user_.trim();
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn add_entry(&mut self, category : &str, entry : &str) {
        
        if self.data_.categories.contains_key(category) == false
        {
            self.data_.categories.insert(category.to_string(), JournalCategory{name: category.to_string(), thoughts: Vec::new()});
        }

        let x = self.data_.categories.get_mut(category);
        x.unwrap().thoughts.push(entry.to_string());
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn display_thoughts(&self) {
    
        // Iterate over Categories
        println!("------------------------------------------------------------");
        let categories = &self.data_.categories;
        for (k,v) in categories.iter() {
            println!("{}:", k);
            // Iterate over values
            for t in v.thoughts.iter() {
                println!("{}", t.trim());
            }
            println!("");
        }
        println!("------------------------------------------------------------");
    }

}
