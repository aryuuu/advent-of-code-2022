use std::{str::FromStr, string::ParseError};

fn main() {
    let result = solution("./input/day4.txt");
    let result_part_2 = solution_part_2("./input/day4.txt");
    println!("result: {}", result);
    println!("result_part_2: {}", result_part_2)
}

#[derive(Debug)]
struct Assignments {
    first_lower: u32,
    first_upper: u32,
    second_lower: u32,
    second_upper: u32,
}

impl FromStr for Assignments {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(",").collect::<Vec<_>>();
        let first_split = splitted[0].split("-").collect::<Vec<_>>();
        let second_split = splitted[1].split("-").collect::<Vec<_>>();

        Ok(Assignments {
            first_lower: first_split[0].parse::<u32>().unwrap(),
            first_upper: first_split[1].parse::<u32>().unwrap(),
            second_lower: second_split[0].parse::<u32>().unwrap(),
            second_upper: second_split[1].parse::<u32>().unwrap(),
        })
    }
}

fn solution(filename: &str) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse::<Assignments>().unwrap())
        .fold(0, |mut acc, assignment| {
            acc += if assignment.first_lower >= assignment.second_lower
                && assignment.first_upper <= assignment.second_upper
                || assignment.second_lower >= assignment.first_lower
                    && assignment.second_upper <= assignment.first_upper
            {
                1
            } else {
                0
            };
            // acc += count;
            acc
        })
}

fn solution_part_2(filename: &str) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse::<Assignments>().unwrap())
        .fold(0, |mut acc, assignment| {
            acc += if assignment.first_lower <= assignment.second_lower
                && assignment.first_upper >= assignment.second_lower
                || assignment.second_lower <= assignment.first_lower
                    && assignment.second_upper >= assignment.first_lower
            {
                1
            } else {
                0
            };
            // acc += count;
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day4.test.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day4.test.txt");
        assert_eq!(result, 4);
    }
}
