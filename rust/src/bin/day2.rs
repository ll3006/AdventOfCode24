use std::{env, fs};

fn input() -> Vec<Vec<i64>> {
    let mut tests: Vec<Vec<i64>> = vec![];
    let args = env::args().collect::<Vec<String>>();
    let content = fs::read_to_string(&args[1]).expect("Cannot read file");
    for line in content.lines() {
        let items = line.split_whitespace()
        .map(|x| x.parse().expect("Cannot parse"))
        .collect();
        
        tests.push(items);
    }
    tests
}

fn part_one() {
    fn test_is_safe(test: &[i64]) -> bool {
        let windows = test.windows(2);
        let diffs = windows.map(|window| window[0] - window[1]);
        if diffs.clone().all(|x| x > 0) || diffs.clone().all(|x| x < 0) {
            let abs_diffs = diffs.map(i64::abs);
            if abs_diffs.clone().all(|x| x >= 1) && abs_diffs.clone().all(|x| x <= 3) {
                // Safe
                return true;
            }
        }
        // Unsafe
        false
    }
    
    let tests = input();
    let mut safe_count = 0;
    
    for test in tests {
        let safe = test_is_safe(&test);
        // println!("{test:?} is {}",  if safe { "Safe" } else { "Unsafe" });
        if safe { safe_count += 1;}
    }
    
    println!("Safe count is {safe_count}");
}

fn part_two() {
    fn test_is_safe(test: &[i64]) -> bool {
        let windows = test.windows(2);
        let diffs = windows.map(|window| window[0] - window[1]);
        if diffs.clone().all(|x| x > 0) || diffs.clone().all(|x| x < 0) {
            let abs_diffs = diffs.map(i64::abs);
            if abs_diffs.clone().all(|x| x >= 1) && abs_diffs.clone().all(|x| x <= 3) {
                // Safe
                return true;
            }
        }
        // Unsafe
        false
    }
    
    let tests = input();
    let mut safe_count = 0;
    
    for test in tests {
        let mut safe = test_is_safe(&test);
        let mut k = 0;
        // If not safe, check if removing a single element makes the test safe
        while !safe && k < test.len()  {
            let mut test = test.clone();
            test.remove(k);
            safe = test_is_safe(&test);
            k += 1;
        }
        
        // println!("{test:?} is {} (dampened)",  if safe { "Safe" } else { "Unsafe" });
        if safe { safe_count += 1;}
    }
    
    println!("Safe count (dampened) is {safe_count}");
}

fn main() {
    part_one();
    part_two();
}