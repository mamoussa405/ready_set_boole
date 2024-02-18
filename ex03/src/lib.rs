mod ast;

pub fn eval_formula(formula: &str) -> bool {
    if formula.is_empty() {
        panic!("Invalid formula");
    }
    let mut tree: ast::AST = ast::AST::new();

    tree.build(formula);
    tree.eval()
    // dbg!(tree);
    // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
    }

    #[test]
    fn logical_and_tests() {
        assert!(eval_formula("11&"));
        assert!(!eval_formula("01&"));
        assert!(!eval_formula("10&"));
        assert!(!eval_formula("00&"));
        assert!(!eval_formula("11&1&0&1&"));
        assert!(eval_formula("111&&"));
        assert!(!eval_formula("111&&0&"));
        assert!(eval_formula("1111&&&11&&11&1&1&&"));
        assert!(!eval_formula("1111&&&10&&11&1&1&&"));
    }

    #[test]
    fn logical_or_tests() {
        assert!(eval_formula("11|"));
        assert!(eval_formula("01|"));
        assert!(eval_formula("10|"));
        assert!(!eval_formula("00|"));
        assert!(eval_formula("11|1|0|1|"));
        assert!(eval_formula("111||"));
        assert!(eval_formula("111||0|"));
        assert!(eval_formula("1111|||11||11|1|1||"));
        assert!(!eval_formula("0000|||00||00|0|0||"));
        assert!(eval_formula("1111|||10||11|1|1||"));
    }

    #[test]
    fn logical_xor_tests() {
        assert!(eval_formula("10^"));
        assert!(eval_formula("01^"));
        assert!(!eval_formula("11^"));
        assert!(!eval_formula("00^"));
        assert!(!eval_formula("11^1^0^1^"));
        assert!(eval_formula("111^^"));
        assert!(eval_formula("111^^0^"));
        assert!(!eval_formula("1111^^^11^^11^1^1^^"));
        assert!(!eval_formula("0000^^^00^^00^0^0^^"));
        assert!(eval_formula("1111^^^10^^11^1^1^^"));
    }

    #[test]
    fn material_condition_tests() {
        assert!(!eval_formula("10>"));
        assert!(eval_formula("11>"));
        assert!(eval_formula("01>"));
        assert!(eval_formula("00>"));
        assert!(eval_formula("110>>01>0>>"));
        assert!(!eval_formula("110>>1>0>"));
        assert!(eval_formula("1111>>>11>>11>1>1>>"));
        assert!(!eval_formula("1111>>>11>>11>1>0>>"));
        assert!(eval_formula("0000>>>00>>00>0>0>>"));
        assert!(eval_formula("1111>>>10>>11>1>1>>"));
    }

    #[test]
    fn logical_equivalence_tests() {
        assert!(eval_formula("11="));
        assert!(!eval_formula("01="));
        assert!(!eval_formula("10="));
        assert!(eval_formula("00="));
        assert!(!eval_formula("110==01=0=="));
        assert!(eval_formula("110==1=0="));
        assert!(eval_formula("1111===11==11=1=1=="));
        assert!(!eval_formula("1111===11==11=1=0=="));
        assert!(eval_formula("0000===00==00=0=0=="));
        assert!(!eval_formula("1111===10==11=1=1=="));
    }

    #[test]
    fn logical_not_tests() {
        assert!(!eval_formula("1!"));
        assert!(!eval_formula("1!!!!!!!!!!!!!"));
        assert!(eval_formula("0!"));
        assert!(eval_formula("0!!!!!!!!!!!!!!!"));
        assert!(!eval_formula("0!!"));
        assert!(!eval_formula("1!0|"));
        assert!(eval_formula("1!0!!!|1="));
    }

    #[test]
    fn all_operators_tests() {
        assert!(!eval_formula("1111&=>11=|11=1!0=>^"));
        assert!(!eval_formula("1111&=>11=|11=10!=>^"));
        assert!(!eval_formula("1111&=>11=|11=10=!>^"));
        assert!(!eval_formula("11^!11&=0>11=|11=10=!>^"));
        assert!(!eval_formula("1!1^11&=1>11=|11=10=!>^"));
        assert!(eval_formula("11!1^1&=!11=|11=10=!>^!"));
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_empty_string_test() {
        eval_formula("");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_just_spaces_test() {
        eval_formula("      ");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test2() {
        eval_formula("1");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test3() {
        eval_formula("10");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test4() {
        eval_formula("111|");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test5() {
        eval_formula("1111&=>11=|11=1!0=>^&");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test1() {
        eval_formula("!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test2() {
        eval_formula("!!!!!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test1() {
        eval_formula("aY=!)>K^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test2() {
        eval_formula("&&!!Abcd");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test3() {
        eval_formula("ABCX&=>11=|11=1!0=>^");
    }

}
