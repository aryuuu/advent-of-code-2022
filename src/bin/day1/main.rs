fn main() {
    let foods = std::fs::read_to_string::<_>("./input/day1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>();
    let result = solution(&foods);
    let result_part_2 = solution_part_2(&foods);
    println!("result: {}", result);
    println!("result_part_2: {}", result_part_2);
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
    // println!("sums: {:?}", &sums);
    
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
    fn it_works_part_2() {
        let foods = vec![
            1000, 2000, 3000, 0, 4000, 0, 5000, 6000, 0, 7000, 8000, 9000, 0, 10000,
        ];
        let result = solution_part_2(&foods);
        assert_eq!(result, 45000);
    }
}
