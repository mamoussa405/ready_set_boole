use super::*;

fn get_equivalence_left_subtree(root: &mut Box<Node>) -> RcNode {
    let mat_node: RcNode =
        Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::MatCond)))));

    mat_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&root.left);
    mat_node.borrow_mut().as_mut().unwrap().right =
        Rc::new(RefCell::new(Some(root.right.borrow().as_ref().unwrap().clone())));
    
    mat_node
}

fn get_equivalence_right_subtree(root: &mut Box<Node>) -> RcNode {
    let mat_node: RcNode =
        Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::MatCond)))));

    mat_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&root.right);
    mat_node.borrow_mut().as_mut().unwrap().right =
        Rc::new(RefCell::new(Some(root.left.borrow().as_ref().unwrap().clone())));

    mat_node
}

pub fn rewrite_equivalence(mut root: Option<&mut Box<Node>>) {
    if root.is_none() {
        return;
    }

    if let Symbols::LogEq = root.as_ref().unwrap().data {
        let tmp_left: RcNode = get_equivalence_left_subtree(root.as_mut().unwrap());

        root.as_mut().unwrap().data = Symbols::And; 
        root.as_mut().unwrap().right = get_equivalence_right_subtree(root.as_mut().unwrap());
        root.as_mut().unwrap().left = Rc::clone(&tmp_left);
    }
    rewrite_equivalence(root.as_mut().unwrap().left.borrow_mut().as_mut());
    rewrite_equivalence(root.as_mut().unwrap().right.borrow_mut().as_mut());
}

pub fn rewrite_material_conditions(mut root: Option<&mut Box<Node>>) {
    if root.is_none() {
        return;
    }

    if let Symbols::MatCond = root.as_ref().unwrap().data {
        let not_node: RcNode = 
            Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

        root.as_mut().unwrap().data = Symbols::Or;
        not_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&root.as_ref().unwrap().left);
        root.as_mut().unwrap().left = Rc::clone(&not_node);
    }
    rewrite_material_conditions(root.as_mut().unwrap().left.borrow_mut().as_mut());
    rewrite_material_conditions(root.as_mut().unwrap().right.borrow_mut().as_mut());
}

fn get_xor_left_subtree(mut root: &mut Box<Node>) -> RcNode {
    let and_node: RcNode =
        Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::And)))));
    let not_node: RcNode =
        Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

    
    not_node.borrow_mut().as_mut().unwrap().right = 
        Rc::new(RefCell::new(Some(root.right.borrow().as_ref().unwrap().clone())));
    and_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&root.left);
    and_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&not_node);

    and_node
}

fn get_xor_right_subtree(mut root: &mut Box<Node>) -> RcNode {
    let and_node: RcNode =
        Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::And)))));
    let not_node: RcNode =
        Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

    not_node.borrow_mut().as_mut().unwrap().right = 
        Rc::new(RefCell::new(Some(root.left.borrow().as_ref().unwrap().clone())));
    and_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&root.right);
    and_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&not_node);

    and_node
}

pub fn rewrite_xor_operator(mut root: Option<&mut Box<Node>>) {
    if root.is_none() {
        return;
    }

    if let Symbols::Xor = root.as_ref().unwrap().data {
        let tmp_left: RcNode = get_xor_left_subtree(root.as_mut().unwrap());

        root.as_mut().unwrap().data = Symbols::Or;
        root.as_mut().unwrap().right = get_xor_right_subtree(root.as_mut().unwrap());
        root.as_mut().unwrap().left = Rc::clone(&tmp_left);
    }
}

pub fn morgan_law_and_double_negation(mut root: Option<&mut Box<Node>>, found_not: bool) { {
    if root.is_none() {
        return;
    }

    if left Symbols::Node = root.as_ref().unwrap().data {
        match root.as_ref().unwrap().right.borrow().as_ref().unwrap().data {
            Symbols::And | Symbols::Or => {

            },
            Symbols::Not => {

            },

        }
    }
}

}