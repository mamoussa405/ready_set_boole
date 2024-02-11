/// add two integers a and b using bitwise operators
/// and return the result as a u32
fn adder(a: &mut u32, b: u32) {
    let mut res: u32 = 0;
    let mut carry: bool = false;

    for i in 0..32 {
        /*
            if this true means a and b have different bits at 
            index i, otherwise they have the same bits, either
            0 and 0 or 1 and 1
        */
        if (*a & (1 << i)) != (b & (1 << i)) {
            if !carry {
                res |= 1 << i;
            }
        }
        else {
            if carry {
                res |= 1 << i;
            }
            carry = (*a & (1 << i)) != 0;
        }
    }

    *a = res
}

/// multiply two integers a and b using bitwise operators
/// by calling the adder function from the previous exercise
/// and return the result as a u32
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut min = a.min(b);
    let max = a.max(b);
    let mut res = max;

    if min == 0 {
        return 0;
    }
    min -= 1;
    while min != 0 {
       adder(&mut res, max);
       min -= 1;
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(5 * 2, multiplier(5, 2));
        assert_eq!(4 * 3, multiplier(4, 3));
    }

    #[test]
    fn odd_numbers_test() {
        assert_eq!(7 * 3, multiplier(7, 3));
        assert_eq!(12345 * 6789, multiplier(12345, 6789));
        assert_eq!(7777 * 7777, multiplier(7777, 7777));
    }

    #[test]
    fn even_numbers_test() {
        assert_eq!(2 * 10, multiplier(2, 10));
        assert_eq!(10000 * 4, multiplier(10000, 4));
        assert_eq!(44444 * 66666, multiplier(44444, 66666));
    }

    #[test]
    fn edge_numbers_test() {
        assert_eq!(0 * 10000, multiplier(0, 10000));
        assert_eq!(1 * 0, multiplier(1, 0));
        assert_eq!(0 * 0, multiplier(0, 0));
    }
}