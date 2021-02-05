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

fn fake_create_character() {
    let name: &str = "Daniel";
    let hp = 50u32;
    let atk: i32 = 21;
    let def: u32 = 29u32;
    let spd = 50;
    println!("I created my character with: hp({}), atk({}), def({}), spd({}).", hp, atk, def, spd);
}

fn fake_create_boss() {
    // 敌人属性
    let boss_hp = 1_000_000;
    let boss_atk = 5000;
    let boss_def = 1000;
    let boss_spd = 100;
    let boss_stunnable = true;
    println!("I created a boss with: hp({}), atk({}), def({}), spd({}).", boss_hp, boss_atk, boss_def, boss_spd);
    // 战利品爆率
    let rare_rate = 0.2;
    let epic_rate = 0.04;
    let legendary_rate = 0.02;
    let commen_rate = 1.0 - rare_rate - epic_rate - legendary_rate;
}

fn map_test() {
    let pos = (1, 1);
    let door_pos = (2,2);
    assert!(pos.0 + 1 == door_pos.0 && pos.1 + 1 == door_pos.1);
    println!("map test passed!");
}

fn main() {
    println!("This is my dungeon game!");
    print_background_story();
    fake_create_character();
    fake_create_boss();
    map_test();
    print_credits();
}