// modules3.rs

//  Solution by Tom Robertson (PapaTBR on github), feel free to review 
//  and comment as you see fit. Hope this helps!

// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Make me compile! Execute `rustlings hint modules3` for hints :)

// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
