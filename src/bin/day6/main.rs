use std::collections::HashSet;

fn main() {
    let result = solution("./input/day6.txt");
    println!("result: {}", result);
    let result_part_2 = solution_part_2("./input/day6.txt");
    println!("result_part_2: {}", result_part_2);
}

fn solution(filename: &str) -> usize {
    let signal = std::fs::read_to_string(filename)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    for i in 0..signal.len() - 4 {
        // lets go with comparison to save space or something idk
        if signal[i] != signal[i + 1]
            && signal[i] != signal[i + 2]
            && signal[i] != signal[i + 3]
            && signal[i + 1] != signal[i + 2]
            && signal[i + 1] != signal[i + 3]
            && signal[i + 2] != signal[i + 3]
        {
            return i + 4;
        }
    }
    1
}

fn solution_part_2(filename: &str) -> usize {
    let signal = std::fs::read_to_string(filename)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    for i in 0..signal.len() - 4 {
        let mut set = HashSet::<char>::new();

        for j in i..i+14 {
            set.insert(signal[j]);
        }

        if set.len() == 14 {
            return i+14
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day6.test.txt");
        assert_eq!(result, 7);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day6.test.txt");
        assert_eq!(result, 19);
    }
}
