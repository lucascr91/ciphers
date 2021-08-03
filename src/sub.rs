const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz1234567890!?.";

pub struct Caesar {
    pub key: i32,
}

pub struct Vigenere {
    pub key: String,
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
                if mode == "encrypt" {
                    let new_letter = std::char::from_u32(
                        ((letter as i32 + self.key - 97).rem_euclid(26) + 97) as u32,
                    );
                    assert!(
                        new_letter.is_some(),
                        "Cannot find the unicode value for {}",
                        letter
                    );
                    result.push(new_letter.unwrap());
                } else if mode == "decrypt" {
                    let new_letter = std::char::from_u32(
                        ((letter as i32 - self.key - 97).rem_euclid(26) + 97) as u32,
                    );
                    assert!(
                        new_letter.is_some(),
                        "Cannot find the unicode value for {}",
                        letter
                    );
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
                    let mut num: i32 = x as i32;
                    if mode == "encrypt" {
                        num += LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap() as i32;
                    } else if mode == "decrypt" {
                        num -= LETTERS.find(key.chars().nth(key_index).unwrap()).unwrap() as i32;
                    }
                    num = (num).rem_euclid(LETTERS.len() as i32);
                    translate.push(LETTERS.chars().nth(num as usize).unwrap());
                    key_index += 1;
                    if key_index == (key.len()) {
                        key_index = 0;
                    }
                }
                None => translate.push(symbol),
            }
        }
        let result: String = translate.into_iter().collect();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGINAL_TEST: &str = "the quick brown fox jumped over the lazy dog";

    #[test]
    fn caesar_encrypt_decrypt() {
        let ins = Caesar { key: 3 };

        let encrypted = ins.encrypt(ORIGINAL_TEST);
        let decrypted = ins.decrypt(&encrypted);

        assert_eq!(encrypted, "wkh txlfn eurzq ira mxpshg ryhu wkh odcb grj");
        assert_eq!(decrypted, ORIGINAL_TEST);
    }

    #[test]
    fn vigenere_encrypt_decrypt() {
        let ins = Vigenere {
            key: "secret".into(),
        };

        let encrypted = ins.encrypt(ORIGINAL_TEST);
        let decrypted = ins.decrypt(&encrypted);

        assert_eq!(encrypted, "?lg 8y2uo d9sc6 jqb na5tgu sbwv vyi 5s41 usz");
        assert_eq!(decrypted, ORIGINAL_TEST)
    }
}
