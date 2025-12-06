use advent_of_code::util::math::count_digits_u64;

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
        let num_digits = count_digits_u64(id) as usize;
        const fn divmod(n: u64, divisor: u64) -> (u64, u64) {
            (n / divisor, n % divisor)
        }
        macro_rules! check_single_digit {
            () => {
                'done: {
                    let mut digits = id;
                    let mask = 10;
                    let (mut rest, left) = divmod(digits, mask);
                    for _ in 1..num_digits {
                        digits = rest;
                        let (new_rest, it) = divmod(digits, mask);
                        if it != left {
                            break 'done true;
                        }
                        rest = new_rest;
                    }
                    break 'done false;
                }
            };
        }
        macro_rules! check_digit_with_num {
            ($n:literal) => {
                'done: {
                    let mut digits = id;
                    const MASK: u64 = 10u64.pow($n as u32);
                    let (mut rest, left) = divmod(digits, MASK);
                    for _ in 1..(num_digits / $n) {
                        digits = rest;
                        let (new_rest, it) = divmod(digits, MASK);
                        if it != left {
                            break 'done true;
                        }
                        rest = new_rest;
                    }
                    break 'done false;
                }
            };
        }
        match num_digits {
            // these match arms must not be merged
            // because if they are merged the compiler will not unroll the loops inside the macros
            2 => {
                return check_single_digit!();
            }
            3 => {
                return check_single_digit!();
            }
            5 => {
                return check_single_digit!();
            }
            7 => {
                return check_single_digit!();
            }
            11 => {
                return check_single_digit!();
            }
            4 => {
                return check_digit_with_num!(2);
            }
            6 => {
                return check_digit_with_num!(2) && check_digit_with_num!(3);
            }
            8 => {
                return check_digit_with_num!(4);
            }
            9 => {
                return check_digit_with_num!(3) && check_single_digit!();
            }
            10 => {
                return check_digit_with_num!(2) && check_digit_with_num!(5);
            }
            _ => unreachable!(),
        };
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
