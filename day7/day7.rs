use std::fs;
use std::collections::HashMap;

fn process(startIndex: usize, arr: & Vec<&str>) -> (usize, i32, Vec<i32>) {

    let mut index = startIndex;
    let mut sizes = Vec::new();
    let mut currentFolderSize = 0;

    while index < arr.len() {

        let split: Vec<&str> = arr[index].trim().split(" ").collect();
        index += 1;

        if split[0] == "$" {

            if split[1] == "cd" && split[2] == ".." { 
                return (index, currentFolderSize, sizes);

            } else if (split[1] == "cd") {
                let (rCount,subFolderSize, rMap) = process(index + 1, arr);
                index = rCount;
                sizes.push(subFolderSize);
                currentFolderSize += subFolderSize;
                sizes.extend(rMap);
            }

        } else if split[0] != "dir" {
            currentFolderSize += split[0].parse::<i32>().unwrap();
        }
    }

    return (index, currentFolderSize, sizes);
}

fn main() {

    let data = fs::read_to_string("input.txt").expect("Can't read input file");
    let rows: Vec<&str> = data.trim().split("\n").collect();
    let (index, totalDirSize, allDirs) = process(0, &rows);

    let res: i32 = allDirs.iter().filter(|x| **x < 100000).sum();
    println!("Round 1: {}", res);

    let totalSize = 70000000;
    let unusedSpace = 30000000;
    let neededSpace = unusedSpace - (totalSize - totalDirSize);

    let res2 = allDirs.iter().filter(|dir| *dir > &neededSpace).min().unwrap();

    println!("Round 2: {}", res2);
    
}