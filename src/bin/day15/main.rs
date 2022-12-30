use std::{collections::HashSet, str::FromStr, string::ParseError};

fn main() {
    let result = solution("./input/day15.txt", 2_000_000);
    println!("result: {result}");
    let result = solution_part_2("./input/day15.txt", 4_000_000);
    println!("result: {result}");
}

fn solution(filename: &str, row: i32) -> usize {
    // parse input
    let input = std::fs::read_to_string(filename).unwrap();
    let pairs = input
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .collect::<Vec<Pair>>();

    let merged_ranges = get_covered_ranges(&pairs, row);
    let covered = merged_ranges
        .iter()
        .map(|range| range.end - range.start)
        .sum::<i32>() as usize;

    let mut beacons = HashSet::new();
    pairs
        .iter()
        .map(|pair| pair.beacon)
        .filter(|beacon| beacon.y == row)
        .for_each(|beacon| {
            beacons.insert(beacon.x);
        });

    covered - beacons.len()
}

fn get_covered_ranges(pairs: &[Pair], row: i32) -> Vec<std::ops::Range<i32>> {
    let mut ranges = pairs
        .iter()
        .map(|pair| pair.get_col_coverage_in_row(row))
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|range| range.start);

    let mut merged_ranges = vec![ranges[0].clone()];

    for next in &ranges[1..] {
        let last_idx = merged_ranges.len() - 1;
        let last = &merged_ranges[last_idx];

        if next.start < last.end || last.end == next.start {
            if next.end > last.end {
                let old = &merged_ranges[last_idx];
                let new = old.start..next.end;
                merged_ranges[last_idx] = new;
            }
        } else {
            merged_ranges.push(next.clone());
        }
    }

    merged_ranges
}

fn solution_part_2(filename: &str, size: i32) -> i64 {
    // parse input
    let input = std::fs::read_to_string(filename).unwrap();
    let pairs = input
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .collect::<Vec<Pair>>();

    let (row, col_ranges) = (0..=size)
        .rev()
        .map(|row| (row, get_covered_ranges(&pairs, row)))
        .find(|(_, ranges)| ranges.len() > 1)
        .unwrap();

    // let mut beacons = HashSet::new();
    // pairs
    //     .iter()
    //     .map(|pair| pair.beacon)
    //     .filter(|beacon| beacon.y == size)
    //     .for_each(|beacon| {
    //         beacons.insert(beacon.x);
    //     });

    // covered - beacons.len()
    let col = col_ranges.first().unwrap().end ;

    i64::from(col) * 4_000_000 + i64::from(row)
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day15.test.txt", 20);
        assert_eq!(result, 56000011);
    }
}
