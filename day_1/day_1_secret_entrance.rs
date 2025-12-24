use std::fs;

fn main() {
    println!("Calculating the password...");
    let mut value: i32 = 50;

    let contents = fs::read_to_string("day_1_input.txt").expect("Could not read file");
    let mut counter = 0;
    for line in contents.lines() {
        let rotation = calc_new_value(line);
        println!(
            "Parsed value {}, initial value {}, final value {}",
            rotation,
            value,
            (value + rotation) % 99
        );
        value = (value + rotation) % 100;
        if value == 0 {
            counter += 1;
        }
        // println!("new line: {}", calc_new_value(line));
    }
    println!("Counter {}", counter)
}

fn get_rotation(str: &str) -> i32 {
    let mut chars = str.chars();
    chars.next(); // skip first character

    let rest: String = chars.collect();
    let number: i32 = rest.parse().expect("Not a number");

    return number;
}

fn calc_new_value(str: &str) -> i32 {
    let mut way: bool = false;
    if str.starts_with("R") {
        way = true;
    }

    let rotation: i32 = get_rotation(str);

    if way {
        return rotation;
    } else {
        return -rotation;
    }
}
