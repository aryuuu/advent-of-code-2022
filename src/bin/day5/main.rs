use std::{collections::VecDeque, str::FromStr, string::ParseError};

fn main() {
    let result = solution("./input/day5.txt");
    println!("result: {}", result);
}

#[derive(Debug)]
struct Operation {
    origin: usize,
    target: usize,
    amount: usize,
}

impl FromStr for Operation {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        let operation = Operation {
            origin: words[3].parse::<usize>().unwrap() - 1,
            target: words[5].parse::<usize>().unwrap() - 1,
            amount: words[1].parse::<usize>().unwrap(),
        };

        Ok(operation)
    }
}

#[derive(Debug)]
struct Cargo {
    stacks: Vec<VecDeque<char>>,
    operations: Vec<Operation>,
}

impl Cargo {
    fn solve(mut self) -> String {
        self.operations.iter().for_each(|op| {
            for _ in 0..op.amount {
                match self.stacks[op.origin].pop_back() {
                    Some(val) => self.stacks[op.target].push_back(val),
                    None => continue,
                }
            }
        });

        let mut result = vec![];
        self.stacks
            .iter_mut()
            .for_each(|stack| match stack.pop_back() {
                Some(val) => result.push(val),
                None => println!("huh"),
            });
        result.iter().collect()
    }
}

impl FromStr for Cargo {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.split("\n\n").collect::<Vec<_>>();
        let mut stack_input = input[0].lines().collect::<Vec<_>>();

        let stacks_size = (stack_input.pop().unwrap().len() + 1) / 4;
        let mut stacks: Vec<VecDeque<_>> = vec![];
        for _ in 0..stacks_size {
            stacks.push(VecDeque::<char>::new());
        }

        stack_input.iter().rev().for_each(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            for i in (0..stacks_size).rev() {
                match chars.get(i * 4 + 1) {
                    Some(val) => {
                        if val.is_uppercase() {
                            stacks[i].push_back(val.to_owned())
                        }
                    }
                    None => continue,
                }
            }
        });

        let ops_input = input[1];
        let operations = ops_input
            .lines()
            .map(|line| line.parse::<Operation>().unwrap())
            .collect::<Vec<_>>();

        let cargo = Cargo { operations, stacks };

        Ok(cargo)
    }
}

fn solution(filename: &str) -> String {
    let cargo = std::fs::read_to_string(filename)
        .unwrap()
        .parse::<Cargo>()
        .unwrap();
    cargo.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let stacks = vec!["ZN".to_string(), "MCD".to_string(), "P".to_string()];
        let result = solution("./input/day5.test.txt");

        assert_eq!(result, "CMZ".to_string());
    }
}

