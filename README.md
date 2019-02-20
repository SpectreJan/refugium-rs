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
Refugium will then prompt you to enter a category name for this thought. If a category with that name already exists for your Journal, it will automatically be added to that category. <br>

Old Thoughts are retrieved with...you guessed it...typing
```
old thoughts
```

A thought can be deleted with
```
delete thought
```
Refugium will prompt you for the category name, where it can find the name. It then lists all available thoughts in that category. You choose the appropriate thought by index number (starting with 1 for all you hacker nerds that like to start at 0)

The help menu is shown when you ask 
```
help
```

Pressing **enter** or typing `Goodbye` will end the program

## Further Development
No Promises are made...I might get bored or scared by the project and it might succumb to the slow death of repository rot!
The Journal now has categories. Which is nice. But encrypted files? Who knows! Maybe later. If any poor soul has found its way to this README. I have no Idea about crypto and for sure need help...I was (naively) thinking (10 sec coherent thoughts max)

Encryption:

1. User enters passwd
2. Refugium generates random salt
3. Refugium generates key (libsodium pwhash)
4. User Data gets CRC
5. User Data is encrypted (libsodium secretbox)
6. Salt (plaintext) + User Data (encrypted) + CRC (encrypted? plaintext? Who cares?) is stored in File
7. Refugium keeps key till it exits? Does it need a session timeout? 

Decryption:

1. User enters passwd
2. Refugium reads salt
3. Refugium tries to decrypt the data
4. Refugium checks CRC of encrypted data
5. If CRC is correct, it seems to have worked
6. If CRC is wrong, tough luck.

Let me know in the issue section why this idea sucks balls, or is brilliant, or is ordinary. As long as I learn something I am happy.
