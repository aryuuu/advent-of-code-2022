fn main() {
    let result = solution("./input/day13.txt");
    println!("result: {result}");
    let result = solution_part_2("./input/day13.txt");
    println!("result: {result}");
}

fn solution(filename: &str) -> usize {
    let pairs = parse(filename);

    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn solution_part_2(filename: &str) -> usize {
    // parse input
    let packets = parse_part_2(filename);
    // count the number of packet that is less than the first divider
    let first_divider = parse_packet("[[2]]");
    let second_divider = parse_packet("[[6]]");
    let first_divider_count = packets.iter().fold(1, |mut acc, packet| {
        if packet < &first_divider {
            acc += 1;
        }

        acc
    });
    // count the number of packet that is less than the second divider
    let second_divider_count = packets.iter().fold(2, |mut acc, packet| {
        // we start at 2 because
        // the second divider must be larger than the first
        if packet < &second_divider {
            acc += 1;
        }

        acc
    });
    // multiply the two
    first_divider_count * second_divider_count
    // profit
}

fn parse(filename: &str) -> Vec<[Packet; 2]> {
    let input = std::fs::read_to_string(filename).unwrap();

    input
        .split("\n\n")
        .map(|pair| {
            let lines = pair.lines().collect::<Vec<_>>();
            let left = lines[0];
            let right = lines[1];

            [parse_packet(left), parse_packet(right)]
        })
        .collect()
}

fn parse_part_2(filename: &str) -> Vec<Packet> {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .replace("\n\n", "\n");

    input.lines().map(parse_packet).collect()
}

fn parse_packet(s: &str) -> Packet {
    let chars: Vec<_> = s.chars().collect();
    let (packet, _) = parse_list(&chars);
    packet
}

fn parse_list(list: &[char]) -> (Packet, &[char]) {
    let mut list = &list[1..];
    let mut packets = vec![];

    loop {
        match list[0] {
            ']' => break,
            ',' => list = &list[1..],
            '[' => {
                let (packet, rest) = parse_list(list);
                packets.push(packet);
                list = rest;
            }
            _ => {
                let (n, rest) = parse_num(list);
                packets.push(n);
                list = rest;
            }
        }
    }

    (Packet::List(packets), &list[1..])
}

fn parse_num(list: &[char]) -> (Packet, &[char]) {
    let mut i = 0;
    while i < list.len() && list[i].is_ascii_digit() {
        i += 1;
    }

    let mut num = 0;
    let mut offset = 1;
    for c in list[..i].iter().rev() {
        num += (*c as u8 - b'0') * offset;
        offset *= 10;
    }

    (Packet::Num(num), &list[i..])
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution("./input/day13.test.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn it_works_part_2() {
        let result = solution_part_2("./input/day13.test.txt");
        assert_eq!(result, 140);
    }
}
