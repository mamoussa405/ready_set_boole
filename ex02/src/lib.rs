/// calculate the gray code of a given number.
/// 
/// # Examples
/// 
/// ```
/// use ex02::gray_code;
/// 
/// assert_eq!(0, gray_code(0));
/// assert_eq!(1, gray_code(1));
/// assert_eq!(3, gray_code(2));
/// ``` 
pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
        assert_eq!(0, gray_code(0));
        assert_eq!(1, gray_code(1));
        assert_eq!(3, gray_code(2));
        assert_eq!(2, gray_code(3));
        assert_eq!(6, gray_code(4));
        assert_eq!(7, gray_code(5));
        assert_eq!(5, gray_code(6));
        assert_eq!(4, gray_code(7));
        assert_eq!(12, gray_code(8));
    }

    #[test]
    fn addition_tests() {
        assert_eq!(26_896, gray_code(19_999));
        assert_eq!(357_913_941, gray_code(429_496_729));
    }
}