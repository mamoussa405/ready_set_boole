mod nnf;

use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
enum Symbols {
    Char(char),
    Not,
    And,
    Or,
    Xor,
    MatCond,
    LogEq,
}

type RcNode = Rc<RefCell<Option<Box<Node>>>>;

#[derive(Debug, Clone)]
struct Node {
    data: Symbols,
    left: RcNode,
    right: RcNode,
}

impl Node {
    fn new(data: Symbols) -> Self {
        Self {
            data,
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
        }
    }
}

#[derive(Debug)]
pub struct AST {
    root: RcNode,
    stack: Vec<RcNode>,
    insert_left: bool,
}

impl AST {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(None)),
            stack: Vec::new(),
            insert_left: false,
        }
    }

    pub fn build(&mut self, formula: &str) {
        let mut stack: Vec<char> = Vec::new();
        let mut is_oper: bool = false;
        let mut processed: usize = 0;

        for c in formula.as_bytes() {
            match c {
                b'A'..=b'Z' => stack.push(*c as char),
                b'|' => self.add_sub_tree(&mut stack, Symbols::Or),
                b'&' => self.add_sub_tree(&mut stack, Symbols::And),
                b'!' => {
                    if is_oper || (formula.len() - processed == 1) {
                        self.add_not_node(&mut stack, formula.len() - processed == 1);
                    } else {
                        let top: char = stack.pop().unwrap_or_else(|| {
                            panic!("Invalid formula");
                        });

                        if top != '!' {
                            stack.push(top);
                            stack.push(*c as char);
                        }
                    }
                }
                b'^' => self.add_sub_tree(&mut stack, Symbols::Xor),
                b'>' => self.add_sub_tree(&mut stack, Symbols::MatCond),
                b'=' => self.add_sub_tree(&mut stack, Symbols::LogEq),
                _ => panic!("Invalid formula"),
            }
            is_oper = if (*c < b'A' || *c > b'Z') && *c != b'!' {
                true
            } else {
                false
            };
            processed += 1;
        }

        if !stack.is_empty() {
            panic!("Invalid formula");
        }
        if self.stack.len() > 1 {
            panic!("Invalid formula");
        }
        let top: RcNode = self.stack.pop().unwrap_or_else(|| {
            panic!("Invalid formula");
        });

        self.root = Rc::clone(&top);
    }

    pub fn simplify_material_properties(&mut self) {
        nnf::rewrite_equivalence(Rc::clone(&self.root));
        nnf::rewrite_material_conditions(Rc::clone(&self.root));
        nnf::rewrite_xor_operator(Rc::clone(&self.root));
        nnf::remove_double_negations(Rc::clone(&self.root), true);
        // nnf::morgan_law_and_double_negation(self.root.borrow_mut().as_mut(), false);
    }

    fn get_top(&mut self, stack: &mut Vec<char>) -> RcNode {
        let mut top: char = stack.pop().unwrap_or_else(|| {
            panic!("Invalid formula");
        });
        let new_node: RcNode;

        if top == '!' {
            new_node = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));
            top = stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });
            new_node.borrow_mut().as_mut().unwrap().right =
                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Char(top))))));
        } else {
            new_node = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Char(top))))));
        }

        new_node
    }

    fn add_not_node(&mut self, stack: &mut Vec<char>, pop: bool) {
        let new_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

        if self.stack.is_empty() && pop {
            new_node.borrow_mut().as_mut().unwrap().right = self.get_top(stack);
            self.stack.push(new_node);
        } else {
            let rhs: RcNode = self.stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });

            new_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&rhs);
            self.stack.push(new_node);
        }
    }

    fn add_sub_tree(&mut self, stack: &mut Vec<char>, symbol: Symbols) {
        if stack.len() > 1 {
            let new_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));
            self.insert_left = false;

            new_node.borrow_mut().as_mut().unwrap().right = self.get_top(stack);
            new_node.borrow_mut().as_mut().unwrap().left = self.get_top(stack);
            if !stack.is_empty() {
                self.insert_left = true;
            }
            self.stack.push(new_node);
        } else if stack.len() == 1 {
            let top: RcNode = self.stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });
            let new_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));

            new_node.borrow_mut().as_mut().unwrap().right = if !self.insert_left {
                self.get_top(stack)
            } else {
                Rc::clone(&top)
            };
            new_node.borrow_mut().as_mut().unwrap().left = if self.insert_left {
                self.get_top(stack)
            } else {
                Rc::clone(&top)
            };
            self.insert_left = false;
            self.stack.push(new_node);
        } else if !self.stack.is_empty() {
            let rhs: RcNode = self.stack.pop().unwrap();
            let lhs: RcNode = self.stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });
            let new_node: RcNode = Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));

            new_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&rhs);
            new_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&lhs);
            self.stack.push(new_node);
        } else {
            panic!("Invalid formula");
        }
    }
}
