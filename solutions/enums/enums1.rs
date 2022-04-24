// enums1.rs

//  Solution by Tom Robertson (PapaTBR on github), feel free to review 
//  and comment as you see fit. Hope this helps!

// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
   Quit,
   Echo,
   Move,
   ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
