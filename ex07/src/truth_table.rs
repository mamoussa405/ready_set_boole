mod ast;

use ast::AST;
use std::collections::BTreeSet;

/// TruthTable struct
pub struct TruthTable {
    width: usize,
    height: usize,
    truth_table: Vec<Vec<char>>,
    unique_chars: BTreeSet<char>,
    formula: String,
    start_index: isize,
    zeros_to_fill: usize,
    fill_zero: bool,
}

impl TruthTable {
    /// Get a new TruthTable instance
    /// # Arguments
    /// *`forumla` -- The logical function to build the truth table for
    pub fn new(formula: &str) -> Self {
        if formula.is_empty() {
            panic!("Invalid formula");
        }
        let mut unique_chars: BTreeSet<char>  = BTreeSet::new();

        // Get the unique characters in the formula using a set
        for c in formula.as_bytes() {
            if *c >= b'A' && *c <= b'Z' {
                unique_chars.insert(*c as char);
            }
        }
        if unique_chars.is_empty() {
            panic!("Invalid formula");
        }

        /*
            The width of the truth table is calculated as follows:
            - The width of the first column is 5 characters long (| x |)
            - The width of the rest of the columns is 4 characters long ( x |) * number of unique characters
        */
        let width: usize = "| x |".len() + (" x |".len() * unique_chars.len());
        /*
            The height of the truth table is calculated as follows:
                2 ^ number of unique characters + 2
            the + 2 is for the header of the truth table
            Example of a valid truth table for the formula "AB&" (A and B):
                | A | B | = |
                |---|---|---|
                | 0 | 0 | 0 |
                | 0 | 1 | 0 |
                | 1 | 0 | 0 |
                | 1 | 1 | 1 |
        */
        let height: usize = (1 << unique_chars.len()) + 2;

        Self {
            width,
            height,
            truth_table: vec![vec![' '; width]; height],
            unique_chars,
            formula: formula.to_string(),
            // The start column index from which we will start filling the truth table.
            start_index: (width - 7) as isize,
            /*
                This variable will be used to store the number of zeros or ones we should fill in each
                column in each time before switching to the other value.
             */
            zeros_to_fill: 1,
            // a boolean to know if we should fill with zeros or switch to ones.
            fill_zero: true,
        }
    }

    /// Fill the header of the truth table with the unique characters of the formula.
    fn fill_table_header(&mut self) {
        let mut iter = self.unique_chars.iter();

        /*
            Fill the first row of the truth table with the unique characters of the formula.
            | A | B | C | = |
         */
        self.truth_table[0][0] = '|';
        self.truth_table[1][0] = '|';
        for i in 1..self.width {
            if i % 4 == 0 {
                self.truth_table[0][i] = '|';
                self.truth_table[0][i - 2] = if i == self.width - 1 {
                    '='
                } else {
                    *(iter.next().unwrap())
                };
            }
        }
        /*
            Fill the second row of the truth table with the '|' and the '-' characters.
            |---|---|---|---|
         */
        for i in 1..self.width {
            self.truth_table[1][i] = if i % 4 == 0  { '|' } else { '-' };
        }
    }

    /// Fill the truth table with the values of the variables.
    pub fn fill(&mut self) {
        let mut tmp: usize = self.zeros_to_fill;

        self.fill_table_header();
        // Iterate over each column and start filling it with the zeros and ones, 
        /*
            Iterate over each column and start filling it with the zeros and ones
            based on its order, the column before the last one will be filled first.
         */
        while self.start_index >= 0 {
            for j in 2..self.height {
                /*
                    The tmp here is the number of zeros or ones we should fill in the current column
                    in each time, if it's 0 then we should switch to the other value (if 0 then 1, if 1 then 0).
                 */
                if tmp == 0 {
                    self.fill_zero = !self.fill_zero;
                    tmp = self.zeros_to_fill;
                }
                self.truth_table[j][self.start_index as usize] = if self.fill_zero { '0' } else { '1' };
                tmp -= 1;
            }
            /*
                Update the following variables:
                - fill_zero: set it to true to start filling the next column with zeros.
                - zeros_to_fill: multiply it by 2 to fill the next column with twice the number of zeros in each time.
                - start_index: decrement it by 4 to move to the next column.
             */
            self.fill_zero = true;
            self.zeros_to_fill *= 2;
            tmp = self.zeros_to_fill;
            self.start_index -= 4;
        }
        // start filling each row with the '|' character in the correct position.
        for i in 2..self.height {
            for j in 0..self.width {
                if j % 4 == 0 {
                    self.truth_table[i][j] = '|';
                }
            }
        }
    }

    /// Evaluate each row in the truth table and fill the last column with the result.
    pub fn eval(&mut self) -> &Vec<Vec<char>> {

        for i in 2..self.height {
            // create a new AST for each row in the truth table.
            let mut ast: AST = AST::new();
            // clone the formula and replace the variables with their values in the current row.
            let mut tmp_formula: String = self.formula.to_string();

            for j in 0..self.width {
                if self.truth_table[0][j] >= 'A' && self.truth_table[0][j] <= 'Z' {
                    tmp_formula = tmp_formula.replace(self.truth_table[0][j], &self.truth_table[i][j].to_string());
                } 
            }
            // build the AST, evaluate it and store the result in the last column of the truth table.
            ast.build(&tmp_formula, false);
            self.truth_table[i][self.width - 3] = if ast.eval() { '1' } else { '0' };
        }

        &self.truth_table
    }

    /// Check if the formula is satisfiable
    pub fn is_sat(&self) -> bool {

        for i in 2..self.height {
            if self.truth_table[i][self.width - 3] == '1' {
                return true;
            }
        }

        false
    }

}