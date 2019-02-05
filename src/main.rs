mod all_strings;
mod user;
mod guide;

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
