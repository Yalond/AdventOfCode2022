use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;




fn round1() {
    let data = fs::read_to_string("input.txt").expect("Unable to read input file.");

    let mut xRegister: i32 = 1;
    let mut currentCycle: i32= 1;
    let mut cycleStrength: i32 = 0;
    let mut targetCycles = HashSet::from([20, 60, 100, 140, 180, 220]);

    for line in data.trim().split("\n") {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        //println!("Parts: {:?}", parts);
        if parts[0] == "addx" {

            currentCycle += 1;
            println!("CurrentCycle: {}, xRegister: {}", currentCycle, xRegister);
            if targetCycles.contains(&currentCycle) {
                cycleStrength += xRegister * currentCycle
                
            }

            currentCycle += 1;

            xRegister += parts[1].parse::<i32>().unwrap();

            println!("CurrentCycle: {}, xRegister: {}", currentCycle, xRegister);
            if targetCycles.contains(&currentCycle) {
                cycleStrength += xRegister * currentCycle
            }

        } else if parts[0] == "noop" {
            currentCycle += 1;

            println!("CurrentCycle: {}, xRegister: {}", currentCycle, xRegister);
            if targetCycles.contains(&currentCycle) {
                cycleStrength += xRegister * currentCycle
            }

        }
   }
    println!("Round 1: {}", cycleStrength);
}

fn updateCycle(currentCycle: i32, x: i32, arr: &mut Vec<char>) {
    let i = (currentCycle - 1) %40;
    if i >= (x - 1) && i <= (x + 1) {
        arr[i as usize] = '#';
    }
}

fn round2() {
    let data = fs::read_to_string("input.txt").expect("Unable to read input file.");

    let mut xRegister: i32 = 1;
    let mut currentCycle: i32= 1;
    let mut cycleStrength: i32 = 0;
    let mut targetCycles = HashSet::from([40, 80, 120, 160, 200, 240]);

    let mut currentRow: Vec<char> = (0..40).map(|x| '.').collect();
    let mut allRows = Vec::new();

    for line in data.trim().split("\n") {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        //println!("Parts: {:?}", parts);
        if parts[0] == "addx" {

            println!("CurrentCycle: {}, xRegister: {}", currentCycle, xRegister);
            updateCycle(currentCycle, xRegister, &mut currentRow);
            if targetCycles.contains(&currentCycle) {
                allRows.push(String::from_iter(currentRow));
                currentRow = (0..40).map(|x| '.').collect();
                
            }
            currentCycle += 1;


            println!("CurrentCycle: {}, xRegister: {}", currentCycle, xRegister);
            updateCycle(currentCycle, xRegister, &mut currentRow);
            if targetCycles.contains(&currentCycle) {
                allRows.push(String::from_iter(currentRow));
                currentRow = (0..40).map(|x| '.').collect();
            }

            currentCycle += 1;
            xRegister += parts[1].parse::<i32>().unwrap();

        } else if parts[0] == "noop" {

            println!("CurrentCycle: {}, xRegister: {}", currentCycle, xRegister);
            updateCycle(currentCycle, xRegister, &mut currentRow);
            if targetCycles.contains(&currentCycle) {
                allRows.push(String::from_iter(currentRow));
                currentRow = (0..40).map(|x| '.').collect();
            }

            currentCycle += 1;

        }
    }
    println!("Round 2");
    for line in allRows {
        println!("{}", line);
    }
}

fn main() {
    round1();
    round2();

}