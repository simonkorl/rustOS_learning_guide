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

fn test_move() {
    const MOVE_UP: usize = 0;
    const MOVE_DOWN: usize = 1; 
    const MOVE_LEFT: usize = 2;
    const MOVE_RIGHT: usize = 3;

    const MOVE_X: [i32; 4] = [0, 0, -1, 1];
    const MOVE_Y: [i32; 4] = [1, -1, 0, 0];

    let mut pos = (1, 1);

    // move left
    pos.0 = pos.0 + MOVE_X[MOVE_LEFT];
    pos.1 = pos.1 + MOVE_Y[MOVE_LEFT];
    println!("move left: {:?}", pos);
    assert_eq!(pos, (0, 1));
    
    // move up
    pos.0 = pos.0 + MOVE_X[MOVE_UP];
    pos.1 = pos.1 + MOVE_Y[MOVE_UP];
    println!("move up: {:?}", pos);
    assert_eq!(pos, (0, 2));

    // move down
    pos.0 = pos.0 + MOVE_X[MOVE_DOWN];
    pos.1 = pos.1 + MOVE_Y[MOVE_DOWN];
    println!("move down: {:?}", pos);
    assert_eq!(pos, (0, 1));

    // move right
    pos.0 = pos.0 + MOVE_X[MOVE_RIGHT];
    pos.1 = pos.1 + MOVE_Y[MOVE_RIGHT];
    println!("move right: {:?}", pos); 
    assert_eq!(pos, (1, 1));

    println!("move test passed!");
}

#[derive(Debug)]
struct Attribute {
    Hp: i32,
    Atk: i32,
    Def: i32,
    Spd: i32
}

#[derive(Debug)]
struct Character<'a> {
    name: &'a str,
    attr: Attribute
}

fn create_character() {
    let chara = Character {
        name: "Daniel",
        attr: Attribute {
            Hp: 50,
            Atk: 21,
            Def: 29,
            Spd: 50
        }
    };
    println!("My character is: {:?}", chara);
}
fn main() {
    println!("This is my dungeon game!");
    print_background_story();
    create_character();
    map_test();
    test_move();
    print_credits();
}