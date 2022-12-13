use std::{str::FromStr, string::ParseError};

fn main() {
    let result = solution("./input/day11.txt");
    println!("result: {result}");
}

fn solution(filename: &str) -> usize {
    let mut monkeys = std::fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|paragraph| paragraph.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].inspect_items().into_iter().for_each(|(target, item)| {
                monkeys[target].items.push(item);
            });
            monkeys[i].add_inspected_count();
            monkeys[i].clear_items();
        }
    }

    let mut counts = monkeys.iter().map(|monkey| monkey.items_inspected).collect::<Vec<usize>>();
    counts.sort();
    counts.pop().unwrap() * counts.pop().unwrap()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Vec<String>,
    divisor: i32,
    true_target: usize,
    false_target: usize,
    items_inspected: usize,
}

impl Monkey {
    fn inspect_items(&self) -> Vec<(usize, i32)> {
        self.items.iter().map(|item| {
            let arg1 = match self.operation[0].parse::<i32>() {
                Ok(val) => val,
                Err(_) => *item,
            };
            let arg2 = match self.operation[2].parse::<i32>() {
                Ok(val) => val,
                Err(_) => *item,
            };

            let mut worry_level = if &self.operation[1] == "+" {
                arg1 + arg2
            } else {
                arg1 * arg2
            };
            worry_level /= 3;

            let target = if worry_level % self.divisor == 0 {
                self.true_target
            } else {
                self.false_target
            };

            (target, worry_level)
        }).collect::<Vec<(usize, i32)>>()
    }

    fn add_inspected_count(&mut self) {
        self.items_inspected += self.items.len();
    }

    fn clear_items(&mut self) {
        self.items.clear();
    }
}

impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let items = lines[1].split(": ").collect::<Vec<_>>()[1]
            .split(", ")
            .collect::<Vec<_>>()
            .iter()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let divisor = lines[3].split_whitespace().collect::<Vec<_>>()[3]
            .parse::<i32>()
            .unwrap();
        let true_target = lines[4].split_whitespace().collect::<Vec<_>>()[5]
            .parse::<usize>()
            .unwrap();
        let false_target = lines[5].split_whitespace().collect::<Vec<_>>()[5]
            .parse::<usize>()
            .unwrap();
        let operation = lines[2].split(" = ").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(str::to_string)
            .collect();

        Ok(Monkey {
            items,
            operation,
            divisor,
            true_target,
            false_target,
            items_inspected: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day11.test.txt");
        assert_eq!(result, 10605);
    }
}
