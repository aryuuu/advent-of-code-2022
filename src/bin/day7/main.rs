fn main() {
    let result = solution("./input/day7.txt");
    println!("result: {result}");
    let result_part_2 = solution_part_2("./input/day7.txt");
    println!("result: {result_part_2}");
}

fn solution(filename: &str) -> usize {
    let binding = std::fs::read_to_string(filename).unwrap();
    let output = binding.lines();

    // create a stack
    // when entering a directory (not coming back to it), push dir to stack, set size to 0
    // if a file is met, add filesize to the top of the stack
    // check all file in dir, add them to total
    // when going back to parent dir, check the size of the top of the stack, if < limit, add
    // to total, and to parent dir size

    let mut stack = vec![0];
    let mut total = 0;
    for line in output {
        // ignore root, ls 
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let cmd = line.split_whitespace().collect::<Vec<_>>();
            if cmd[2] == ".." {
                let curr_dir_size = stack.pop().unwrap();
                if curr_dir_size <= 100_000 {
                    total += curr_dir_size;
                }

                *stack.last_mut().unwrap() += curr_dir_size;
            } else {
                stack.push(0);
            }
        } else {
            let file_desc = line.split_whitespace().collect::<Vec<_>>();
            let size = file_desc[0].parse::<usize>();
            match size {
                Ok(val) => *stack.last_mut().unwrap() += val,
                _ => println!("ignore"),
            }
        }
    }

    while !stack.is_empty() {
        let curr_dir_size = stack.pop().unwrap();

        if curr_dir_size <= 100_000 {
            total += curr_dir_size;
        }

        let last = stack.last_mut();
        match last {
            Some(val) => *val += curr_dir_size,
            None => break,
        }
    }

    total
}

fn solution_part_2(filename: &str) -> usize {
    // similar to part 1, but we don't go with the total
    // instead we will take max size that doesn't exceed the limit

    let binding = std::fs::read_to_string(filename).unwrap();
    let output = binding.lines();

    let mut stack = vec![0];
    let mut all_dirs = vec![];
    for line in output {
        // ignore root, ls 
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let cmd = line.split_whitespace().collect::<Vec<_>>();
            if cmd[2] == ".." {
                let curr_dir_size = stack.pop().unwrap();
                all_dirs.push(curr_dir_size);

                *stack.last_mut().unwrap() += curr_dir_size;
            } else {
                stack.push(0);
            }
        } else {
            let file_desc = line.split_whitespace().collect::<Vec<_>>();
            let size = file_desc[0].parse::<usize>();
            match size {
                Ok(val) => *stack.last_mut().unwrap() += val,
                _ => println!("ignore"),
            }
        }
    }

    while !stack.is_empty() {
        let curr_dir_size = stack.pop().unwrap();

        all_dirs.push(curr_dir_size);

        let last = stack.last_mut();
        match last {
            Some(val) => *val += curr_dir_size,
            None => break,
        }
    }

    let total_space = 70000000;
    let space_required = 30000000;
    let free_space = total_space - all_dirs.last().unwrap();
    let space_to_delete = space_required - free_space;

    all_dirs.into_iter().filter(|dir| *dir >= space_to_delete).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day7.test.txt");
        assert_eq!(result, 95437);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day7.test.txt");
        assert_eq!(result, 24933642);
    }
}

