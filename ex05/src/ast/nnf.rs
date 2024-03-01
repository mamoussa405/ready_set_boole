use super::*;

fn get_equivalence_left_subtree(root: &mut Box<Node>) -> RcNode {
    /*
        This function will return the left subtree of the equivalence operator
        The algorithm is as follows:
        1. Create a new node with the material condition operator
        2. Set the left subtree to the left subtree of the root
        3. Set the right subtree to a clone of the right subtree of the root
        4. Return the new node
            +---------------------+       +--------------------+
            | equivalence subtree |       | material condition |
            +---------------------+       +--------------------+
                    <=>                            =>
                   /   \                          /  \
                  /     \           --->         /    \ 
                 /       \                      /      \
                A         B                    A        B'
        PS: B' is a clone of B
     */
    let mat_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::MatCond)))));

    mat_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&root.left);
    mat_node.borrow_mut().as_mut().unwrap().right = Rc::new(RefCell::new(Some(
        root.right.borrow().as_ref().unwrap().clone(),
    )));

    mat_node
}

fn get_equivalence_right_subtree(root: &mut Box<Node>) -> RcNode {
    /*
        This function will return the right subtree of the equivalence operator
        The algorithm is as follows:
        1. Create a new node with the material condition operator
        2. Set the left subtree to the right subtree of the root
        3. Set the right subtree to a clone of the left subtree of the root
        4. Return the new node
            +---------------------+       +--------------------+
            | equivalence subtree |       | material condition |
            +---------------------+       +--------------------+
                    <=>                            =>
                   /   \                          /  \
                  /     \           --->         /    \ 
                 /       \                      /      \
                A         B                    B        A'
        PS: A' is a clone of A
     */
    let mat_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::MatCond)))));

    mat_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&root.right);
    mat_node.borrow_mut().as_mut().unwrap().right = Rc::new(RefCell::new(Some(
        root.left.borrow().as_ref().unwrap().clone(),
    )));

    mat_node
}

/// Rewrite the equivalence operator
/// # Arguments
/// * `curr_node` - The root of the AST
pub fn rewrite_equivalence(curr_node: RcNode) {
    /*
        In this function, we will rewrite the equivalence operator following this rule:
        (A <=> B) <=> (A => B) & (B => A)
        The algorithm is as follows:
        1. Iterate through the AST
        2. If the current node is an equivalence operator, change it to an AND operator
        3. Update the left subtree with the material condition (A => B)
        4. Update the right subtree with the material condition (B => A)
            +---------------------+       +--------------------+
            | equivalence subtree |       |    rewrite rule    |
            +---------------------+       +--------------------+
                    <=>                             &
                   /   \                          /   \
                  /     \           --->         /     \ 
                 /       \                      /       \
                A         B                    =>        =>
                                              /  \      /  \
                                             A    B'   B    A'
        PS: A' is a clone of A, B' is a clone of B
     */
    match curr_node.borrow_mut().as_mut() {
        Some(ref mut node) => {
            if let Symbols::LogEq = node.data {
                let tmp_left: RcNode = get_equivalence_left_subtree(node);

                node.data = Symbols::And;
                node.right = get_equivalence_right_subtree(node);
                node.left = Rc::clone(&tmp_left);
            }
            rewrite_equivalence(Rc::clone(&node.left));
            rewrite_equivalence(Rc::clone(&node.right));
        },
        None => {}
    }
}

/// Rewrite the implication operator
/// # Arguments
/// * `curr_node` - The root of the AST
pub fn rewrite_material_conditions(curr_node: RcNode) {
    /*
        In this function, we will rewrite the material condition operator following this rule:
        (A => B) <=> !A | B
        The algorithm is as follows:
        1. Iterate through the AST
        2. If the current node is a material condition operator, change it to an OR operator
        3. Update the left subtree with the negation of A
            +---------------------+       +--------------------+
            | material condition  |       |    rewrite rule    |
            +---------------------+       +--------------------+
                    =>                             |
                   /  \                           / \
                  /    \           --->          /   \ 
                 /      \                       /     \
                A        B                    !A       B
     */
    match curr_node.borrow_mut().as_mut() {
        Some(ref mut node) => {
            if let Symbols::MatCond = node.data {
                let not_node: RcNode =
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

                node.data = Symbols::Or;
                not_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&node.left);
                node.left = Rc::clone(&not_node);
            }
            rewrite_material_conditions(Rc::clone(&node.left));
            rewrite_material_conditions(Rc::clone(&node.right));
        },
        None => {}
    }
}

fn get_xor_left_subtree(curr_node: &mut Box<Node>) -> RcNode {
    /*
        This function will return the left subtree of the XOR operator
        The algorithm is as follows:
        1. Create a new node with the AND operator
        2. Create a new node with the NOT operator
        3. Set the right subtree of the NOT operator to a clone of the right subtree of the root
        4. Set the left subtree of the AND operator to the left subtree of the root
        5. Set the right subtree of the AND operator to the NOT operator
        6. Return the AND operator
            +---------------------+       +--------------------+
            |    XOR operator    |       |    rewrite rule    |
            +---------------------+       +--------------------+
                    ^                             &
                   / \                           / \
                  /   \           --->          /   \ 
                 /     \                       /     \
                A       B                     A      !B
     */
    let and_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::And)))));
    let not_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

    not_node.borrow_mut().as_mut().unwrap().right = Rc::new(RefCell::new(Some(
        curr_node.right.borrow().as_ref().unwrap().clone(),
    )));
    and_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&curr_node.left);
    and_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&not_node);

    and_node
}

fn get_xor_right_subtree(curr_node: &mut Box<Node>) -> RcNode {
    /*
        This function will return the right subtree of the XOR operator
        The algorithm is as follows:
        1. Create a new node with the AND operator
        2. Create a new node with the NOT operator
        3. Set the right subtree of the NOT operator to a clone of the left subtree of the root
        4. Set the left subtree of the AND operator to the right subtree of the root
        5. Set the right subtree of the AND operator to the NOT operator
        6. Return the AND operator
            +---------------------+       +--------------------+
            |    XOR operator    |       |    rewrite rule    |
            +---------------------+       +--------------------+
                    ^                             &
                   / \                           / \
                  /   \           --->          /   \ 
                 /     \                       /     \
                A       B                     B      !A
     */
    let and_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::And)))));
    let not_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

    not_node.borrow_mut().as_mut().unwrap().right = Rc::new(RefCell::new(Some(
        curr_node.left.borrow().as_ref().unwrap().clone(),
    )));
    and_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&curr_node.right);
    and_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&not_node);

    and_node
}

/// Rewrite the XOR operator
/// # Arguments
/// * `curr_node` - The root of the AST
pub fn rewrite_xor_operator(curr_node: RcNode) {
    /*
        In this function, we will rewrite the XOR operator following this rule:
        A ^ B <=> (A & !B) | (B & !A)
        The algorithm is as follows:
        1. Iterate through the AST
        2. If the current node is an XOR operator, change it to an OR operator
        3. Update the left subtree with the AND operator (A & !B)
        4. Update the right subtree with the AND operator (B & !A)
            +---------------------+       +--------------------+
            |    XOR operator    |       |    rewrite rule    |
            +---------------------+       +--------------------+
                    ^                             |
                   / \                           / \
                  /   \           --->          /   \ 
                 /     \                       /     \
                A       B                     &       &
                                             / \     / \
                                            A  !B   B  !A
     */
    match curr_node.borrow_mut().as_mut() {
        Some(ref mut node) => {
            if let Symbols::Xor = node.data {
                let tmp_left: RcNode = get_xor_left_subtree(node);

                node.data = Symbols::Or;
                node.right = get_xor_right_subtree(node);
                node.left = Rc::clone(&tmp_left);
            }
            rewrite_xor_operator(Rc::clone(&node.right));
            rewrite_xor_operator(Rc::clone(&node.left));
        },
        None => {}
    }
}

/// Eliminate double negation from the current subtree
/// # Arguments
/// * `curr_node` - The root of the current subtree
/// * `not_oper_cnt` - The number of NOT operators
pub fn eliminate_double_negation(curr_node: RcNode, not_oper_cnt: usize) -> RcNode {
    /*
        In this function, we will eliminate the double negation following this rule:
        !!A <=> A
        The algorithm is as follows:
        1. Iterate through the AST
        2. If the current node is a NOT operator, increment the counter
        3. If the counter is even, return the current node
        4. If the counter is odd, return a new NOT operator with the current node as the right subtree
     */
    match curr_node.borrow_mut().as_mut() {
        Some(ref mut node) => match node.data {
            Symbols::Not => 
                eliminate_double_negation(Rc::clone(&node.right), not_oper_cnt + 1),
            _ => {
                if not_oper_cnt % 2 == 0 {
                    return Rc::clone(&curr_node);
                } else {
                    let not_node =
                        Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));
                    
                    not_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&curr_node);
                    return not_node;
                }
            }
        },
        None => {
            Rc::new(RefCell::new(None))
        }
    }

}


/// Remove double negations from the AST
/// # Arguments
/// * `curr_node` - The root of the AST
pub fn remove_double_negations(curr_node: RcNode) {
    /*
        Call the eliminate_double_negation function for each node in the AST
     */
    if curr_node.borrow().is_none() {
        return;
    }
    if let Symbols::Char(_) = curr_node.borrow().as_ref().unwrap().data {
        return;
    }
    match curr_node.borrow_mut().as_mut() {
        Some(ref mut node) => {
            node.right = eliminate_double_negation(Rc::clone(&node.right), 0);
            node.left = eliminate_double_negation(Rc::clone(&node.left), 0);
            remove_double_negations(Rc::clone(&node.left));
            remove_double_negations(Rc::clone(&node.right));
        },
        None => {}
    }
}

/// Remove the NOT operator from the current subtree when applying De Morgan's laws
/// # Arguments
/// * `curr_node` - The root of the current subtree
/// * `found_not` - A boolean indicating if a NOT operator was found
pub fn remove_not_node(curr_node: RcNode, found_not: bool) -> (RcNode, bool) {
    /*
        In this function, we will remove the NOT operator from the current subtree when applying De Morgan's laws
        The algorithm is as follows:
        1. Iterate through the AST
        2. If the current node is a NOT operator, call the function recursively with the right subtree and the found_not variable
        3. If the current node is an AND or OR operator, return a tuple with the current node and true,
            indicating that the subtree should be called recursively
        4. If the current node is a leaf, return a new NOT operator with the current node as the right subtree if we found a NOT operator
            while iterating through the AST or the current node if we didn't find a NOT operator with the option to call the subtree recursively
            set to false.
     */
    match curr_node.borrow().as_ref() {
        Some(ref node) => {
            if let Symbols::Not = node.data {
                remove_not_node(Rc::clone(&node.right), found_not)
            } else {
                match node.data {
                    Symbols::And | Symbols::Or => {
                        return (Rc::clone(&curr_node), true);
                    },
                    _ => {
                        if found_not {
                            let not_node: RcNode =
                                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

                            not_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&curr_node);
                            return (not_node, false);
                        } else {
                            return (Rc::clone(&curr_node), false);
                        }
                    }
                } 
            }
        },
        None => {
            return (Rc::new(RefCell::new(None)), false);
        }
    }
}

/// Apply De Morgan's law to the AST
/// # Arguments
/// * `curr_node` - The root of the AST
/// * `found_not` - A boolean indicating if a NOT operator was found
pub fn morgan_law(curr_node: RcNode, mut found_not: bool) {
    /*
        In this function, we will apply De Morgan's laws to the AST
        The algorithm is as follows:
        1. Iterate through the AST
        2. If the current node is an AND or OR operator, change it to the other operator if we found a NOT operator
        3. If the current node is a NOT operator, change the found_not variable to its negation
        4. Call the remove_not_node function for the left and right subtrees, and decide if we should call the subtree recursively
            if after removing the NOT operator we still have an AND or OR operator.
     */
    match curr_node.borrow_mut().as_mut() {
        Some(ref mut node) => {
            if found_not {
                match node.data {
                    Symbols::And => node.data = Symbols::Or,
                    Symbols::Or => node.data = Symbols::And,
                    _ => {}
                }
            }
            let tmp: bool = found_not;
            if let Symbols::Not = node.right.borrow().as_ref().unwrap().data {
                found_not = !found_not;
            }
            let (right_subtree, call_subtree) = remove_not_node(Rc::clone(&node.right), found_not);
            
            node.right = right_subtree;
            if call_subtree {
                morgan_law(Rc::clone(&node.right), found_not);
            }

            found_not = tmp;
            if let Symbols::Not = node.left.borrow().as_ref().unwrap().data {
                found_not = !found_not;
            }
            let (left_subtree, call_subtree) = remove_not_node(Rc::clone(&node.left), found_not);
            
            node.left = left_subtree;
            if call_subtree {
                morgan_law(Rc::clone(&node.left), found_not);
            }
        },
        None => {}
    }
}

/// Get the RPN formula from the AST using Post Order Traversal
/// # Arguments
/// * `curr_node` - The root of the AST
pub fn get_rpn_formula(curr_node: RcNode) -> String {
    let mut res: String = String::new();

    match curr_node.borrow().as_ref() {
        Some(ref node) => {
            res += &get_rpn_formula(Rc::clone(&node.left));
            res += &get_rpn_formula(Rc::clone(&node.right));
            match node.data {
                Symbols::And => res += "&",
                Symbols::Or => res += "|",
                Symbols::Not => res += "!",
                Symbols::Char(c) => res += &c.to_string(),
                _ => {}
            };

            res
        },
        None => "".to_string()
    }
}
