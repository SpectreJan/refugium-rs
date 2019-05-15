use all_strings;
use journal;

use std::io;
use std::collections::HashMap;

pub struct Guide {

    user_vec_ : HashMap<String, journal::Journal>,
    greetings_ : all_strings::Greeting,
    curr_user_ : String
}

impl Guide {

////////////////////////////////////////////////////////////////////////////////
    pub fn new() -> Guide {

        Guide {
            
            user_vec_ : HashMap::new(),
            greetings_ : all_strings::Greeting::new(),
            curr_user_ : String::new()
        }

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn init(&mut self)-> Result<(), u32> {


        if let Err(err) =  std::fs::create_dir("./users/")
        {
            // Usually fail in this case, but AlreadyExists gets a pass
            // cause this just means the user is not yet registered
            if err.kind() != std::io::ErrorKind::AlreadyExists
            {
                return Err(0)
            }
        }

        println!("{}", all_strings::REFUGIUM);
        println!("{}\n{}",
                self.greetings_.hello.trim(),
                self.greetings_.ask_name.trim());

        let mut curr_name = String::new();

        loop
        {
            if let Err(e) = io::stdin().read_line(&mut curr_name)
            {
                // Any Error here sucks cause without proper user name
                // no chicken dinner
                println!("Error Reading Username from keyboard: {}", e);
                return Err(1);
            }

            curr_name.pop();

            if curr_name.is_empty()
            {
                println!("I am sorry, an empty name is not a good name, choose a different one");
            }
            else
            {
                break;
            }
        }

        let files = std::fs::read_dir("./users/").unwrap();
        for file in files
        {
            // This looks complicated
            // Probably there is an easier rustonian way
            let file_name = file.unwrap().file_name();
            let file_str = file_name.to_str().unwrap();
            let mut file_string = file_str.to_string();

            let user_name_end = file_string.find('.').unwrap_or(file_string.len());
            file_string.truncate(user_name_end);
            let username = file_string.clone();

            let mut old_journal = journal::Journal::new(username.as_str());
            if let Err(e) = old_journal.init() {
                panic!("{}", e);
            }
            // Is this copy or move??
            self.user_vec_.insert(file_string, old_journal);
        }

        if self.user_vec_.contains_key(curr_name.as_str()) == false
        {
            println!("Hello {} {}", curr_name.trim(), self.greetings_.new_greet);
            let new_journal = journal::Journal::new(curr_name.as_str());
            self.user_vec_.insert(curr_name.to_string(), new_journal);
        }
        else
        {
            println!("Hello {} {}", curr_name.trim(), self.greetings_.old_greet);
        }

        self.curr_user_ = curr_name;

        Ok(())

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn run(&mut self) {

        let mut entry = String::new();

        let mut quit = false;
        
        Guide::help();

        while !quit
        {
            entry.clear();
            println!("\nWhat can I do for you my dear?");
            io::stdin().read_line(&mut entry).expect("");

            let entry_lower = entry.to_lowercase();

            // Additional Newline
            println!("");

            match entry_lower.trim().as_ref()
            {
                "goodbye"        => quit = Guide::quit(),
                "new thought"    => self.add_thought(),
                "old thoughts"   => self.get_thoughts(),
                "delete thought" => self.delete_thought(),
                "help"           => Guide::get_offers(),
                ""               => quit = Guide::quit(),
                _                => println!("{}", self.greetings_.unknown_wish)
            }

        }

        println!("{}", self.greetings_.goodbye);
        let curr_journal = self.user_vec_.get_mut(&self.curr_user_);
        curr_journal.unwrap().close();
    }

////////////////////////////////////////////////////////////////////////////////
    fn add_thought(&mut self) {

        // Get Category
        println!("{}", self.greetings_.ask_category);

        let mut category = String::new();
        io::stdin().read_line(&mut category).expect("");
        
        // Get thought
        println!("{}", self.greetings_.new_thought);

        let mut thought = String::new();
        io::stdin().read_line(&mut thought).expect("");

        let curr_user = self.user_vec_.get_mut(&self.curr_user_);
        curr_user.unwrap().add_entry(category.trim(),
                        thought.trim());
    }

////////////////////////////////////////////////////////////////////////////////
    fn get_thoughts(&mut self) {
        let curr_user = self.user_vec_.get_mut(&self.curr_user_);
        curr_user.unwrap().display_thoughts();
    }

////////////////////////////////////////////////////////////////////////////////
    fn delete_thought(&mut self) {
        let mut category = String::new();

        println!("You want to be relieved of a thought?\n\
                Tell me, in which category do we find it?");

        {
            let curr_user = self.user_vec_.get(&self.curr_user_);
            curr_user.unwrap().display_categories();
        
            io::stdin().read_line(&mut category).expect("");

            let ret = curr_user.unwrap().
                display_thoughts_in_category(category.as_str().trim());

            match ret {
               Ok(()) => (),
                Err(s) => {
                    println!("Error: {}", s);
                    return;
                }
            }
        }

        println!("What particular thought do you want to get rid of? Just tell me its number!");
        let mut thought = String::new();
        io::stdin().read_line(&mut thought).expect("");
        let t_number = thought.trim();
        if let Ok(i) = t_number.parse::<usize>() {
            let mut curr_user = self.user_vec_.get_mut(&self.curr_user_).unwrap();
            if let Err(e) = curr_user.delete_thought(category.as_str().trim(), i)
            {
                println!("{}", e);
            }
        }
        else
        {
            println!("this was not an integer: {}", t_number)
        }

    }
////////////////////////////////////////////////////////////////////////////////
    fn get_offers() {

        let offers = "We offer:\n\
        New Thought\n\
        Old thoughts\n\
        Goodbye";

        println!("{}", offers);
    }

////////////////////////////////////////////////////////////////////////////////
    fn quit() -> bool {
        return true
    }

////////////////////////////////////////////////////////////////////////////////
    fn help() {
        println!("{}", all_strings::HELP);
    }

}
