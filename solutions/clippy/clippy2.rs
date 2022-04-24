// clippy2.rs

//  Solution by Tom Robertson (PapaTBR on github), feel free to review 
//  and comment as you see fit. Hope this helps!

// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    while let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
