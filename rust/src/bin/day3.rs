use std::{collections::VecDeque, env, fs::read_to_string};
use regex::Regex;

fn input() -> String {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = read_to_string(path); 
    contents.expect("Cannot read file")
}


fn part_one_regex() {
    let content = input();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Wrong regex");
    
    let mut mul_sum = 0;
    for result in regex.captures_iter(&content) {
        let x: i64 = result.get(1).unwrap().as_str().parse().unwrap();
        let y: i64 = result.get(2).unwrap().as_str().parse().unwrap();
        mul_sum += x*y;
    }
    println!("Multiplication sum is {mul_sum}");
}

fn part_two_regex() {
    let content = input();
    let regex_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let regex_do =  Regex::new(r"(do|don't)\(\)").unwrap();
    
    let mut values: Vec<(usize, i64)> = vec![]; 
    
    for result in regex_mul.captures_iter(&content) {
        let (_, params) = result.extract();
        let [x,y] =  params.map(|x| x.parse::<i64>().unwrap());
        let start = result.get(0).unwrap().range().start;
        values.push((start, (x*y)));
    }
    
    let mut enabled: VecDeque<(usize, bool)> = vec![(0, true)].into();
    
    for result in regex_do.captures_iter(&content) {
        let (_, [x]) = result.extract();
        let start = result.get(0).unwrap().range().start;
        match x {
            "do" => enabled.push_back((start, true)),
            "don't" => enabled.push_back((start, false)),
            _ => ()
        };
    }
    
    let (_, mut curr_enabled) = enabled.pop_front().unwrap();
    let (mut next_offset, mut next_val) = enabled.pop_front().unwrap();
    
    let mut mul_sum = 0;
    
    for (n, x) in values {
        if n > next_offset {
            curr_enabled = next_val;
            if let Some((a, b)) = enabled.pop_front() { (next_offset, next_val) = (a,b) }
        }
        if curr_enabled { mul_sum +=  x }
    }
    
    println!("Multiplication sum (\\w conditionals) is {mul_sum}");
}


fn main() {
    part_one_regex();
    part_two_regex();
}