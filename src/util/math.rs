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

pub fn count_digits(n: i64) -> u32 {
    // Convert to a non-negative u64 without overflowing.
    // i64::MIN cannot be negated into a positive i64, so handle it explicitly.
    let u = if n >= 0 {
        n as u64
    } else if n == i64::MIN {
        9_223_372_036_854_775_808u64 // absolute value of i64::MIN
    } else {
        (-n) as u64
    };

    // Tower of if statements checking increasing powers of 10.
    if u < 10 { 1 }
    else if u < 100 { 2 }
    else if u < 1_000 { 3 }
    else if u < 10_000 { 4 }
    else if u < 100_000 { 5 }
    else if u < 1_000_000 { 6 }
    else if u < 10_000_000 { 7 }
    else if u < 100_000_000 { 8 }
    else if u < 1_000_000_000 { 9 }
    else if u < 10_000_000_000 { 10 }
    else if u < 100_000_000_000 { 11 }
    else if u < 1_000_000_000_000 { 12 }
    else if u < 10_000_000_000_000 { 13 }
    else if u < 100_000_000_000_000 { 14 }
    else if u < 1_000_000_000_000_000 { 15 }
    else if u < 10_000_000_000_000_000 { 16 }
    else if u < 100_000_000_000_000_000 { 17 }
    else if u < 1_000_000_000_000_000_000 { 18 }
    else { 19 }
}

#[cfg(test)]
mod tests {
    use super::count_digits;

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
