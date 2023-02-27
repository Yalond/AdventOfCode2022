use std::fs;
use std::collections::HashMap;


fn calculateResult(game: String) -> i32 {
    return 0;
}

fn round1() {
    let data = fs::read_to_string("input.txt").expect("can't open file");
    data.trim();
    let mut split = data.split("\n");

    let mut totalScoreOuter = 0;

    let winning = vec![('A', 'Y'), ('B', 'Z'), ('C', 'X')];
    let draw = vec![('A', 'X'), ('B', 'Y'), ('C', 'Z')];

    for line in split {

        if line != "" {
            let mut totalScore = 0;
            let mut other = line.chars().nth(0).unwrap();
            let mut player = line.chars().nth(2).unwrap();

            let p = (other, player);

            if winning.contains(&p) { totalScore += 6; }
            else if draw.contains(&p) { totalScore += 3;}


            if player == 'X' { totalScore += 1;}
            else if player == 'Y' { totalScore += 2;}
            else { totalScore += 3;}


            //println!("roundScore score: {}", totalScore.to_string());

            totalScoreOuter += totalScore;
        }
    }
    println!("Round 1: {}", totalScoreOuter.to_string());
}

fn round2() {
    let data = fs::read_to_string("input.txt").expect("can't open file");
    data.trim();

    let mut split = data.split("\n");
    let mut totalScore = 0;

    let winning = HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]);
    let draw = HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]);
    let loose = HashMap::from([('A', 'Z'), ('B', 'X'), ('C', 'Y')]);
    let score = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);


    for line in split {

        if line != "" {
            let mut other = line.chars().nth(0).unwrap();
            let mut shouldState = line.chars().nth(2).unwrap();
            let mut player = 'X';

            if shouldState == 'X' { // lose
                player = loose[&other]; 
            } else if shouldState == 'Y' { // draw
                player = draw[&other]; 
                totalScore += 3;
            } else { // win
                player = winning[&other]; 
                totalScore += 6;
            }
            totalScore += score[&player];
        }
    }
    println!("Round 2: {}", totalScore.to_string());
}

// what the fuck did you just

fn main() {
    round1();
    round2();
}
