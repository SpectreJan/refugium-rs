pub struct Journal {
    filename_ : String,
    categories_ : Vec<String>
}

#[allow(dead_code)]
impl Journal {

    pub fn new(user : &str) -> Journal {

        Journal {

            filename_ : format!("{}.txt", user),
            categories_ : Vec::new()

        }

    }

    pub fn add_category(&mut self, category : &'static str) {

        self.categories_.push(category.to_string());

    }

    pub fn get_filename<'a>(&self) -> &str {
        
        return self.filename_.trim();

    }

}
