mod caesar;

use caesar::Translate;
use caesar::Caesar;

fn main() {
    let cypher = Caesar {
        key: 4
    };

    let message = String::from("We do not learn, and that what we call learning is only a process of recollection.");
    // println!("{}", cypher.decrypt(&message));
    println!("{}", cypher.encrypt(&message));
}
