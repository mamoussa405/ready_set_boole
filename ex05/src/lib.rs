mod ast;

use ast::AST;

pub fn negation_normal_form(formula: &str) -> String {
    if formula.is_empty() {
        panic!("Invalid formula");
    }
    let mut tree: AST = AST::new();

    tree.build(formula);
    tree.simplify_material_properties();
    tree.get_rpn_formula()
    // dbg!(tree);
    // "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
        assert_eq!("A!B!|", negation_normal_form("AB&!"));
        assert_eq!("A!B!&", negation_normal_form("AB|!"));
        assert_eq!("A!B|", negation_normal_form("AB>"));
        assert_eq!("A!B|B!A|&", negation_normal_form("AB="));
        assert_eq!("A!B!&C!|", negation_normal_form("AB|C&!"));
    }

    #[test]
    fn more_tests() {
        assert_eq!("A", negation_normal_form("A!!"));
        assert_eq!("A!B&", negation_normal_form("A!B&"));
        assert_eq!("AB&C!D!|&", negation_normal_form("AB&!CD&|!"));
        assert_eq!("EF!&FE!&|M!N|N!M|&|MN!&NM!&|E!F|F!E|&|&", negation_normal_form("EF=MN=="));
        assert_eq!("A!B!C!||", negation_normal_form("ABC&!>"));
        assert_eq!("AB|", negation_normal_form("A!B>"));
        assert_eq!("A!B!&", negation_normal_form("A!B>!"));
        assert_eq!("A!CK&&", negation_normal_form("ACK&!|!"));
        assert_eq!("AB!&BA!&|", negation_normal_form("AB^"));
        assert_eq!("A!B|B!A|&", negation_normal_form("AB^!"));
        assert_eq!("A!B|B!A|&C|", negation_normal_form("AB^C>"));
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_empty_string_test() {
        negation_normal_form("");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_just_spaces_test() {
        negation_normal_form("      ");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test1() {
        negation_normal_form("AAA!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test2() {
        negation_normal_form("A");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test3() {
        negation_normal_form("AB");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test4() {
        negation_normal_form("AAC|");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test5() {
        negation_normal_form("ABCD&=>AB=|CE=F!G=>^^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test1() {
        negation_normal_form("!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test2() {
        negation_normal_form("!!!!!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test1() {
        negation_normal_form("aY=!)>K^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test2() {
        negation_normal_form("&&!!Abcd");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test3() {
        negation_normal_form("1111&=>11=|11=1!0=>^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test4() {
        negation_normal_form("111|");
    }
}
