use std::fs;
use std::collections::BinaryHeap;


fn topNItems(n: usize) -> i32 {

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut split = data.split("\n\n");
    let mut heap = BinaryHeap::new();

    for line in split {
        let nums = line.split("\n");
        let mut currentTotal = 0; 

        for num in nums {
            match num.parse::<i32>() {
                Ok(n) => currentTotal += n,
                Err(e) => println!("Can't add numbers"),
            }
        }

        heap.push(currentTotal);
    }

    let mut total = 0;
    for i in 0..n {
        total += heap.pop().unwrap();
    }
    return total;
}

fn main() {
    println!("top 1 items: {}", topNItems(1).to_string());
    println!("top 3 items: {}", topNItems(3).to_string());
}

