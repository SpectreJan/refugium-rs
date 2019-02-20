mod all_strings;
mod guide;
mod journal;
mod fstorage;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

fn main() {
    let mut guide = guide::Guide::new();
    
    match guide.init()
    {
        Err(e) => match e {
            0 => panic!("Could not create User directory"),
            1 => panic!("Could not read from keyboard"),
            _ => panic!("Unknown error: {}", e)
        }
        Ok(()) => guide.run()
    }
}
