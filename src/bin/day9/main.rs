use std::collections::HashSet;

fn main() {
    let result = solution("./input/day9.txt");
    println!("result: {result}");
    let result = solution_part_2("./input/day9.txt");
    println!("result: {result}");
}

fn solution(filename: &str) -> usize {
    let binding = std::fs::read_to_string(filename).unwrap();
    let motions = binding.lines().map(|line| {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let step = split[1].parse::<usize>().unwrap();
        (split[0], step)
    });

    let mut visited = HashSet::<String>::new();
    visited.insert("0-0".to_string());

    let mut head = Point::default();
    let mut tail = Point::default();

    for motion in motions {
        for i in 0..(motion.1) {
            head.move_dir(motion.0);
            if tail.is_touching(&head) {
                continue;
            }

            tail.chase(&head);
            visited.insert(format!("{}-{}", tail.x, tail.y));
        }
    }

    visited.len()
}

fn solution_part_2(filename: &str) -> usize {
    let binding = std::fs::read_to_string(filename).unwrap();
    let motions = binding.lines().map(|line| {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let step = split[1].parse::<usize>().unwrap();
        (split[0], step)
    });

    let mut visited = HashSet::<String>::new();
    visited.insert("0-0".to_string());

    let mut knots = vec![Point::default(); 10];

    for motion in motions {
        for i in 0..(motion.1) {
            knots[0].move_dir(motion.0);

            for j in (1..knots.len()) {
                if knots[j].is_touching(&knots[j-1]) {
                    continue;
                }

                let lead = knots[j-1];
                knots[j].chase(&lead);
            }

            visited.insert(format!("{}-{}", knots.last().unwrap().x, knots.last().unwrap().y));
        }
    }

    visited.len()
}

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn chase(&mut self, other: &Point) {
        // this method assumes self and other does not touch and will be touching again in one
        // movement
        if self.x < other.x {
            self.move_right();
        }
        if self.x > other.x {
            self.move_left();
        }
        if self.y < other.y {
            self.move_up();
        }
        if self.y > other.y {
            self.move_down();
        }
    }

    fn is_touching(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn move_dir(&mut self, direction: &str) {
        match direction {
            "U" => {
                self.move_up();
            }
            "D" => {
                self.move_down();
            }
            "L" => {
                self.move_left();
            }
            "R" => {
                self.move_right();
            }
            _ => println!("wha"),
        }
    }

    fn move_down(&mut self) {
        self.y -= 1;
    }

    fn move_up(&mut self) {
        self.y += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day9.test.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day9.test.txt");
        assert_eq!(result, 1);
    }
}
