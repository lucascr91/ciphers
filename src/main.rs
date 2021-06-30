mod sub;

use sub::Translate;
use sub::Vigenere;

fn main() {
    let cypher = Vigenere {
        key: String::from("PIZZA")
    };

    let message = String::from("We do not learn, and that what we call learning is only a process of recollection.");
    // println!("{}", cypher.decrypt(&message));
    println!("{}", cypher.encrypt(&message));
}
