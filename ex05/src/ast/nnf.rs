use super::*;

fn get_equivalence_left_subtree(root: &mut Box<Node>) -> RcNode {
    let mat_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::MatCond)))));

    mat_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&root.left);
    mat_node.borrow_mut().as_mut().unwrap().right = Rc::new(RefCell::new(Some(
        root.right.borrow().as_ref().unwrap().clone(),
    )));

    mat_node
}

fn get_equivalence_right_subtree(root: &mut Box<Node>) -> RcNode {
    let mat_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::MatCond)))));

    mat_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&root.right);
    mat_node.borrow_mut().as_mut().unwrap().right = Rc::new(RefCell::new(Some(
        root.left.borrow().as_ref().unwrap().clone(),
    )));

    mat_node
}

pub fn rewrite_equivalence(curr_node: RcNode) {
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

pub fn rewrite_material_conditions(curr_node: RcNode) {
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
    let and_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::And)))));
    let not_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

    not_node.borrow_mut().as_mut().unwrap().right = Rc::new(RefCell::new(Some(
        curr_node.left.borrow().as_ref().unwrap().clone(),
    )));
    and_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&curr_node.right);
    and_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&not_node);

    and_node
}

pub fn rewrite_xor_operator(curr_node: RcNode) {

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

pub fn eliminate_double_negation(curr_node: RcNode, not_oper_cnt: usize) -> RcNode {
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

pub fn remove_double_negations(curr_node: RcNode) {
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

pub fn remove_not_node(curr_node: RcNode, found_not: bool) -> (RcNode, bool) {
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

pub fn morgan_law(curr_node: RcNode, mut found_not: bool) {
    if curr_node.borrow().is_none() {
        return;
    }
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
