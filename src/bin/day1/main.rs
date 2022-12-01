fn main() {
    let foods = std::fs::read_to_string::<_>("./input/day1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>();
    let result = solution(&foods);
    let result_alt = alt_solution(&foods);
    let result_part_2 = solution_part_2(&foods);
    let result_part_2_alt = alt_solution_part_2(&foods);
    println!("result: {}", result);
    println!("result: {}", result_alt);
    println!("result_part_2: {}", result_part_2);
    println!("result_part_2_alt: {}", result_part_2_alt);
}

fn solution(foods: &Vec<u32>) -> u32 {
    let mut max = 0;
    let mut sum = 0;

    for food in foods {
        if 0 == *food {
            sum = 0;
            continue;
        }
        sum += food;

        if sum >= max {
            max = sum;
        }
    }

    max
}

fn alt_solution(foods: &Vec<u32>) -> u32 {
    let dummy = foods
        .iter()
        .fold((vec![], 0), |mut acc: (Vec<u32>, u32), food: &u32| {
            if *food == 0 {
                acc.0.push(acc.1);
                acc.1 = 0;
            } else {
                acc.1 += food;
            }
            acc
        })
        .0
        .iter()
        .max()
        .unwrap()
        .to_owned();

    dummy
}

fn alt_solution_alt(foods: &Vec<u32>) -> u32 {
    let dummy = foods
        .iter()
        .fold(vec![0], |mut acc: Vec<u32>, food: &u32| {
            // let mut sum = 0;
            if *food == 0 {
                acc.push(0);
                // acc.0.push(acc.1);
                // acc.1 = 0;
            } else {
                let length = acc.len();
                acc[length-1] += food;
                // acc.1 += food;
            }
            acc
        })
        .iter()
        .max()
        .unwrap()
        .to_owned();

    dummy
}

fn solution_part_2(foods: &Vec<u32>) -> u32 {
    let mut max = 0;
    let mut sum = 0;
    let mut sums = vec![];

    for food in foods {
        if *food == 0 {
            sums.push(sum);
            sum = 0;
            continue;
        }
        sum += food;

        if sum >= max {
            max = sum;
        }
    }
    sums.push(sum);

    sums.sort();
    sums.reverse();

    sums[0] + sums[1] + sums[2]
}

fn alt_solution_part_2(foods: &Vec<u32>) -> u32 {
    let mut sums = foods
        .iter()
        .fold(vec![0], |mut acc: Vec<u32>, food: &u32| {
            if *food == 0 {
                acc.push(0);
            } else {
                let length = acc.len();
                acc[length-1] += food;
            }
            acc
        });

    sums.sort_by(|a, b| b.cmp(a));

    sums[0] + sums[1] + sums[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foods = vec![
            1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000,
        ];
        let result = solution(&foods);
        assert_eq!(result, 24000);
    }

    #[test]
    fn it_works_alt() {
        let foods = vec![
            1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000,
        ];
        let result = alt_solution(&foods);
        assert_eq!(result, 24000);
    }

    #[test]
    fn it_works_part_2() {
        let foods = vec![
            1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000,
        ];
        let result = solution_part_2(&foods);
        assert_eq!(result, 45000);
    }

    #[test]
    fn it_works_alt_part_2() {
        let foods = vec![
            1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000,
        ];
        let result = alt_solution_part_2(&foods);
        assert_eq!(result, 45000);
    }
}
