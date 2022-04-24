// functions3.rs

//  Solution by Tom Robertson (PapaTBR on github), feel free to review 
//  and comment as you see fit. Hope this helps!

// Make me compile! Execute `rustlings hint functions3` for hints :)


fn main() {
    call_me(4);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
