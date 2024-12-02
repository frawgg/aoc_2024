use std::{collections::HashMap, env::args, fs};

fn main() {
    let args: Vec<String> = args().collect();

    let input = fs::read_to_string(&args[1])
        .unwrap()
        .trim()
        .to_owned();

    let (left, right) = parse(input);

    println!("{}", part_one(&left, &right));
    println!("{}", part_two(&left, &right));
}

fn part_one(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let total: u32 = left.iter().zip(right.iter())
        .map(|(lv, rv)| lv.abs_diff(*rv))
        .sum();

    total
}

fn part_two(left: &Vec<u32>, right: &Vec<u32>) -> u64 {
    let mut map: HashMap<u32, u32> = HashMap::new();

    right.iter().for_each(|v| *map.entry(*v).or_default() += 1);

    left.iter().map(|v| *v as u64 * *map.get(v).unwrap_or(&0) as u64)
        .sum::<u64>()
}

fn parse(input: String) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right) = (Vec::<u32>::new(), Vec::<u32>::new());

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<u32>().unwrap());
        right.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    (left, right)
}
