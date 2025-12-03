use advent_of_code::util::math::SignExt;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .map(|line| {
            let clicks = *&line[1..].parse::<i32>().expect("invalid int");
            if line.starts_with("R") {
                clicks
            } else {
                -clicks
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut cur = 50;
    let mut zero_count = 0;
    let parsed = parse_input(input);
    for i in parsed {
        cur = (cur + i) % 100;
        if cur == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cur = 50;
    let mut zero_count = 0;
    let parsed = parse_input(input);
    for i in parsed {
        let old = cur;
        cur += i % 100;
        let new = cur;

        zero_count += i.abs() / 100;

        if old % 100 == 0 {
            continue;
        } else if new % 100 == 0 {
            zero_count += 1;
        } else if new / 100 != old / 100 {
            zero_count += 1;
        } else if new / 100 == 0 && new.sign() != old.sign() {
            zero_count += 1;
        }
    }
    Some(zero_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1118));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6289));
    }
}
