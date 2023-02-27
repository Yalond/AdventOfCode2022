use std::fs;

fn round1() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    const RADIX: u32 = 10;

    let mut grid = data.trim().split("\n").map(|line| {
        return line.trim().chars().map(|c| (c.to_digit(RADIX).unwrap() as i32, false)).collect();
    }).collect::<Vec<Vec<(i32, bool)>>>();

    let w = grid.len() - 1;
    let h = grid[0].len() - 1;

    for i in 0..grid.len() {
        let mut heights = vec![-1,-1,-1,-1];
        for j in 0..grid[1].len() {
            if (grid[i][j].0 > heights[0]) {
                grid[i][j].1 = true;
                heights[0] = grid[i][j].0;
            }

            if (grid[i][w-j].0 > heights[2]) {
                grid[i][w-j].1 = true;
                heights[2] = grid[i][w-j].0;
            }
        }
    }

    for j in 0..grid[0].len() {
            let mut heights = vec![-1,-1,-1,-1];
            for i in 0..grid.len() {

            if (grid[i][j].0 > heights[1]) {
                grid[i][j].1 = true;
                heights[1] = grid[i][j].0;
            }

            if (grid[h-i][j].0 > heights[3]) {
                grid[h-i][j].1 = true;
                heights[3] = grid[h-i][j].0;
            }
        }
    }

    for line in &grid {

        println!("grid: {:?}", line);
    }

    let totalCount: i32 = grid.iter().flat_map(|x|  x.iter().map(|x| x.1 as i32)).sum();

    println!("Round 1: {}", totalCount);
}

fn getSlice<T>(grid: &Vec<Vec<T>>, start: (usize, usize), end: (usize, usize)) -> Vec<T> {
    let mut result = Vec::new();
    if (start.0 == end.0) {
        for i in start.1..end.1 {
            result.push(grid[start.0][i]);
        }
    } else if (start.1 == end.1) {
        for i in start.0..end.0 {
            result.push(grid[i][start.0]);
        }
    } else {
        //throw Error("Can only slice rows and columns, not diagonals.");
        return result;
    }
}

fn getScenicScore(grid: &Vec<Vec<(i32, i32)>>, i: i32, j: i32) -> i32 {
    let startHeight = grid[i as usize][j as usize].0;
    let mut gridWidth = grid[0].len() as i32;
    let mut gridHeight = grid.len() as i32;

    let mut scores = vec![0,0,0,0];

    let mut offset = j;
    while true {
        offset += 1;
        if (offset >= gridWidth) {
            scores[0] = offset - j - 1;
            break;
        } 
        if (grid[i as usize][offset as usize].0 >= startHeight) {
                scores[0] = offset - j;
                break;
        }
    }

    let mut offset = j;
    while true {
        offset -= 1;
        if (offset < 0) {
            scores[1] = j - offset - 1;
            break;
        }
        if grid[i as usize][offset as usize].0 >= startHeight {
            scores[1] = j - offset;
            break;
        }
    }

    let mut offset = i;
    while true {
        offset += 1;
        if (offset >= gridHeight) {
            scores[2] = offset - i - 1;
            break;
        }
        if (grid[offset as usize][j as usize].0 >= startHeight) {
            scores[2] = offset - i;
            break;
        }
    }

    let mut offset = i;
    while true {
        offset -= 1;
        if (offset < 0) {
            scores[3] = i - offset - 1;
            break;
 
        }
        if (grid[offset as usize][j as usize].0 >= startHeight) {
            scores[3] = i - offset;
            break;
        }
    }
    println!("i: {}, j: {}, scores: {:?}", i, j, scores);
    return scores[0] * scores[1] * scores[2] * scores[3];

}

fn round2() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    const RADIX: u32 = 10;

    let mut grid = data.trim().split("\n").map(|line| {
        return line.trim().chars().map(|c| (c.to_digit(RADIX).unwrap() as i32, 0)).collect();
    }).collect::<Vec<Vec<(i32, i32)>>>();

    let w = grid.len() - 1;
    let h = grid[0].len() - 1;

    for i in 0..grid.len() {
        for j in 0..grid[1].len() {
            grid[i][j].1 = getScenicScore(&grid, i as i32, j as i32) as i32;
        }
    }

    let totalCount: i32 = grid.iter().flat_map(|x|  x.iter().map(|x| x.1)).max().unwrap();

    for line in &grid {

        println!("grid: {:?}", line);
    }

    println!("Round 2: {}", totalCount);

}

fn main() {

    round1();
    round2();

}