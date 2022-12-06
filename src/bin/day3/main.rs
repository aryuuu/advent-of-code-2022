use std::collections::{HashMap, HashSet};

const LOWER_BOUND: u8 = b'a' - 1;
const UPPER_BOUND: u8 = b'A' - 1;

fn main() {
    println!("part 1: {}", solution("./input/day3.txt"));
    // let rucksacks = std::fs::read_to_string::<_>("./input/day3.txt")
    //     .unwrap()
    //     .lines()
    //     .map(|line| line.parse::<u32>().unwrap_or(0))
    //     .collect::<Vec<u32>>();
    // let result = solution(&foods);
    // let result_alt = alt_solution(&foods);
    // let result_part_2 = solution_part_2(&foods);
    // let result_part_2_alt = alt_solution_part_2(&foods);
    // println!("result: {}", result);
    // println!("result: {}", result_alt);
    // println!("result_part_2: {}", result_part_2);
    // println!("result_part_2_alt: {}", result_part_2_alt);
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
                    let result = if c.is_ascii_lowercase() {
                        c as u8 - LOWER_BOUND
                    } else {
                        c as u8 - UPPER_BOUND + 26
                    };
                    return result;
                })
                .map(|val| val as usize)
        })
        .sum::<usize>()
}

// #[derive(Debug)]
// struct Rucksack {
//     first_comparment: HashSet<char>,
//     second_comparment: HashSet<char>,
// }

// impl Rucksack {
//     // add code here
//     // fn get_common_item(self) -> Vec<&char> {
//     //     self.first_comparment
//     //         .intersection(&self.second_comparment)
//     //         .into_iter()
//     //         .collect::<Vec<&char>>()
//     // }
// }

// fn solution(rucksacks: Vec<HashSet<char>>) -> usize {
//     unimplemented!()
//     // rucksacks.iter().fold(, f)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let foods = vec![
        //     1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000,
        // ];
        let result = solution("./input/day3.test.txt");
        assert_eq!(result, 157);
    }
}
