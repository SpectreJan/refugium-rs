# refugium-rs
Weird Little Personal Journal Project

## Reason
This is a little project I used to learn a bit of rust <br>
At the same time I was aiming create something to help the self-healing process when battling personal demons.
Therfor the journal application is set to guide the person through the whole process of writing down a single thought and to revisit old thoughts
rather than just a bunch of commands.

## State
The App is really really Pre-Alpha. Usage is pretty much limited to writing down unstructured thoughts and listing all old thoughts in chronological order.
There is also some serious dead code, e.g. journal.rs is useless right now as I am in the middle of a rewrite.

## Prerequisites
1. cargo
2. rustc > 1.29

That should be it for the moment.
```
cargo run
```
...and you are good to go.

## Usage
```
cargo run
```
to start the program. First you need to enter your name. Refugium will check whether you used it before and greet you accordingly.
You can now add a new thought by typing
```
new thought
```
Old Thoughts are retrieved with...you guessed it...typing
```
old thoughts
```
The help menu is shown when you ask 
```
help
```

Pressing **enter** or typing `Goodbye` will end the program

## Further Development
No Promises are made...I might get bored or scared by the project and it might succumb to the slow death of repository rot!
But a better journal with categories would be cool. And encrypted files? Who knows!
