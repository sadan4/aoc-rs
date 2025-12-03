advent_of_code::solution!(3);

pub fn parse_input(s: &str) -> impl Iterator<Item = &[u8]> {
    s.trim().split('\n').map(|line| line.as_bytes())
}

pub fn part_one(input: &str) -> Option<u64> {
    return Some(parse_input(input).fold(0u64, |acc, cur| acc + highest_combo(cur) as u64));
    fn highest_combo(ints: &[u8]) -> u8 {
        let first_idx = max_idx(&ints[..ints.len() - 1]);
        let second_idx = max_idx(&ints[first_idx + 1..]) + first_idx + 1;
        (ints[first_idx] - '0' as u8) * 10 + (ints[second_idx] - '0' as u8)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    return Some(parse_input(input).fold(0u64, |acc, cur| acc + highest_combo(cur) as u64));
    fn highest_combo(ints: &[u8]) -> u64 {
        let mut result = 0;
        let mut last_idx = 0;
        for step in (0..=11).rev() {
            let start = last_idx;
            let end = ints.len() - step;
            // not sure if this is an actual optimization
            if end - start == 1 {
                let mut step = step as i8;
                // would this be faster if it was raw indexing instead of iter?
                let ret = ints[last_idx..].iter().fold(result, |acc, &cur| {
                    let this = ((cur - '0' as u8) as u64) * 10u64.pow(step as u32);
                    step -= 1;
                    this + acc
                });
                debug_assert_eq!(step, -1);
                return ret;
            }
            let possible = &ints[start..end];
            let cur_idx = last_idx + max_idx(possible);
            result += (ints[cur_idx] - '0' as u8) as u64 * 10u64.pow(step as u32);
            last_idx = cur_idx + 1;
        }
        return result;
    }
}

fn max_idx(arr: &[u8]) -> usize {
    let mut max = 0;
    let mut max_idx = 0;
    for i in 0..arr.len() {
        let cur = arr[i];
        if cur == '9' as u8 {
            return i;
        }
        if cur > max {
            max = cur;
            max_idx = i;
        }
    }
    return max_idx;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(17766));
    }

    #[test]
    fn test_part_one_sample() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        // sample result
        assert_eq!(result, Some(176582889354075));
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
