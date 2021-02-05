//! This is my first rust project!
//! It is a developing dungeon game!
//! Please stay tuned and enjoy!

// producer name
const first_name: &str = "Simon";
const last_name: &str = "Korl";

fn print_credits() {
    println!("----- Credits ----");
    println!("Programmer: {0} {1}\nScript: {0} {1}", first_name, last_name); 
    println!("---------------");
}

fn print_background_story() {
    println!("----- Background Story -----");
    println!("You woke up in a deep dark cave.\nYou know you have to get out of here.");
    println!("---------------");
}

fn main() {
    println!("This is my dungeon game!");
    print_background_story();
    print_credits();
}