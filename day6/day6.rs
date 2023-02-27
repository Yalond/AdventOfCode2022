use std::{fs, collections::HashSet};

fn checker(count: usize) -> usize {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    for (i, chars) in data.chars().collect::<Vec<char>>().windows(count).enumerate() {
        if chars.iter().collect::<HashSet<&char>>().len() == chars.len() { return  i + count }
    }
    return 0;
}

fn main() {
    println!("Round 1: {}", checker(4));
    println!("Round 2: {}", checker(14));
}