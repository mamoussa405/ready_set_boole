mod ast;

use ast::AST;
use std::collections::BTreeSet;


pub struct TruthTable {
    width: usize,
    height: usize,
    truth_table: Vec<Vec<char>>,
    start_index: isize,
    zeros_to_fill: usize,
    fill_zero: bool,
}

impl TruthTable {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            truth_table: vec![vec![' '; width]; height],
            start_index: (width - 7) as isize,
            zeros_to_fill: 1,
            fill_zero: true,
        }
    }

    fn fill_table_header(&mut self, chars: BTreeSet<char>) {
        let mut iter = chars.iter();

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

        for i in 1..self.width {
            self.truth_table[1][i] = if i % 4 == 0  { '|' } else { '-' };
        }
    }

    pub fn fill(&mut self, unique_chars: BTreeSet<char>) -> &Vec<Vec<char>> {
        let mut tmp: usize = self.zeros_to_fill;

        self.fill_table_header(unique_chars);
        while self.start_index >= 0 {
            for j in 2..self.truth_table.len() {
                if tmp == 0 {
                    self.fill_zero = !self.fill_zero;
                    tmp = self.zeros_to_fill;
                }
                self.truth_table[j][self.start_index as usize] = if self.fill_zero { '0' } else { '1' };
                tmp -= 1;
            }
            self.fill_zero = true;
            self.zeros_to_fill *= 2;
            tmp = self.zeros_to_fill;
            self.start_index -= 4;
        }
        for i in 2..self.height {
            for j in 0..self.width {
                if j % 4 == 0 {
                    self.truth_table[i][j] = '|';
                }
            }
        }

        &self.truth_table
    }

    pub fn eval(&mut self, formula: &str) {

        for i in 2..self.height {
            let mut ast: AST = AST::new();
            let mut tmp_formula: String = formula.to_string();

            for j in 0..self.width {
                if self.truth_table[0][j] >= 'A' && self.truth_table[0][j] <= 'Z' {
                    tmp_formula = tmp_formula.replace(self.truth_table[0][j], &self.truth_table[i][j].to_string());
                } 
            }
            ast.build(&tmp_formula);
            self.truth_table[i][self.width - 3] = if ast.eval() { '1' } else { '0' };
        }
    }

    pub fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.truth_table[i][j]);
            }
            println!();
        }
    }

}