use std::collections::{BinaryHeap, HashSet};

fn main() {
    let result = solution("./input/day12.txt");
    println!("result: {result}");
    let result = solution_part_2("./input/day12.txt");
    println!("result: {result}");
}

fn solution(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut hill = vec![vec![0; cols]; rows];
    let mut start = Coordinate { row: 0, col: 0 };
    let mut end = Coordinate { row: 0, col: 0 };

    for (row, line) in input.lines().enumerate() {
        for (col, chara) in line.chars().enumerate() {
            let letter = match chara {
                'S' => {
                    start.row = row;
                    start.col = col;
                    'a'
                }
                'E' => {
                    end.row = row;
                    end.col = col;
                    'z'
                }
                _ => chara,
            };

            let height = letter as u8 - b'a';
            hill[row][col] = height;
        }
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Node {
        coordinate: start,
        cost: 0,
    });
    visited.insert(start);

    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        if curr.coordinate == end {
            return curr.cost;
        }

        let curr_height = hill[curr.coordinate.row][curr.coordinate.col];
        let neighbors = curr.coordinate.get_neighbors(rows, cols);
        let walkable_neighbors = neighbors
            .iter()
            .filter(|neighbor| {
                let height = hill[neighbor.row][neighbor.col];
                height <= curr_height || curr_height + 1 == height
            })
            .collect::<Vec<_>>();

        for neighbor in walkable_neighbors {
            if visited.insert(*neighbor) {
                queue.push(Node {
                    coordinate: *neighbor,
                    cost: curr.cost + 1,
                })
            }
        }
    }

    usize::MAX
}

fn solution_part_2(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut hill = vec![vec![0; cols]; rows];
    let mut start = Coordinate { row: 0, col: 0 };
    let mut end = Coordinate { row: 0, col: 0 };

    for (row, line) in input.lines().enumerate() {
        for (col, chara) in line.chars().enumerate() {
            let letter = match chara {
                'S' => {
                    end.row = row;
                    end.col = col;
                    'a'
                }
                'E' => {
                    start.row = row;
                    start.col = col;
                    'z'
                }
                _ => chara,
            };

            let height = letter as u8 - b'a';
            hill[row][col] = height;
        }
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Node {
        coordinate: start,
        cost: 0,
    });
    visited.insert(start);

    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        let curr_height = hill[curr.coordinate.row][curr.coordinate.col];
        if curr_height == 0 {
            return curr.cost;
        }

        let neighbors = curr.coordinate.get_neighbors(rows, cols);
        let walkable_neighbors = neighbors
            .iter()
            .filter(|neighbor| {
                let height = hill[neighbor.row][neighbor.col];
                height >= curr_height || curr_height - 1 == height
            })
            .collect::<Vec<_>>();

        for neighbor in walkable_neighbors {
            if visited.insert(*neighbor) {
                queue.push(Node {
                    coordinate: *neighbor,
                    cost: curr.cost + 1,
                })
            }
        }
    }

    usize::MAX
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn get_neighbors(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut result = vec![];

        // up
        if self.row > 0 {
            result.push(Self {
                row: self.row - 1,
                col: self.col,
            })
        }

        // down
        if self.row < rows - 1 {
            result.push(Self {
                row: self.row + 1,
                col: self.col,
            })
        }

        // left
        if self.col > 0 {
            result.push(Self {
                row: self.row,
                col: self.col - 1,
            })
        }

        // right
        if self.col < cols - 1 {
            result.push(Self {
                row: self.row,
                col: self.col + 1,
            })
        }

        result
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Node {
    coordinate: Coordinate,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day12.test.txt");
        assert_eq!(result, 31);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day12.test.txt");
        assert_eq!(result, 29);
    }
}
