fn main() {
    let result = solution("./input/day8.txt");
    println!("result: {result}");
}

fn solution(filename: &str) -> usize {
    let forest = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let forest_length = forest.len();
    let forest_width = forest[0].len();

    let mut visible_count = 0;
    for i in 0..forest_length {
        if i == 0 || i == forest_length - 1 {
            visible_count += forest_width;
            continue;
        }

        for j in 0..forest_width {
            // corner case
            if j == 0 || j == forest_width - 1 {
                visible_count += 1;
                continue;
            }

            let curr_tree = forest[i][j];
            let mut cover_count = 0;

            // left check
            for k in 0..j {
                if curr_tree <= forest[i][k] {
                    cover_count += 1;
                    break;
                }
            }
            // right check
            for k in j+1..forest_width {
                if curr_tree <= forest[i][k] {
                    cover_count += 1;
                    break;
                }
            }
            // up check
            for k in 0..i {
                if curr_tree <= forest[k][j] {
                    cover_count += 1;
                    break;
                }
            }
            // bottom check
            for k in i+1..forest_length {
                if curr_tree <= forest[k][j] {
                    cover_count += 1;
                    break;
                }
            }

            if cover_count < 4 {
                visible_count += 1;
            }
        }
    }

    visible_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day8.test.txt");
        assert_eq!(result, 21);
    }
}
