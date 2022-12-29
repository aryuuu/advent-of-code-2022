use std::{collections::HashSet, str::FromStr, string::ParseError};

fn main() {
    let result = solution("./input/day15.txt", 2_000_000);
    println!("result: {result}");
}

fn solution(filename: &str, row: i32) -> usize {
    // parse input
    let input = std::fs::read_to_string(filename).unwrap();
    let pairs = input
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .collect::<Vec<Pair>>();

    let mut columns = HashSet::<i32>::new();

    pairs.iter().for_each(|pair| {
        let covered_columns = pair.get_col_coverage_in_row(row);
        for col in covered_columns {
            columns.insert(col);
        }
    });

    pairs.iter().for_each(|pair| {
        // remove col of beacons in row from set
        if pair.beacon.y == row {
            columns.remove(&pair.beacon.x);
        }
    });

    columns.len()
}

#[derive(Debug)]
struct Pair {
    sensor: Coordinate,
    beacon: Coordinate,
}

impl Pair {
    fn get_col_coverage_in_row(&self, row: i32) -> std::ops::Range<i32> {
        if !self.can_reach_row(row) {
            return 0..0;
        }

        let coverage = self.get_coverage();

        let start = self.sensor.x - (coverage as i32 - (self.sensor.y - row).abs());
        let end = self.sensor.x + (coverage as i32 - (self.sensor.y - row).abs());

        start..end + 1
    }

    fn can_reach_row(&self, row: i32) -> bool {
        self.get_coverage() >= (self.sensor.y - row).abs() as usize
    }

    fn get_coverage(&self) -> usize {
        self.sensor.manhattan_distance(&self.beacon)
    }
}

impl FromStr for Pair {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_input, beacon_input) = s.split_once(": ").unwrap();

        let (sensor_x_raw, sensor_y_raw) = sensor_input.split_once(",").unwrap();
        let (beacon_x_raw, beacon_y_raw) = beacon_input.split_once(",").unwrap();

        let sensor_x = sensor_x_raw
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let sensor_y = sensor_y_raw
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let beacon_x = beacon_x_raw
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let beacon_y = beacon_y_raw
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();

        let sensor = Coordinate {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon = Coordinate {
            x: beacon_x,
            y: beacon_y,
        };

        Ok(Self { sensor, beacon })
    }
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day15.test.txt", 10);
        assert_eq!(result, 26);
    }
}
