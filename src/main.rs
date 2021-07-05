mod sub;

use sub::Caesar;
use sub::Translate;
use std::fs;

fn main() {
    let cypher = Caesar {
        key: 8,
    };
    let contents = fs::read_to_string("/home/lucas/projects/cyphers/crimeAndPunishment.txt").unwrap();
    let encrypt_text = cypher.encrypt(&contents);
    let new_file = "/home/lucas/projects/cyphers/crimeAndPunishment.encrypt.txt";
    fs::write(new_file, encrypt_text).unwrap();
}
