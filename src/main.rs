mod caesar;

use caesar::Translate;
use caesar::Caesar;

fn main() {
    let cypher = Caesar {
        key: 4
    };

    let message = String::from("Fala malandro");
    // println!("{}", cypher.decrypt(&message));
    println!("{}", cypher.encrypt(&message));
}
