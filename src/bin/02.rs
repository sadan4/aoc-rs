use advent_of_code::util::math::{
    count_digits_u64, int_to_digits_u64_2,
};

advent_of_code::solution!(2);

fn parse_input(input: &str) -> impl Iterator<Item = u64> + use<'_> {
    input.trim().split(',').flat_map(|it| {
        let mut arr = it
            .trim()
            .split('-')
            .map(|it| it.parse::<u64>().expect("failed to parse number"));
        let start = arr.next().unwrap();
        let end = arr.next().unwrap();
        debug_assert!(arr.next().is_none());
        start..=end
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    return Some(parse_input(input).filter(|&it| !is_valid_id(it)).sum());
    fn is_valid_id(id: u64) -> bool {
        let num_digits = count_digits_u64(id);
        if num_digits % 2 == 1 {
            return true;
        }
        let mask = 10u64.pow(num_digits / 2);
        let top_digits = id / mask;
        let bottom_digits = id % mask;
        top_digits != bottom_digits
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    return Some(parse_input(input).filter(|&it| !is_valid_id(it)).sum());
    fn is_valid_id(id: u64) -> bool {
        // <= instead of < because 10 also happens to be valid
        if id <= 10 {
            return true;
        }
        let mut buf = [0u8; 20];
        let num_digits = count_digits_u64(id) as usize;
        int_to_digits_u64_2(id, num_digits, &mut buf);
        let digits = &buf[..num_digits];
        let ret = compute_chunk_sizes(num_digits).iter().all(|&it| {
            let mut chunks = digits.chunks(it as usize);
            let first = chunks.nth(0).unwrap();
            for it in chunks {
                if it != first {
                    return true;
                }
            }
            return false;
        });
        ret
    }

    #[allow(unused)]
    fn compute_chunk_sizes(size: usize) -> &'static [u8] {
        match size {
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => &[1],
            4 => &[2, 1],
            6 => &[2, 3, 1],
            8 => &[2, 4, 1],
            9 => &[3, 1],
            10 => &[2, 5, 1],
            12 => &[2, 3, 4, 6, 1],
            14 => &[2, 7, 1],
            15 => &[3, 5, 1],
            16 => &[2, 4, 8, 1],
            18 => &[2, 3, 6, 9, 1],
            20 => &[2, 4, 5, 10, 1],
            _ => unreachable!(),
        }
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
