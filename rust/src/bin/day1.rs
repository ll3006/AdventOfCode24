/* 
Pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest left number with the second-smallest right number, and so on.
Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances. For example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4; if you pair up a 9 with a 3, the distance apart is 6.
*/

use std::{env, fs, iter::zip};

fn input() -> (Vec<isize>, Vec<isize>) {
    let mut list1: Vec<isize> = vec![];
    let mut list2: Vec<isize> = vec![];
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    for line in contents.lines() {
        let data: Vec<&str> = line.split_whitespace().collect();   
        list1.push(data[0].parse().expect("Not a number"));
        list2.push(data[1].parse().expect("Not a number"));
    }
    
    (list1, list2)
}

fn remove<T: PartialEq>(list: &mut Vec<T>, val: &T) {
    list.iter().position(|x| x == val).map(|x| list.swap_remove(x));
}

fn part_one() {
    let (mut list1, mut list2) = input();

    let mut total_diff = 0; 
    while !list1.is_empty() && !list2.is_empty() {
        let min1 = *list1.iter().min().expect("Iterator is empty");
        let min2 = *list2.iter().min().expect("Iterator is empty");
        total_diff += min1.abs_diff(min2);
        remove(&mut list1, &min1);
        remove(&mut list2, &min2);
    }
    println!("Total diff is {total_diff}");
}

fn part_one_v2() {
    let (mut list1, mut list2) = input();
    list1.sort_unstable();
    list2.sort_unstable();
    
    let total_diff = zip(list1, list2).fold(0, |acc, (a, b)| acc + a.abs_diff(b));
    println!("Total diff is {total_diff}");
}


fn part_two() {
    let (list1, list2) = input();
    
    let mut similarity_score = 0;
    for x in list1 {
        let count: isize = list2.iter().filter(|&y| *y == x).count().try_into().unwrap();
        similarity_score += x*count;
    }
    
    println!("Similarity score is {similarity_score}");
}
fn main() {
    part_one();
    part_one_v2();
    part_two();
}