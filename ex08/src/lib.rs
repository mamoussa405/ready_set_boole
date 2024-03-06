use std::collections::HashSet;

fn fill_powerset(
    set: &Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
    tmp_set: &mut Vec<i32>,
    start_index: usize,
) {
    ans.push(tmp_set.clone());

    for i in start_index..set.len() {
        tmp_set.push(set[i]);
        fill_powerset(set, ans, tmp_set, i + 1);
        tmp_set.pop();
    }
}

/// Get the powerset of a given set
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut s: HashSet<i32> = HashSet::new();

    for it in set.iter() {
        s.insert(*it);
    }
    if s.len() != set.len() {
        panic!("Error: Set with duplicate numbers");
    }
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut tmp_set: Vec<i32> = Vec::new();

    fill_powerset(&set, &mut ans, &mut tmp_set, 0);
    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_of_positive_numbers_test() {
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ],
            powerset(vec![1, 2, 3])
        );
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 2, 3, 4],
                vec![1, 2, 4],
                vec![1, 3],
                vec![1, 3, 4],
                vec![1, 4],
                vec![2],
                vec![2, 3],
                vec![2, 3, 4],
                vec![2, 4],
                vec![3],
                vec![3, 4],
                vec![4],
            ],
            powerset(vec![1, 2, 3, 4])
        );
        assert_eq!(
            vec![
                vec![],
                vec![1000],
                vec![1000, 2000],
                vec![2000],
            ],
            powerset(vec![1000, 2000])
        );

    }

    #[test]
    fn set_of_negative_numbers_test() {
        assert_eq!(
            vec![
                vec![],
                vec![-1],
                vec![-1, -2],
                vec![-2],
            ],
            powerset(vec![-1, -2])
        );
        assert_eq!(
            vec![
                vec![],
                vec![-1000],
                vec![-1000, -2000],
                vec![-2000],
            ],
            powerset(vec![-1000, -2000])
        );
    }

    #[test]
    fn set_of_mixed_numbers_test() {
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, -1],
                vec![-1],
            ],
            powerset(vec![1, -1])
        );


        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![1, -2],
                vec![1, -2, 3],
                vec![1, 3],
                vec![-2],
                vec![-2, 3],
                vec![3],
            ],
            powerset(vec![1, -2, 3])
        );
    }

    #[test]
    fn empty_set_test() {
        let expec: Vec<Vec<i32>> = vec![vec![]];

        assert_eq!(
            expec,
            powerset(vec![])
        );
    }


    #[test]
    fn set_of_one_element_test() {
        assert_eq!(
            vec![
                vec![],
                vec![0],
            ],
            powerset(vec![0])
        );
        assert_eq!(
            vec![vec![], vec![1]],
            powerset(vec![1])
        );
    }

    #[test]
    #[should_panic(expected = "Error: Set with duplicate numbers")]
    fn set_with_one_duplicate_element_test() {
        powerset(vec![1, 1, 3]);
    }

    #[test]
    #[should_panic(expected = "Error: Set with duplicate numbers")]
    fn set_with_more_then_one_duplicate_element_test() {
        powerset(vec![1, 1, 3, 3]);
    }
}