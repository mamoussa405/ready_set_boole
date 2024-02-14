use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
enum Symbols {
    False,
    True,
    Not,
    And,
    Or,
    Xor,
    MaterialCondition,
    LogicalEquivalence,
}

#[derive(Debug)]
struct Node {
    data: Symbols,
    left: Rc<RefCell<Option<Box<Node>>>>,
    right: Rc<RefCell<Option<Box<Node>>>>,
}

#[derive(Debug)]
pub struct AST {
    root: Rc<RefCell<Option<Box<Node>>>>,
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

impl AST {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(None)),
        }
    }

    pub fn build(&mut self, formula: &str) {
        let mut stack: Vec<char> = Vec::new();

        for c in formula.as_bytes() {
            match c {
                b'1' | b'0' => stack.push(*c as char),
                b'|' => self.add_sub_tree(&mut stack, Symbols::Or),
                b'&' => self.add_sub_tree(&mut stack, Symbols::And),
                b'!' => self.add_sub_tree(&mut stack, Symbols::Not),
                b'^' => self.add_sub_tree(&mut stack, Symbols::Xor),
                b'>' => self.add_sub_tree(&mut stack, Symbols::MaterialCondition),
                b'=' => self.add_sub_tree(&mut stack, Symbols::LogicalEquivalence),
                _ => panic!("Invalid formula"),
            }
        }

        if !stack.is_empty() {
            panic!("Invalid formula");
        }
    }

    fn add_sub_tree(&mut self, stack: &mut Vec<char>, symbol: Symbols) {
        let sub_root: Rc<RefCell<Option<Box<Node>>>> =
            Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));

        if self.root.borrow_mut().is_none() {
            let rhs = stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });
            let lhs = stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });

            sub_root.borrow_mut().as_mut().unwrap().left = if lhs == '1' {
                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::True)))))
            } else {
                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::False)))))
            };
            sub_root.borrow_mut().as_mut().unwrap().right = if rhs == '1' {
                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::True)))))
            } else {
                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::False)))))
            };
            self.root = Rc::clone(&sub_root);
        } else {
            let top = stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });


            if stack.is_empty() {
                sub_root.borrow_mut().as_mut().unwrap().right = if top == '1' {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::True)))))
                } else {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::False)))))
                };
                sub_root.borrow_mut().as_mut().unwrap().left = Rc::clone(&self.root);
            } else {
                sub_root.borrow_mut().as_mut().unwrap().left = if top == '1' {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::True)))))
                } else {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::False)))))
                };
                sub_root.borrow_mut().as_mut().unwrap().right = Rc::clone(&self.root);
            }
            self.root = Rc::clone(&sub_root);
        }
    }

    pub fn eval(&self) -> bool {
        self.eval_tree(self.root.borrow().as_ref())
    }

    fn eval_tree(&self, root: Option<&Box<Node>>) -> bool {
        match root.as_ref().unwrap().data {
            Symbols::And => {
                self.eval_tree(root.as_ref().unwrap().left.borrow().as_ref())
                    & self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref())
            }
            Symbols::Or => {
                self.eval_tree(root.as_ref().unwrap().left.borrow().as_ref())
                    | self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref())
            }
            Symbols::Xor => {
                self.eval_tree(root.as_ref().unwrap().left.borrow().as_ref())
                    ^ self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref())
            }
            Symbols::MaterialCondition => {
                !(self.eval_tree(root.as_ref().unwrap().left.borrow().as_ref()) &&
                    !self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref()))
            },
            Symbols::LogicalEquivalence => {
                self.eval_tree(root.as_ref().unwrap().left.borrow().as_ref())
                    == self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref())
            },
            Symbols::True => true,
            Symbols::False => false,
            _ => false,
        }
    }
}
