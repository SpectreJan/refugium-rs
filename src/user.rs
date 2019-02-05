use std::io::Write;
use std::io::BufRead;
use std::fs::OpenOptions;

pub struct User {
    name_ : String,
    thought_ : Vec<String>,
}

#[allow(dead_code)]
impl User {

////////////////////////////////////////////////////////////////////////////////
    pub fn new(name : &str) -> User {

        User {
            name_ : name.to_string(),
            thought_ : Vec::new(),
        }

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn init(&mut self) {

        let file_op = OpenOptions::new().
            read(true).
            create(false).
            open(format!("./users/{}.txt", self.name_.as_str()));

        if file_op.is_ok()
        {
            let file = std::io::BufReader::new(file_op.unwrap());
            for line in file.lines()
            {
                self.thought_.push(line.unwrap());
            }
        }
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn get_name(&self) -> &str {
        return self.name_.as_str();
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn add_thought(&mut self, thought : &str) {

        let file = OpenOptions::new().
            read(true).
            write(true).
            append(true).
            create(true).
            open(format!("./users/{}.txt", self.name_.as_str()));
        
        let err = file.unwrap().write(format!("Thought: {}", thought).as_bytes());
        match err
        {

            Ok(_bytes) => self.thought_.push(thought.to_string()),
            Err(e) => println!("Sorry, but we could not log your thought due to: {:?}", e)
        }

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn get_thought(&self, index : usize) -> &str {
        return self.thought_.get(index).unwrap().as_str();
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn get_thoughts(&self) {
        for thought in self.thought_.iter()
        {
            println!("{}", thought);
        }
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn get_num_thoughts(&self) -> usize {
        return self.thought_.len();
    }
        
}
