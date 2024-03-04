mod truth_table;

use truth_table::TruthTable;

/// Build and print the truth table of the given formula
/// # Arguments
/// * `formula` - A string slice that holds the formula to be evaluated 
fn build_print_truth_table(formula: &str) -> Vec<Vec<char>> {
    let mut truth_table: TruthTable = TruthTable::new(formula);

    truth_table.fill();
    // this will be a copy of the truth table we will use it in the tests
    let res: Vec<Vec<char>> = truth_table.eval().clone();
    truth_table.print();

    res
}

/// Print the truth table of the given formula
/// # Arguments
/// * `formula` - A string slice that holds the formula to be evaluated
/// # Example
/// ```
/// use ex04::print_truth_table;
/// print_truth_table("AB&");
/// // Output:
/// // | A | B | = |
/// // |---|---|---|
/// // | 0 | 0 | 0 |
/// // | 0 | 1 | 0 |
/// // | 1 | 0 | 0 |
/// // | 1 | 1 | 1 |
/// ```
pub fn print_truth_table(formula: &str) {
    build_print_truth_table(formula);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_table(mut table: Vec<Vec<char>>) -> Vec<String> {
        let res: Vec<String> = table.iter_mut()
            .map(|v: &mut Vec<char>| {
                let tmp: Vec<String> = v.iter_mut()
                    .map(|c: &mut char| { c.to_string() })
                    .collect();
                tmp.join("")
            }).collect();

        res
    }

    #[test]
    fn basic_tests() {
        /*------------------------ Test 1 ------------------------------------- */
        let mut formula: &str = "AB&";
        let mut expected: Vec<String> = 
            vec![
                format!("| {} | {} | = |", 'A', 'B'),
                format!("|---|---|---|"),
                format!("| {} | {} | {} |", '0', '0', '0'),
                format!("| {} | {} | {} |", '0', '1', '0'),
                format!("| {} | {} | {} |", '1', '0', '0'),
                format!("| {} | {} | {} |", '1', '1', '1'),
            ];
        let mut res: Vec<String> = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 2 ------------------------------------- */
        formula = "FE|";
        expected = 
            vec![
                format!("| {} | {} | = |", 'E', 'F'),
                format!("|---|---|---|"),
                format!("| {} | {} | {} |", '0', '0', '0'),
                format!("| {} | {} | {} |", '0', '1', '1'),
                format!("| {} | {} | {} |", '1', '0', '1'),
                format!("| {} | {} | {} |", '1', '1', '1'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 3 ------------------------------------- */
        formula = "NM^";
        expected = 
            vec![
                format!("| {} | {} | = |", 'M', 'N'),
                format!("|---|---|---|"),
                format!("| {} | {} | {} |", '0', '0', '0'),
                format!("| {} | {} | {} |", '0', '1', '1'),
                format!("| {} | {} | {} |", '1', '0', '1'),
                format!("| {} | {} | {} |", '1', '1', '0'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 4 ------------------------------------- */
        formula = "KL=";
        expected = 
            vec![
                format!("| {} | {} | = |", 'K', 'L'),
                format!("|---|---|---|"),
                format!("| {} | {} | {} |", '0', '0', '1'),
                format!("| {} | {} | {} |", '0', '1', '0'),
                format!("| {} | {} | {} |", '1', '0', '0'),
                format!("| {} | {} | {} |", '1', '1', '1'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 4 ------------------------------------- */
        formula = "XY>";
        expected = 
            vec![
                format!("| {} | {} | = |", 'X', 'Y'),
                format!("|---|---|---|"),
                format!("| {} | {} | {} |", '0', '0', '1'),
                format!("| {} | {} | {} |", '0', '1', '1'),
                format!("| {} | {} | {} |", '1', '0', '0'),
                format!("| {} | {} | {} |", '1', '1', '1'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 5 ------------------------------------- */
        formula = "A!";
        expected = 
            vec![
                format!("| {} | = |", 'A'),
                format!("|---|---|"),
                format!("| {} | {} |", '0', '1'),
                format!("| {} | {} |", '1', '0'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
    }

    #[test]
    fn advanced_tests() {
        /*------------------------ Test 1 ------------------------------------- */
        let mut formula: &str = "AB&C|";
        let mut expected: Vec<String> = 
            vec![
                format!("| {} | {} | {} | = |", 'A', 'B', 'C'),
                format!("|---|---|---|---|"),
                format!("| {} | {} | {} | {} |", '0', '0', '0', '0'),
                format!("| {} | {} | {} | {} |", '0', '0', '1', '1'),
                format!("| {} | {} | {} | {} |", '0', '1', '0', '0'),
                format!("| {} | {} | {} | {} |", '0', '1', '1', '1'),
                format!("| {} | {} | {} | {} |", '1', '0', '0', '0'),
                format!("| {} | {} | {} | {} |", '1', '0', '1', '1'),
                format!("| {} | {} | {} | {} |", '1', '1', '0', '1'),
                format!("| {} | {} | {} | {} |", '1', '1', '1', '1'),
            ];
        let mut res: Vec<String> = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 2 ------------------------------------- */
        formula = "AB|C&";
        expected = 
            vec![
                format!("| {} | {} | {} | = |", 'A', 'B', 'C'),
                format!("|---|---|---|---|"),
                format!("| {} | {} | {} | {} |", '0', '0', '0', '0'),
                format!("| {} | {} | {} | {} |", '0', '0', '1', '0'),
                format!("| {} | {} | {} | {} |", '0', '1', '0', '0'),
                format!("| {} | {} | {} | {} |", '0', '1', '1', '1'),
                format!("| {} | {} | {} | {} |", '1', '0', '0', '0'),
                format!("| {} | {} | {} | {} |", '1', '0', '1', '1'),
                format!("| {} | {} | {} | {} |", '1', '1', '0', '0'),
                format!("| {} | {} | {} | {} |", '1', '1', '1', '1'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 3 ------------------------------------- */
        formula = "AB=C>D^";
        expected = 
            vec![
                format!("| {} | {} | {} | {} | = |", 'A', 'B', 'C', 'D'),
                format!("|---|---|---|---|---|"),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '0', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '0', '1', '1'),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '1', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '1', '1', '0'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '0', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '0', '1', '0'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '1', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '1', '1', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '0', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '0', '1', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '1', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '1', '1', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '0', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '0', '1', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '1', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '1', '1', '0'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 4 ------------------------------------- */
        formula = "AB=C>D^!";
        expected = 
            vec![
                format!("| {} | {} | {} | {} | = |", 'A', 'B', 'C', 'D'),
                format!("|---|---|---|---|---|"),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '0', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '0', '1', '0'),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '1', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '0', '0', '1', '1', '1'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '0', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '0', '1', '1'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '1', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '0', '1', '1', '1', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '0', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '0', '1', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '1', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '0', '1', '1', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '0', '0', '1'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '0', '1', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '1', '0', '0'),
                format!("| {} | {} | {} | {} | {} |", '1', '1', '1', '1', '1'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 5 ------------------------------------- */
        formula = "XX=X>X^!";
        expected = 
            vec![
                format!("| {} | = |", 'X'),
                format!("|---|---|"),
                format!("| {} | {} |", '0', '1'),
                format!("| {} | {} |", '1', '1'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
        /*------------------------ Test 6 ------------------------------------- */
        formula = "XY=!X>K^";
        expected = 
            vec![
                format!("| {} | {} | {} | = |", 'K', 'X', 'Y'),
                format!("|---|---|---|---|"),
                format!("| {} | {} | {} | {} |", '0', '0', '0', '1'),
                format!("| {} | {} | {} | {} |", '0', '0', '1', '0'),
                format!("| {} | {} | {} | {} |", '0', '1', '0', '1'),
                format!("| {} | {} | {} | {} |", '0', '1', '1', '1'),
                format!("| {} | {} | {} | {} |", '1', '0', '0', '0'),
                format!("| {} | {} | {} | {} |", '1', '0', '1', '1'),
                format!("| {} | {} | {} | {} |", '1', '1', '0', '0'),
                format!("| {} | {} | {} | {} |", '1', '1', '1', '0'),
            ];
        res = format_table(build_print_truth_table(formula));

        assert_eq!(expected, res);
        /*-------------------------------------------------------------------------*/
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_empty_string_test() {
        build_print_truth_table("");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_just_spaces_test() {
        build_print_truth_table("      ");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test1() {
        build_print_truth_table("AAA!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test2() {
        build_print_truth_table("A");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test3() {
        build_print_truth_table("AB");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test4() {
        build_print_truth_table("AAC|");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_operators_test5() {
        build_print_truth_table("ABCD&=>AB=|CE=F!G=>^^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test1() {
        build_print_truth_table("!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_no_enough_values_test2() {
        build_print_truth_table("!!!!!");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test1() {
        build_print_truth_table("aY=!)>K^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test2() {
        build_print_truth_table("&&!!Abcd");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test3() {
        build_print_truth_table("1111&=>11=|11=1!0=>^");
    }

    #[test]
    #[should_panic(expected = "Invalid formula")]
    fn invalid_formula_unknown_symbol_test4() {
        build_print_truth_table("111|");
    }

}
