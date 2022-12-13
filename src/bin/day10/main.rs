use std::{str::FromStr, string::ParseError};

fn main() {
    let result = solution("./input/day10.txt");
    println!("result: {result}");
}

fn solution(filename: &str) -> i32 {
    let instructions = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let mut timeline: Vec<i32> = vec![1];

    instructions
        .iter()
        .for_each(|ins| match ins {
            Instruction::Noop => {
                let latest_x = timeline.last().unwrap();
                timeline.push(latest_x.clone());
            }
            Instruction::Addx(val) => {
                let latest_x = timeline.last().unwrap().clone();
                timeline.push(latest_x);
                timeline.push(latest_x + val);
            }
        });

    timeline
        .into_iter()
        .enumerate()
        .fold(0i32, |mut acc: i32, (idx, x)| -> i32 {
            if idx >= 19 && (idx - 19) % 40 == 0 {
                acc += ((idx as i32) + 1) * x;
            }
            acc
        })
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            return Ok(Instruction::Noop);
        }

        let val = s.split_whitespace().collect::<Vec<_>>()[1]
            .parse::<i32>()
            .unwrap();
        Ok(Instruction::Addx(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day10.test.txt");
        assert_eq!(result, 13140);
    }
}
