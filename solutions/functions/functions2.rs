// functions2.rs

//  Solution by Tom Robertson (PapaTBR on github), feel free to review 
//  and comment as you see fit. Hope this helps!

// Make me compile! Execute `rustlings hint functions2` for hints :)



fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
