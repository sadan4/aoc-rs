pub trait SignExt {
    fn sign(self: Self) -> i8;
}

macro_rules! impl_sign_ext {
    ($t:ty) => {
        impl SignExt for $t {
            fn sign(self: Self) -> i8 {
                if self == 0 {
                    0
                } else if self > 0 {
                    1
                } else {
                    -1
                }
            }
        }
    };
}

impl_sign_ext!(i8);
impl_sign_ext!(i16);
impl_sign_ext!(i32);
impl_sign_ext!(i64);

pub fn count_digits_u64(u: u64) -> u32 {
    // Tower of if statements checking increasing powers of 10.
    /*  */
    if u < 0_000_000_000_000_000_010 {
        1
    } else if u < 0_000_000_000_000_000_100 {
        2
    } else if u < 0_000_000_000_000_001_000 {
        3
    } else if u < 0_000_000_000_000_010_000 {
        4
    } else if u < 0_000_000_000_000_100_000 {
        5
    } else if u < 0_000_000_000_001_000_000 {
        6
    } else if u < 0_000_000_000_010_000_000 {
        7
    } else if u < 0_000_000_000_100_000_000 {
        8
    } else if u < 0_000_000_001_000_000_000 {
        9
    } else if u < 0_000_000_010_000_000_000 {
        10
    } else if u < 0_000_000_100_000_000_000 {
        11
    } else if u < 0_000_001_000_000_000_000 {
        12
    } else if u < 0_000_010_000_000_000_000 {
        13
    } else if u < 0_000_100_000_000_000_000 {
        14
    } else if u < 0_001_000_000_000_000_000 {
        15
    } else if u < 0_010_000_000_000_000_000 {
        16
    } else if u < 0_100_000_000_000_000_000 {
        17
    } else if u < 1_000_000_000_000_000_000 {
        18
    } else {
        19
    }
}

pub fn count_digits(n: i64) -> u32 {
    count_digits_u64(i64_abs_to_u64(n))
}

pub fn i64_abs_to_u64(n: i64) -> u64 {
    if n >= 0 {
        n as u64
    } else if n == i64::MIN {
        i64::MAX as u64 + 1
    } else {
        (-n) as u64
    }
}

pub fn int_to_digits(n: i64) -> Vec<u8> {
    let mut n = i64_abs_to_u64(n);
    let mut count = count_digits_u64(n) as usize;
    // it can have at most 19 digits
    let mut ret = Vec::<u8>::with_capacity(count);
    // we set every entry right after, nothing uninit is ever read
    unsafe { ret.set_len(count); }
    loop {
        count -= 1;
        ret[count] = (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    debug_assert_eq!(count, 0);
    ret
}

#[cfg(test)]
mod tests {
    mod test_count_digits {
        use super::super::count_digits;

        #[test]
        fn test_zero() {
            assert_eq!(count_digits(0), 1);
        }

        #[test]
        fn test_positives() {
            assert_eq!(count_digits(7), 1);
            assert_eq!(count_digits(42), 2);
            assert_eq!(count_digits(123456789), 9);
            assert_eq!(count_digits(9_223_372_036_854_775_807), 19); // i64::MAX
        }

        #[test]
        fn test_negatives() {
            assert_eq!(count_digits(-7), 1);
            assert_eq!(count_digits(-42), 2);
            assert_eq!(count_digits(-123456789), 9);
        }

        #[test]
        fn test_min() {
            assert_eq!(count_digits(i64::MIN), 19); // -9223372036854775808
        }
    }

    mod test_int_to_digits {
        use super::super::int_to_digits;

        #[test]
        fn test_zero() {
            assert_eq!(int_to_digits(0), vec![0]);
        }

        #[test]
        fn test_positives() {
            assert_eq!(int_to_digits(7), vec![7]);
            assert_eq!(int_to_digits(42), vec![4, 2]);
            assert_eq!(int_to_digits(123456789), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(
                int_to_digits(i64::MAX),
                format!("{}", i64::MAX)
                    .chars()
                    .map(|i| i as u8 - '0' as u8)
                    .collect::<Vec<u8>>()
            );
        }

        #[test]
        fn test_negatives() {
            assert_eq!(int_to_digits(-7), vec![7]);
            assert_eq!(int_to_digits(-42), vec![4, 2]);
            assert_eq!(int_to_digits(-123456789), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(
                int_to_digits(i64::MIN),
                format!("{}", i64::MIN)
                    .chars()
                    // skip `-`
                    .skip(1)
                    .map(|i| i as u8 - '0' as u8)
                    .collect::<Vec<u8>>()
            );
        }
    }
}
