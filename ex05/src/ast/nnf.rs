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
    and_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&curr_node.right);
    and_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&not_node);

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

fn eliminate_double_negation(curr_node: RcNode, not_oper_cnt: usize) -> RcNode {
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

pub fn remove_double_negations(mut curr_node: RcNode, is_root: bool) {
    if curr_node.borrow().is_none() {
        return;
    }
    if let Symbols::Char(_) = curr_node.borrow().as_ref().unwrap().data {
        return;
    }
    if is_root {
        let mut res: RcNode = Rc::new(RefCell::new(None));
        match curr_node.borrow_mut().as_mut() {
            Some(ref mut node) => match node.data {
                Symbols::Not => {
                    res = eliminate_double_negation(Rc::clone(&curr_node), 0);
                    match res.borrow_mut().as_mut() {
                        Some(ref mut node) => match node.data {
                            Symbols::Not => {
                                remove_double_negations(Rc::clone(&node.right), false)
                            }
                            _ => {
                                remove_double_negations(Rc::clone(&node.right), false);
                                remove_double_negations(Rc::clone(&node.left), false);
                            }
                        },
                        None => {}
                    }

                }
                _ => {
                    remove_double_negations(Rc::clone(&node.right), false);
                    remove_double_negations(Rc::clone(&node.left), false);
                }
            },
            None => {}
        };
        if res.borrow().is_some() {
            curr_node = Rc::clone(&res);
        }
    } 
    // else {
    //     match curr_node.borrow_mut().as_mut() {
    //         Some(ref mut node) => {
    //             // if node.right.borrow_mut().is_some() {
    //                 let mut res: RcNode = Rc::new(RefCell::new(None));
    //                 match node.right.borrow().as_ref().unwrap().data {
    //                     Symbols::Not => {
    //                         res = eliminate_double_negation(Rc::clone(&node.right), 0);
    //                         remove_double_negations(Rc::clone(&res.borrow().as_ref().unwrap().right), false);
    //                     }
    //                     _ => remove_double_negations(Rc::clone(&node.right), false),
    //                 };
    //                 // if res.borrow().is_some() {
    //                     node.right = Rc::clone(&res);
    //                 // }
    //             // }
    //             // if node.left.borrow_mut().is_some() {
    //                 let mut res: RcNode = Rc::new(RefCell::new(None));
    //                 match node.left.borrow().as_ref().unwrap().data {
    //                     Symbols::Not => {
    //                         res = eliminate_double_negation(Rc::clone(&node.left), 0);
    //                         remove_double_negations(Rc::clone(&res.borrow().as_ref().unwrap().left), false);
    //                     }
    //                     _ => remove_double_negations(Rc::clone(&node.left), false),
    //                 };
    //                 // if res.borrow().is_some() {
    //                     node.left = Rc::clone(&res);
    //                 // }
    //             // }
    //         },
    //         None => {}
    //     }
    // }
}
