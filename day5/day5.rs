use std::fs;
use std::iter::FromIterator;

fn sameItems(macerator: fn(&Vec<usize>, &mut Vec<Vec<char>>)) -> String {
    let data = fs::read_to_string("input.txt").expect("Unable to read file.");
    let parts: Vec<&str> = data.split("\n\n").collect();

    let stacksRaw: Vec<&str> = parts[0].split("\n").collect();

    let mut stackLists = stacksRaw.iter().rev().fold(Vec::new(), |mut xs, x| {
        x.chars().collect::<Vec<char>>().chunks(4).enumerate().for_each(|(i, x)| {
            if xs.len() <= i { xs.push(Vec::new()); }
            if x[1] != ' ' { xs[i].push(x[1]) };
        });
        xs 
    });

    parts[1].trim().split("\n").for_each(|row| {
        let commands: Vec<usize> = row.trim().split(" ").skip(1).step_by(2).map(|x| { x.parse::<usize>().unwrap()}).collect();
        macerator(&commands, &mut stackLists);
    });

    return String::from_iter(stackLists.iter().map(|v| v.last().unwrap()));
}

fn round1() -> String {
    sameItems(|cs, stackLists| {
        for i in 0..cs[0] {
            let t = stackLists[cs[1]-1].len() - 1;
            let elementToAdd = stackLists[cs[1]-1][t];
            stackLists[cs[2]-1].push(elementToAdd);
            stackLists[cs[1]-1].remove(t);
        }
    })
}

fn round2() -> String {
    sameItems(|cs, stackLists| {
        let from = stackLists[cs[1] - 1].clone();
        stackLists[cs[2]-1].extend_from_slice(&from[(from.len() - cs[0])..from.len()]);
        stackLists[cs[1]-1].drain(from.len() - cs[0]..from.len());
    })
}

fn main() {
    println!("Round1: {}", round1());
    println!("Round2: {}", round2());
}