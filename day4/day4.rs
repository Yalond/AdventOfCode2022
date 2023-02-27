use std::fs;

fn looper(f: fn(a: i32, b: i32, c: i32, d: i32) -> bool) -> i32 {
    let data = fs::read_to_string("input.txt").expect("Unable to read file.");
    return data.trim().split("\n").map(|x| {
        let runs: Vec<i32> = x.split(&[',', '-'][..]).map(|x| x.parse::<i32>().unwrap()).collect();
        (f(runs[0], runs[1], runs[2], runs[3]) || f(runs[2], runs[3], runs[0], runs[1])) as i32
    }).sum()
    
}


fn main() {
    println!("Round1: {}", looper(|a,b,c,d| a >= c && b <= d));
    println!("Round2: {}", looper(|a,b,c,d| (a >= c && a <= d) || (b >= c && b <= d)));
}