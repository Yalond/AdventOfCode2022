use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;


fn getPriority(character: char) -> i32 {
    let lowcaseOffset = 97;
    let upcaseOffset = 65 - 26;
    //println!("Character: {}", character);
    let mut charVal = character as i32;
    if character.is_lowercase() {
        charVal -= lowcaseOffset;
    } else {
        charVal -= upcaseOffset;
    }
    charVal += 1;
    return charVal;
}

fn round1() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    
    let mut split = data.trim().split("\n");
    let mut totalCount = 0;
    for line in split {
        let (first, last) = line.split_at(line.chars().count()/2);
        for character in first.chars() {
            if last.contains(character) {
                totalCount += getPriority(character);
                break;
            }
        }
    }
    println!("Round 1: {}", totalCount);

}

fn round2() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let h:i32 = data.trim().split("\n").collect::<Vec<&str>>().iter()
            .map(|x| HashSet::from_iter(x.chars()))
            .collect::<Vec<HashSet<char>>>()
            .chunks(3)
            .map(|x| {
                return getPriority(*HashSet::from_iter(x[0].intersection(&x[1]).copied())
                        .intersection(&x[2]).next().unwrap());
            }).sum();
 
    println!("Round 2: {}", h);

}

fn main() {
    round1();
    round2();
    

}
