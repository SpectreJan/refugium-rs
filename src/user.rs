use journal::Journal;

pub struct User {
    name_ : String,
    journal_ : Journal
}

#[allow(dead_code)]
impl User {

////////////////////////////////////////////////////////////////////////////////
    pub fn new(name : &str) -> User {

        User {
            name_ : name.to_string(),
            journal_ : Journal::new(name)
        }

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn init(&mut self) {
        if let Err(e) = self.journal_.init()
        {
            panic!("{}", e);
        }
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn get_name(&self) -> &str {
        return self.name_.as_str();
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn add_thought(&mut self, thought : &str, category : &str) {

        self.journal_.add_entry(category, thought);

    }

////////////////////////////////////////////////////////////////////////////////
    pub fn get_thoughts(&self) {
        self.journal_.display_thoughts();
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn close_journal(&mut self) {
        self.journal_.close();
    }
        
}
