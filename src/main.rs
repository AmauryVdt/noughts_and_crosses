mod party;
mod grid;

use party::Party;

fn main() {
    println!("Hello, world!");
    let party = Party::new();
    party.start();
}