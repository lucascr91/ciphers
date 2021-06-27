#![allow(dead_code)]

enum Key {
    Number,
    Word
}

struct Caesar {
    key: Key::Number
}

trait Translate {
    fn encrypt(&self) -> String;
    fn decrypt(&self) -> String;
}

impl Translate for Caesar {
    
}

fn main() {
    println!("Hello, world!");
}
