mod truth_table;

use truth_table::TruthTable;
use std::collections::BTreeSet;

pub fn print_truth_table(formula: &str) {
    let mut unique_chars: BTreeSet<char>  = BTreeSet::new();

    for c in formula.as_bytes() {
        if *c >= b'A' && *c <= b'Z' {
            unique_chars.insert(*c as char);
        }
    }
    let width: usize = "| x |".len() + (" x |".len() * unique_chars.len());
    let height: usize = (1 << unique_chars.len()) + 2;
    let mut truth_table: TruthTable = TruthTable::new(width, height);

    truth_table.fill(unique_chars);
    truth_table.eval(formula);
    truth_table.print();
}

#[cfg(test)]
mod tests {

}
