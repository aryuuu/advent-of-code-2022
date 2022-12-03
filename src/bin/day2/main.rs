use core::fmt;
use std::{fmt::write, str::FromStr, string::ParseError};

fn main() {
    let matches = std::fs::read_to_string::<_>("./input/day2.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Match>().unwrap())
        .collect::<Vec<Match>>();

    let result = solution(&matches);
    // let result_alt = alt_solution(&foods);
    // let result_part_2 = solution_part_2(&foods);
    // let result_part_2_alt = alt_solution_part_2(&foods);
    println!("result: {}", result);
    // println!("result: {}", result_alt);
    // println!("result_part_2: {}", result_part_2);
    // println!("result_part_2_alt: {}", result_part_2_alt);
}

fn solution(matches: &Vec<Match>) -> usize {
    matches.iter().fold(0, |acc, val| acc + val.get_result())
}

#[derive(Debug, Clone, Copy)]
struct Match {
    theirs: Hand,
    ours: Hand,
}

impl Match {
    fn get_result(self) -> usize {
        self.ours.get_result(self.theirs)
    }
}

impl FromStr for Match {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.split(" ").collect::<Vec<&str>>();
        let result = Match {
            theirs: Hand::from_str(hands[0]).unwrap(),
            ours: Hand::from_str(hands[1]).unwrap(),
        };

        Ok(result)
    }
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock(usize),
    Paper(usize),
    Scissor(usize),
}

impl Hand {
    fn get_result(self, opponent: Hand) -> usize {
        match self {
            Hand::Rock(val) => match opponent {
                Hand::Rock(_) => val + 3,
                Hand::Paper(_) => val,
                Hand::Scissor(_) => val + 6,
            },
            Hand::Paper(val) => match opponent {
                Hand::Rock(_) => val + 6,
                Hand::Paper(_) => val + 3,
                Hand::Scissor(_) => val,
            },
            Hand::Scissor(val) => match opponent {
                Hand::Rock(_) => val,
                Hand::Paper(_) => val + 6,
                Hand::Scissor(_) => val + 3,
            },
        }
    }
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::Rock(1)),
            "B" => Ok(Hand::Paper(2)),
            "C" => Ok(Hand::Scissor(3)),
            "X" => Ok(Hand::Rock(1)),
            "Y" => Ok(Hand::Paper(2)),
            "Z" => Ok(Hand::Scissor(3)),
            _ => Err(HandParseError),
        }
    }
}

#[derive(Debug, Clone)]
struct HandParseError;

impl fmt::Display for HandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid hand")
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matches = vec![
            Match{
                theirs: Hand::Rock(1),
                ours: Hand::Paper(2),
            },
            Match{
                theirs: Hand::Paper(2),
                ours: Hand::Rock(1),
            },
            Match{
                theirs: Hand::Scissor(3),
                ours: Hand::Scissor(3),
            },
        ];

        let result = solution(&matches);
        assert_eq!(result, 15);
    }
}
