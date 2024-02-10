pub fn adder(a: u32, b: u32) -> u32 {
    let mut res: u32 = 0;
    let mut carry: bool = false;

    for i in 0..32 {
        if (a & (1 << i)) != (b & (1 << i)) {
            if !carry {
                res |= 1 << i;
            }
            carry = false;
        }
        else {
            if carry {
                res |= 1 << i;
            }
            carry = (a & (1 << i)) != 0;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_add() {
        assert_eq!(3 + 3, adder(3, 3));
        assert_eq!(10 + 3, adder(10, 3));
    }

    #[test]
    fn odd_numbers_add() {
        assert_eq!(9 + 31, adder(9, 31));
        assert_eq!(11 + 21, adder(11, 21));
        assert_eq!(15 + 101, adder(15, 101));
    }

    #[test]
    fn even_numbers_add() {
        assert_eq!(8 + 30, adder(8, 30));
        assert_eq!(12 + 22, adder(12, 22));
        assert_eq!(16 + 102, adder(16, 102));
    }

    #[test]
    fn edge_numbers_add() {
        assert_eq!(0 + 0, adder(0, 0));
        assert_eq!(1 + 1, adder(1, 1));
        assert_eq!(0 + 1, adder(0, 1));
        assert_eq!(4294967294 + 1, adder(4294967294, 1));
    }
}