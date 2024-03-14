mod truth_table;
mod ast;

use ast::AST;
use truth_table::TruthTable;

fn move_conjunctions_to_end(formula: &str) -> String {
    let mut res: String = String::new();
    let mut conjunction_cnt: usize = 0;

    for c in formula.as_bytes() {
        if *c != b'&' {
            res += &(*c as char).to_string();
        }
        conjunction_cnt += (*c == b'&') as usize;
    }

    for _i in 0..conjunction_cnt {
        res += &"&";
    }

    res
}

/// Get conjunctive normal form
/// # Arguments
/// * `formula` -- The formula to get the cnf for
pub fn conjunctive_normal_form(formula: &str) -> String {
    if formula.is_empty() {
        panic!("Invalid formula");
    }
    let mut tree: AST = AST::new();
    
    tree.build(formula, true);
    tree.simplify_material_properties();
    let nnf: String = tree.get_rpn_formula();
    if tree.is_valid_cnf() {
        return move_conjunctions_to_end(&nnf);
    }
    let mut truth_table: TruthTable = TruthTable::new(&nnf);

    truth_table.fill();
    truth_table.eval();
    move_conjunctions_to_end(&truth_table.get_cnf_formula())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
        assert_eq!("A!B!|",  conjunctive_normal_form("AB&!"));
        assert_eq!("A!B!&",  conjunctive_normal_form("AB|!"));
        assert_eq!("AB|C&",  conjunctive_normal_form("AB|C&"));
        assert_eq!("AB|C|D|",  conjunctive_normal_form("AB|C|D|"));
        assert_eq!("ABCD&&&",  conjunctive_normal_form("AB&C&D&"));
        assert_eq!("A!B!|C!|",  conjunctive_normal_form("AB&!C!|"));
        assert_eq!("A!B!C!&&",  conjunctive_normal_form("AB|!C!&"));
    }

    #[test]
    fn more_tests() {
        assert_eq!("CK|L|M|CK|L|M!|CK|L!|M|CK!|L|M|CK!|L|M!|&&&&", conjunctive_normal_form("ML&KL&|C|"));
        assert_eq!("CK|L!|M|CK!|L|M!|&", conjunctive_normal_form("ML^KL=>C|"));
        assert_eq!("A!A&", conjunctive_normal_form("A!A&"));
        assert_eq!("A!", conjunctive_normal_form("A!"));
        assert_eq!("AB|C|AB|C!|A!B|C|A!B!|C|&&&", conjunctive_normal_form("AB|!AC!&^!"));
        assert_eq!("AB|D|AB!|D!|A!B|D!|A!B!|D!|&&&", conjunctive_normal_form("AB=B>D^"));
        assert_eq!("AB|", conjunctive_normal_form("AB=B="));
        assert_eq!("AB|D|", conjunctive_normal_form("AB=B=D|"));
        assert_eq!("AB|D|AB!|D|A!B!|D|&&", conjunctive_normal_form("AB>D>"));
    }
    
    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_empty_string_test() {
        conjunctive_normal_form("");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_just_spaces_test() {
        conjunctive_normal_form("      ");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test1() {
        conjunctive_normal_form("AAA!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test2() {
        conjunctive_normal_form("A");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test3() {
        conjunctive_normal_form("AB");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test4() {
        conjunctive_normal_form("AAC|");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test5() {
        conjunctive_normal_form("ABCD&=>AB=|CE=F!G=>^^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test1() {
        conjunctive_normal_form("!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test2() {
        conjunctive_normal_form("!!!!!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test1() {
        conjunctive_normal_form("aY=!)>K^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test2() {
        conjunctive_normal_form("&&!!Abcd");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test3() {
        conjunctive_normal_form("1111&=>11=|11=1!0=>^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test4() {
        conjunctive_normal_form("111|");
    }
}