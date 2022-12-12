fn main() {
    let result = solution("./input/day8.txt");
    println!("result: {result}");
    let result = solution_part_2("./input/day8.txt");
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

fn solution_part_2(filename: &str) -> usize {
    let forest = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let forest_length = forest.len();
    let forest_width = forest[0].len();
    let mut max_score = 0;

    for i in 0..forest_length {
        // if i == 0 || i == forest_length - 1 {
        //     visible_count += forest_width;
        //     continue;
        // }

        for j in 0..forest_width {
            // corner case
            // if j == 0 || j == forest_width - 1 {
            //     visible_count += 1;
            //     continue;
            // }

            let curr_tree = forest[i][j];
            let mut top_view = 0;
            let mut bottom_view = 0;
            let mut left_view = 0;
            let mut right_view = 0;

            // left check
            for k in (0..j).rev() {
                left_view += 1;
                if curr_tree <= forest[i][k] {
                    break;
                }
            }
            // right check
            for k in j+1..forest_width {
                right_view += 1;
                if curr_tree <= forest[i][k] {
                    break;
                }
            }
            // up check
            for k in (0..i).rev() {
                top_view += 1;
                if curr_tree <= forest[k][j] {
                    break;
                }
            }
            // bottom check
            for k in i+1..forest_length {
                bottom_view += 1;
                if curr_tree <= forest[k][j] {
                    break;
                }
            }

            let curr_score = top_view * bottom_view * left_view * right_view;
            if curr_score > max_score {
                // println!("curr_tree: {curr_tree}");
                // println!("i, j: {i}, {j}");
                max_score = curr_score;
            }
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day8.test.txt");
        assert_eq!(result, 21);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day8.test.txt");
        assert_eq!(result, 8);
    }
}
