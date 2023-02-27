use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn subtract(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}
fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn distance(a: (i32, i32), b: (i32, i32)) -> f64 {
    let heading = subtract(a, b);
    let dir = heading.0 * heading.0 + heading.1 * heading.1;
    return (dir as f64).sqrt();
}


fn getNorm(x: i32) -> i32 {
    if (x > 0) {
        return 1;
    } else if (x < 0) {
        return -1;
    } else {
        return 0;
    }
}

fn printBoard(seen: &HashSet<(i32, i32)>) {
    for i in (-50..50).rev() {
        let g = (-50..100).map(|x| if seen.contains(&(x, i)) { '#'} else {'.'});
        let s = String::from_iter(g);
        println!("{}", s)
    }
}

fn printState(head: (i32, i32), seen: &Vec<(i32, i32)>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let offset = 4;
    let dist = 8;

    for i in -offset..offset {
        let inner: Vec<char> = (-10..10).map(|x| '.').collect();
        grid.push(inner);
    }
    let mut t: u32 = 10;
    for i in seen.iter().rev() {
        t -= 1;
        grid[dist-(offset + i.1) as usize][(offset + i.0) as usize] = char::from_digit(t, 10).unwrap();
    }
    grid[dist -(offset + head.1) as usize ][(offset + head.0) as usize] = 'H';
    for i in grid.iter() {
        println!("{}", String::from_iter(i));
    }
}

fn getNewTailPos(headPos: (i32, i32), tailPos: (i32, i32)) -> (i32, i32) {

    let surroundingOffsets = vec![(1, 1), (-1, -1), (1, -1), (-1, 1)];
    let g = subtract(headPos, tailPos);
    let distFloat = distance(headPos, tailPos);
    let dist = distFloat as i32;

    if (dist == 2) {
        if headPos.0 == tailPos.0  {
            return add(tailPos, (0, getNorm(g.1)));

        } else if (headPos.1 == tailPos.1) {
            return add(tailPos, (getNorm(g.0), 0));
        } else {

            let mut closestOffset = surroundingOffsets[0].clone();
            let mut closestDist: f64 = 10000000.0;
            for offset in surroundingOffsets.iter() {
                let currentDist = distance(add(tailPos, offset.clone()), headPos);
                if (currentDist < closestDist) {
                    closestDist = currentDist;
                    closestOffset = offset.clone();
                }
            }
            return add(tailPos, closestOffset);
        }
    }
    return tailPos;
}

fn round1() {
    let data = fs::read_to_string("input.txt").expect("Can't read input file");

    let mut tailPositions = HashSet::new();
    let mut tailPos = (0, 0);
    let mut headPos = (0, 0);

    let surroundingOffsets = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    tailPositions.insert(tailPos);

    for line in data.trim().split("\n") {
        let gs: Vec<&str> = line.trim().split(" ").collect();
        let dir = gs[0];
        let amount = gs[1].parse::<i32>().unwrap();
        for m in 0..amount {

            match dir {
                "U" => headPos = (headPos.0, headPos.1 + 1),
                "D" => headPos = (headPos.0, headPos.1 - 1),
                "L" => headPos = (headPos.0 - 1, headPos.1),
                "R" => headPos = (headPos.0 + 1, headPos.1),
                _ => {}

            }

            tailPos = getNewTailPos(headPos, tailPos);
            tailPositions.insert(tailPos);
        }
    }

    println!("Round 1 {}", tailPositions.len());
}


fn round2() {
    let data = fs::read_to_string("input.txt").expect("Can't read input file");

    let mut tailPositions = HashSet::new();
    //let mut tailPos = (0, 0);
    let mut headPos = (0, 0);

    let mut tailList: Vec<(i32, i32)> = (0..9).map(|i| (0, 0)).collect();
    println!("len tail list: {}", tailList.len());

    for line in data.trim().split("\n") {
        let gs: Vec<&str> = line.trim().split(" ").collect();
        let dir = gs[0];
        let amount = gs[1].parse::<i32>().unwrap();
        for m in 0..amount {

            match dir {
                "U" => headPos = (headPos.0, headPos.1 + 1),
                "D" => headPos = (headPos.0, headPos.1 - 1),
                "L" => headPos = (headPos.0 - 1, headPos.1),
                "R" => headPos = (headPos.0 + 1, headPos.1),
                _ => {}
            }

            for i in 0..tailList.len() {
                if (i == 0) {
                    tailList[i] = getNewTailPos(headPos, tailList[i]);
                } else {
                    tailList[i] = getNewTailPos(tailList[i - 1], tailList[i]);
                }
            }
            println!("{:?}, {:?}", headPos, tailList);

            tailPositions.insert(tailList[tailList.len() - 1]);
        }
    }


    println!("Round 2 {}", tailPositions.len());

}

fn main() {
    round1();
    round2();
}
