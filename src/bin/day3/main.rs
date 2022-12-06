#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};

const LOWER_BOUND: u8 = b'a' - 1;
const UPPER_BOUND: u8 = b'A' - 1;

fn main() {
    println!("part 1: {}", solution("./input/day3.txt"));
    println!("part 2: {}", solution_part_2("./input/day3.txt"));
}

fn solution(filename: &str) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .flat_map(|line| {
            let mid = line.len() / 2;
            let (first, second) = line.split_at(mid);
            let first = first.chars().collect::<HashSet<char>>();
            second
                .chars()
                .filter(move |c| first.contains(c))
                .collect::<HashSet<char>>()
                .into_iter()
                .map(|c| {
                    if c.is_ascii_lowercase() {
                        c as u8 - LOWER_BOUND
                    } else {
                        c as u8 - UPPER_BOUND + 26
                    }
                })
                .map(|val| val as usize)
        })
        .sum::<usize>()
}

fn solution_part_2(filename: &str) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .array_chunks::<3>()
        .flat_map(|group| {
            group
                .iter()
                .flat_map(|line| line.chars().collect::<HashSet<char>>().into_iter())
                .fold(HashMap::new(), |mut map: HashMap<char, u32>, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .into_iter()
                .filter(|(_, v)| *v == 3)
        })
        .map(|c| c.0)
        .map(|c| {
            if c.is_ascii_lowercase() {
                c as u8 - LOWER_BOUND
            } else {
                c as u8 - UPPER_BOUND + 26
            }
         })
        .map(|c| c as usize)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day3.test.txt");
        assert_eq!(result, 157);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day3.test.txt");
        assert_eq!(result, 70);
    }
}
