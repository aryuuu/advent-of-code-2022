use std::{str::FromStr, string::ParseError};

fn main() {
    let result = solution("./input/day11.txt");
    println!("result: {result}");
    let result = solution_part_2("./input/day11.txt");
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
            monkeys[i]
                .inspect_items()
                .into_iter()
                .for_each(|(target, item)| {
                    monkeys[target].items.push(item);
                });
            monkeys[i].add_inspected_count();
            monkeys[i].clear_items();
        }
    }

    let mut counts = monkeys
        .iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<usize>>();
    counts.sort();
    counts.pop().unwrap() * counts.pop().unwrap()
}

fn solution_part_2(filename: &str) -> usize {
    let mut monkeys = std::fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|paragraph| paragraph.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();

    let common_multiple: usize = monkeys.iter().map(|monkey| monkey.divisor).product();

    monkeys
        .iter_mut()
        .for_each(|monkey| monkey.common_multiple = common_multiple);

    for a in 0..10_000 {
        for i in 0..monkeys.len() {
            monkeys[i]
                .inspect_items_part_2()
                .into_iter()
                .for_each(|(target, item)| {
                    monkeys[target].items.push(item);
                });
            monkeys[i].add_inspected_count();
            monkeys[i].clear_items();
        }
    }

    let mut counts = monkeys
        .iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<usize>>();
    counts.sort();
    counts.pop().unwrap() * counts.pop().unwrap()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Vec<String>,
    divisor: usize,
    common_multiple: usize,
    true_target: usize,
    false_target: usize,
    items_inspected: usize,
}

impl Monkey {
    fn inspect_items(&self) -> Vec<(usize, usize)> {
        self.items
            .iter()
            .map(|item| {
                let arg1 = match self.operation[0].parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => *item,
                };
                let arg2 = match self.operation[2].parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => *item,
                };

                let mut worry_level = if &self.operation[1] == "+" {
                    arg1 + arg2
                } else {
                    arg1 * arg2
                };
                worry_level /= self.common_multiple;

                let target = if worry_level % self.divisor == 0 {
                    self.true_target
                } else {
                    self.false_target
                };

                (target, worry_level)
            })
            .collect::<Vec<(usize, usize)>>()
    }

    fn inspect_items_part_2(&self) -> Vec<(usize, usize)> {
        self.items
            .iter()
            .map(|item| {
                let arg1 = match self.operation[0].parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => *item,
                };
                let arg2 = match self.operation[2].parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => *item,
                };

                let mut worry_level = if &self.operation[1] == "+" {
                    arg1 + arg2
                } else {
                    arg1 * arg2
                };

                worry_level %= self.common_multiple;

                let target = if worry_level % self.divisor == 0 {
                    self.true_target
                } else {
                    self.false_target
                };

                (target, worry_level)
            })
            .collect::<Vec<(usize, usize)>>()
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
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let divisor = lines[3].split_whitespace().collect::<Vec<_>>()[3]
            .parse::<usize>()
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
            common_multiple: 3,
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

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day11.test.txt");
        assert_eq!(result, 2713310158);
    }
}
