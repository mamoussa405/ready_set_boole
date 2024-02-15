mod ast;

pub fn eval_formula(formula: &str) -> bool {
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
}
