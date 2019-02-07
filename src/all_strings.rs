pub struct Greeting {
    pub hello : &'static str,
    pub ask_name : &'static str,
    pub old_greet : &'static str,
    pub new_greet : &'static str,
    pub goodbye : &'static str,
    pub new_thought : &'static str,
    pub unknown_wish : &'static str,
    pub ask_category: &'static str
}

impl Greeting {
    pub fn new() -> Greeting {

        Greeting {
            hello : "Hello weary traveler!",
            ask_name : "How may I call you?",
            old_greet : "old friend, great to see you again",
            new_greet : "new friend, may you feel warm and welcome here",
            goodbye : "Goodbye my friend. We'll miss you, \
            but may all the fortunes be with you",
            new_thought : "What's on your mind?",
            unknown_wish: "Sorry, we did dot understand you :(\nFor a list of services just type help!",
            ask_category: "In which category does your thought belong to?"
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Static strings
////////////////////////////////////////////////////////////////////////////////
pub static REFUGIUM : &'static str =
        "\
###########################################################\n\
## REFUGIUM                                              ##\n\
###########################################################";

pub static HELP : &'static str = "Here are the services refugium offers:\n\n\
                   help         -> Lists all services\n\
                   New thought  -> Add a new thought to your journal\n\
                   Old thoughts -> Shows your old thoughts inside your journal\n\
                   Goodbye      -> With this you can leave the refugium";
