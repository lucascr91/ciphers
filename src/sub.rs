const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

pub struct Caesar {
    pub key: i32
}

pub struct Vigenere {
    pub key: String
}

pub trait Translate {
    fn encrypt(&self, message: &str) -> String;
    fn decrypt(&self, message: &str) -> String;
    fn replace(&self, message: &str, mode: &str) -> String;
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

impl Translate for Vigenere {
    fn encrypt(&self, message: &str) -> String {
        self.replace(message, "encrypt")
    }

    fn decrypt(&self, message: &str) -> String {
        self.replace(message, "decrypt")
    }

    fn replace(&self, message: &str, mode: &str) -> String {
        let mut translate: Vec<char> = Vec::new();
        let mut key_index = 0;
        let key = self.key.to_lowercase();
        let message = message.to_lowercase();
    
        for symbol in message.chars() {
            let index_letter = LETTERS.find(symbol);
            match index_letter {
                Some(x) => {
                    let mut num:i32 = x as i32;
                    if mode == "encrypt" {
                        num+=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap() as i32;
                    } else if mode == "decrypt" {
                        num-=LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap() as i32;
                    }
                    num= (num).rem_euclid(LETTERS.chars().collect::<Vec<char>>().len() as i32);
                    translate.push(LETTERS.chars().nth(num as usize).unwrap());
                    key_index+=1;
                    if key_index == (key.chars().collect::<Vec<char>>().len()) {
                        key_index = 0;
                    }
                }
                None =>  {
                    translate.push(symbol)
                }
            }
        }
        let result: String = translate.into_iter().collect();
        result
    }
}