use std::{
    fmt::{self, Display},
    str::FromStr,
    string::ParseError,
};

fn main() {
    let result = solution("./input/day14.txt");
    println!("result: {result}")
}

fn solution(filename: &str) -> usize {
    // the goal is to count the number of grain of sand until the next grain fall below the lowest
    // rock
    // the sand drop from 500, 0
    //
    // parse input
    let input = std::fs::read_to_string(filename).unwrap();
    let rock_paths = input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|node| node.parse::<Coordinate>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // get the max x and y
    let mut max_x = 0;
    let mut max_y = 0;

    rock_paths.iter().for_each(|path| {
        path.iter().for_each(|coor| {
            if coor.x > max_x {
                max_x = coor.x
            }
            if coor.y > max_y {
                max_y = coor.y
            }
        })
    });
    // draw map
    let mut grid = vec![vec![Cell::Air; max_x + 2]; max_y + 2];
    // fill grid with rocks
    rock_paths.iter().for_each(|path| {
        let windows = path.windows(2);
        for window in windows {
            if window[0].x < window[1].x {
                for i in window[0].x..=window[1].x {
                    grid[window[0].y][i] = Cell::Rock;
                }
            } else if window[0].x > window[1].x {
                for i in window[1].x..=window[0].x {
                    grid[window[0].y][i] = Cell::Rock;
                }
            } else if window[0].y < window[1].y {
                for j in window[0].y..=window[1].y {
                    grid[j][window[0].x] = Cell::Rock;
                }
            } else if window[0].y > window[1].y {
                for j in window[1].y..=window[0].y {
                    grid[j][window[0].x] = Cell::Rock;
                }
            }
        }
    });

    // loop for generating new sand
    let mut count = 0;
    'generate: loop {
        let mut new_sand_coor = Coordinate { x: 500, y: 0 };
        'go_down: loop {
            if new_sand_coor.y > max_y {
                break 'generate;
            }

            match grid[new_sand_coor.y + 1][new_sand_coor.x] {
                Cell::Air => {
                    new_sand_coor.y += 1;
                }
                _ => match grid[new_sand_coor.y + 1][new_sand_coor.x - 1] {
                    Cell::Air => {
                        new_sand_coor.y += 1;
                        new_sand_coor.x -= 1;
                    }
                    _ => match grid[new_sand_coor.y + 1][new_sand_coor.x + 1] {
                        Cell::Air => {
                            new_sand_coor.y += 1;
                            new_sand_coor.x += 1;
                        }
                        _ => {
                            grid[new_sand_coor.y][new_sand_coor.x] = Cell::Sand;
                            break 'go_down;
                        }
                    },
                },
            }
        }

        count += 1;

        if new_sand_coor.y > max_y {
            break 'generate;
        }
    }

    count
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<usize> = s
            .trim()
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect();

        Ok(Self {
            x: vals[0],
            y: vals[1],
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Air,
    Sand,
    Rock,
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Air => write!(f, "."),
            Cell::Sand => write!(f, "o"),
            Cell::Rock => write!(f, "#"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day14.test.txt");
        assert_eq!(result, 24);
    }
}
