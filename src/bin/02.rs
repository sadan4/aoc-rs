use std::ops::RangeInclusive;

use advent_of_code::util::math::count_digits;

advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<RangeInclusive<i64>> {
    input
        .trim()
        .split(',')
        .map(|it| {
            let arr = it
                .trim()
                .split('-')
                .map(|it| {
                    it.parse::<i64>()
                        .expect(&*format!("failed to parse: {}", it))
                })
                .collect::<Vec<_>>();
            let start = arr[0];
            let end = arr[1];
            start..=end
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    return Some(
        parse_input(input)
            .into_iter()
            .flat_map(|it| it.collect::<Vec<_>>())
            .filter(|it| !is_valid_id(*it))
            .sum::<i64>() as u64,
    );
    fn is_valid_id(id: i64) -> bool {
        let num_digits = count_digits(id);
        if num_digits % 2 == 1 {
            return true;
        }
        let mask = 10i64.pow(num_digits / 2);
        let top_digits = id / mask;
        let bottom_digits = id % mask;
        top_digits != bottom_digits
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    return Some(
        parse_input(input)
            .into_iter()
            .flat_map(|it| it.collect::<Vec<_>>())
            .filter(|it| !is_valid_id(*it))
            .sum::<i64>() as u64,
    );
    fn is_valid_id(id: i64) -> bool {
        if id < 10 {
            return true;
        }
        let i_love_rust_lifetimes = format!("{id}");
        let digits = i_love_rust_lifetimes.chars().collect::<Vec<_>>();
        let num_digits = digits.len();
        let ret = (1..num_digits)
            .filter(|&it| (num_digits as f64 / (it as f64)) % 1.0 == 0.0)
            .all(|it| {
                let chunks = digits.chunks(it).collect::<Vec<_>>();
                let first = chunks[0];
                for i in chunks {
                    if i != first {
                        return true;
                    }
                }
                return false;
            });
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(23701357374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(34284458938));
    }
}
