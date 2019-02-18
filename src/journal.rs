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
        let mut c_count: u8 = 1;
        println!("------------------------------------------------------------");
        let categories = &self.data_.categories;
        for (k,v) in categories.iter() {
            println!("{}. {}:", c_count, k,); 
            c_count += 1;
            Journal::list_thoughts_in_category(v);
        }
        println!("------------------------------------------------------------");
    }
    
////////////////////////////////////////////////////////////////////////////////
    pub fn display_categories(&self) {
    
        // Iterate over Categories
        let mut c_count: u8 = 1;
        println!("------------------------------------------------------------");
        let categories = &self.data_.categories;
        for category_name in categories.keys() {
            println!("{}. {}", c_count, category_name); 
            c_count += 1;
        }
        println!("------------------------------------------------------------");
    }
////////////////////////////////////////////////////////////////////////////////
    pub fn display_thoughts_in_category(&self, category : &str)
    {
        let data: &JournalEntry = &self.data_;
        let current_category = data.categories.get(category);

        match current_category {
            Some(ref cat) => Journal::list_thoughts_in_category(cat),
            None => println!("Sorry, You do not have entries for this category!"),
        }
    }
////////////////////////////////////////////////////////////////////////////////
    fn list_thoughts_in_category(category : &JournalCategory) {
        let mut t_count: u16 = 1;
        let thoughts = &category.thoughts;
        for t in thoughts {
            println!("  {}. {}", t_count, t.trim());
            t_count += 1;
        }
        println!("");
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn delete_thought(&mut self, category : &str, entry : usize) -> Result<(), String> {

        let mut num_thoughts: usize;

        if entry == 0
        {
            return Err("No Entry zero recorded".to_string());
        }
           
        if self.data_.categories.contains_key(category) == true
        {

            let current_cat = self.data_.categories.get_mut(category).unwrap();
            
            num_thoughts = current_cat.thoughts.len();
            if entry <= num_thoughts
            {
                current_cat.thoughts.remove(entry-1);
                num_thoughts -= 1;
            }
            else
            {
                return Err(format!("Entry {} out of range for thoughts in category {}", entry, category));
            }
            
        }
        else
        {
            return Err(format!("I am sorry, you have not recorded a category named {} yet", category));
        }

        if num_thoughts == 0
        {
            self.data_.categories.remove(category);
        }

        Ok(())

    }

}
