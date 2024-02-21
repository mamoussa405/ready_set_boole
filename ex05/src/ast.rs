use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
enum Symbols {
    Char(char),
    Not,
    And,
    Or,
    Xor,
    MatCond,
    LogEq,
}

#[derive(Debug)]
struct Node {
    data: Symbols,
    left: Rc<RefCell<Option<Box<Node>>>>,
    right: Rc<RefCell<Option<Box<Node>>>>,
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
    root: Rc<RefCell<Option<Box<Node>>>>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(None)),
        }
    }

    pub fn build(&mut self, formula: &str) {
        let mut stack: Vec<char> = Vec::new();
        let mut insert_right: bool = false;
        let mut is_oper: bool = false;
        let mut processed: usize = 0;


        for c in formula.as_bytes() {
            match c {
                b'A'..=b'Z' => stack.push(*c as char),
                b'|' => self.add_sub_tree(&mut stack, Symbols::Or, insert_right),
                b'&' => self.add_sub_tree(&mut stack, Symbols::And, insert_right),
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
                b'^' => self.add_sub_tree(&mut stack, Symbols::Xor, insert_right),
                b'>' => self.add_sub_tree(&mut stack, Symbols::MatCond, insert_right),
                b'=' => self.add_sub_tree(&mut stack, Symbols::LogEq, insert_right),
                _ => panic!("Invalid formula"),
            }

            match c {
                b'A'..=b'Z' => insert_right = false,
                b'|' | b'&' | b'!' | b'^' | b'>' | b'=' => insert_right = true,
                _ => {}
            }
            is_oper = if *c != b'1' && *c != b'0' && *c != b'!' {
                true
            } else {
                false
            };
            processed += 1;
        }

        if !stack.is_empty() {
            panic!("Invalid formula");
        }
    }

    fn add_not_node(&mut self, stack: &mut Vec<char>, pop: bool) {
        let not_node: Rc<RefCell<Option<Box<Node>>>> =
            Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

        if self.root.borrow_mut().is_none() {
            if pop {
                let top: char = stack.pop().unwrap_or_else(|| {
                    panic!("Invalid formula");
                });

                not_node.borrow_mut().as_mut().unwrap().right = 
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Char(top))))));
            }
            self.root = Rc::clone(&not_node);
        } else {
            not_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&self.root);
            self.root = Rc::clone(&not_node);
        }
    }

    fn add_sub_tree(&mut self, stack: &mut Vec<char>, symbol: Symbols, insert_right: bool) {
        let sub_root: Rc<RefCell<Option<Box<Node>>>> =
            Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));

        if self.root.borrow_mut().is_none() {
            let rhs = stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });
            let lhs = stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });

            sub_root.borrow_mut().as_mut().unwrap().left = 
                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Char(lhs))))));
            sub_root.borrow_mut().as_mut().unwrap().right = 
                Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Char(rhs))))));
            self.root = Rc::clone(&sub_root);
        } else {
            let top = stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });

            if insert_right {
                sub_root.borrow_mut().as_mut().unwrap().left = if top == '1' {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::True)))))
                } else {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::False)))))
                };
                sub_root.borrow_mut().as_mut().unwrap().right = Rc::clone(&self.root);
            } else {
                sub_root.borrow_mut().as_mut().unwrap().right = if top == '1' {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::True)))))
                } else {
                    Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::False)))))
                };
                sub_root.borrow_mut().as_mut().unwrap().left = Rc::clone(&self.root);
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
            Symbols::MatCond => {
                !(self.eval_tree(root.as_ref().unwrap().left.borrow().as_ref())
                    && !self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref()))
            }
            Symbols::LogEq => {
                self.eval_tree(root.as_ref().unwrap().left.borrow().as_ref())
                    == self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref())
            }
            Symbols::Not => {
                !self.eval_tree(root.as_ref().unwrap().right.borrow().as_ref())
            },
            Symbols::True => true,
            Symbols::False => false,
        }
    }
}
