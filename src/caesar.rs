#![allow(dead_code)]

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

pub struct Caesar {
    pub key: i32
}

pub trait Translate {
    fn encrypt(&self, message: &str) -> String;
    fn decrypt(&self, message: &str) -> String;
    pri fn replace(&self, message: &str, mode: &str) -> String;
}

impl Translate for Caesar {
    fn encrypt(&self, message: &str) -> String {
        self.replace(message, "encrypt")
    }

    fn decrypt(&self, message: &str) -> String {
        self.replace(message, "decrypt")
    }

    fn replace(&self, message: &str, mode: &str) -> String {
        let mut result: Vec<char> = Vec::new();
        let message = message.to_lowercase();
        for letter in message.chars() {
            if LETTERS.contains(letter) {
                if mode=="encrypt" {
                    let new_letter = std::char::from_u32((((letter as u32) as i32 + self.key-97) as u32)%26 + 97);
                    assert!(new_letter.is_some(), "Cannot find the unicode value for {}",letter);
                    result.push(new_letter.unwrap());
                } else if mode=="decrypt" {
                    let new_letter = std::char::from_u32((((letter as u32) as i32 - self.key-97) as u32)%26 + 97);
                    assert!(new_letter.is_some(), "Cannot find the unicode value for {}",letter);
                    result.push(new_letter.unwrap());
                }
            } else {
                result.push(letter)
            }
        }
        let translation: String = result.into_iter().collect();
        translation
    }
}