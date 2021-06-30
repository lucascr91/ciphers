pub struct Columnar {
    pub key: usize,
}

impl Columnar {
    pub fn encrypt(&self, message: &str) -> String {
        let mut cipher_text:Vec<char> = Vec::new();

        for column in 0..self.key {
            let mut current_index: usize = column;
            while current_index< message.len() {
                cipher_text.push(message.chars().nth(current_index).unwrap());
                current_index+=self.key;
            }
        }
        let result: String = cipher_text.iter().collect();
        result
    }

    pub fn decrypt(&self, message: &str) -> {
        
    }
}