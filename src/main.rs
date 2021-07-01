mod transp;

use transp::Columnar;

fn main() {
    let cypher = Columnar {
        key: 8,
    };

    // let message = String::from("Behind every great fortune there is a crime.");
    let message = String::from("T  eethachr e riywrgin eermdga ee rlia eetstbvah");
    println!("{}", cypher.decrypt(&message));
    // println!("{}", cypher.encrypt(&message));
}
