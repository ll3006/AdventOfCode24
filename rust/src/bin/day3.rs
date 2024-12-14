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

struct Stream<'a> {
    string: &'a str,
    position: usize
}

impl<'a> Stream<'a> {
    pub fn new(string: impl Into<&'a str> ) -> Self {
        Self { string: string.into(), position: 0 }
    }
    
    fn _peek(&self, count: usize) -> (usize, &'a str) {
        let string = self.string;
        let start = self.position;
        let end = 
            string[start..]
            .char_indices()
            .nth(count)
            .map_or(string.len(), |(idx, _)|  start + idx);
    
        (end, &string[start..end])
        
    }
    pub fn peek(&mut self, count: usize) -> &'a str {
        let (_, slice) = self._peek(count);
        slice
    }
    
    pub fn take(&mut self, count: usize) -> &'a str {
        let (end, slice) = self._peek(count);
        self.position = end;
        slice

    }
    
    pub fn skip(&mut self, count: usize) {
        let (end, _) = self._peek(count);
        self.position = end;
    }
    
    pub fn over(&self) -> bool {
        self.position >= self.string.len()
    }
    
    pub fn next(&mut self) -> &str {
        self.take(1)
    }
    
    // pub fn restore(&mut self, pos: usize) {
    //     self.position = pos;
    // }
}

fn part_two() {
    const MUL_OPEN: &str = "mul(";
    const DO: &str = "do()";
    const DONT: &str = "don't()";
    
    let contents = input();
    let mut stream = Stream::new(contents.as_str());

    let mut enabled = true;
    let mut tot_mult = 0;
    while !stream.over() {
        match stream.peek(7) {
            t if t.starts_with(MUL_OPEN) => {
                stream.skip(MUL_OPEN.len());
                let mut digit1 = String::new();
                let mut digit2 = String::new();
                let mut found_comma = false;
                let mut completed = false;
                while !completed {
                    match stream.next() {
                        t if t.chars().next().map_or(false, char::is_numeric) => {
                            let digit = if found_comma { &mut digit2 } else { &mut digit1 };
                        digit.push_str(t);
                        },
                        "," if !digit1.is_empty() && !found_comma => {
                            found_comma = true;
                        }
                        ")" if !digit1.is_empty() && found_comma && !digit2.is_empty() => {
                            completed = true;
                        }
                        _ => { break; }
                    }
                }
                if completed && enabled {
                    println!("{digit1}, {digit2}");
                    let digit1: i64 = digit1.parse().unwrap();
                    let digit2: i64 = digit2.parse().unwrap();
                    tot_mult += digit1 * digit2; 
                }
            },        
            t if t.starts_with(DO) => {
                stream.skip(DO.len());
                enabled = true;
                
            }, 
            t if t.starts_with(DONT) => {
                stream.skip(DONT.len());
                enabled = false;
            },
            _ =>  {
                stream.skip(1);
            } 
        }
    }
    println!("Multiplication sum (\\w conditionals) is {tot_mult}");
}

fn main() {
    part_one_regex();
    part_two_regex();
    part_two();
}