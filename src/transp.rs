extern crate num_traits;

use num_traits::Float;

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

    pub fn decrypt(&self, message: &str) -> String {
        let num_of_cols = (Float::ceil((message.len() as f64)/(self.key as f64))) as i32;
        let num_of_rows = self.key as i32;
        let shaded = (num_of_cols*num_of_rows) - (message.len() as i32);
        let mut plain_text:Vec<Vec<char>>= vec![Vec::new();num_of_cols as usize];
        let mut column = 0;
        let mut row =0;

        for symbol in message.chars() {
            plain_text[column].push(symbol);
            column +=1;

            if (column == num_of_cols as usize) || (column==((num_of_cols as usize)-1) && row>=((num_of_rows as usize)-(shaded as usize))) {
                column = 0;
                row +=1;
            }
        }
        let mut words:Vec<String> = Vec::new();
        for i in 0..plain_text.len() {
            words.push(plain_text[i].clone().into_iter().collect());
        }
        let result: String = words.into_iter().collect();
        result
    }
}