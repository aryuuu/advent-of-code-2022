fn main() {
    let result = solution("./input/day7.txt");
    println!("result: {result}");
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

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day7.test.txt");
        assert_eq!(result, 95437);
    }
}

