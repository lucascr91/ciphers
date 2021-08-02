mod sub;

use std::fs;
use sub::Caesar;
use sub::Translate;

fn main() {
    let cypher = Caesar { key: 8 };
    let contents =
        fs::read_to_string("/home/lucas/projects/cyphers/crimeAndPunishment.encrypt.txt").unwrap();
    let encrypt_text = cypher.decrypt(&contents);
    let new_file = "/home/lucas/projects/cyphers/crimeAndPunishment.decrypt.txt";
    fs::write(new_file, encrypt_text).unwrap();
}
