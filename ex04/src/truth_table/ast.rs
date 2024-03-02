use std::{cell::RefCell, fmt::Debug, rc::Rc};

/// The possible tokens in the AST
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

/// The possible var tokens, either 1, 0 or A..Z
enum CharType {
    Var,
    ZeroOne,
    None,
}

type RcNode = Rc<RefCell<Option<Box<Node>>>>;

/// The AST node
#[derive(Debug, Clone)]
pub struct Node {
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
    not_cnt: usize,
}

impl AST {
    /// Get new AST instance
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(None)),
            stack: Vec::new(),
            insert_left: false,
            not_cnt: 0,
        }
    }

    /// Build the AST from a string
    /// # Arguments
    /// * `formula` - A string slice that holds the formula
    /// # Panics
    /// If the formula is invalid
    pub fn build(&mut self, formula: &str) {
        let mut stack: Vec<char> = Vec::new();
        let mut is_oper: bool = false;
        let mut processed: usize = 0;
        let mut var_type: CharType = CharType::None;


        for c in formula.as_bytes() {
            match c {
                b'1' | b'0' | b'A'..=b'Z' => { 
                    match var_type {
                        CharType::ZeroOne if *c >= b'A' && *c <= b'Z' => panic!("Invalid formula"),
                        CharType::Var if *c == b'1' || *c == b'0' => panic!("Invalid formula"),
                        CharType::None => {
                            var_type = if *c == b'1' || *c == b'0' {
                                CharType::ZeroOne
                            } else {
                                CharType::Var
                            }
                        },
                        _ => {}
                    };
                    stack.push(*c as char);
                },
                b'|' => self.add_sub_tree(&mut stack, Symbols::Or),
                b'&' => self.add_sub_tree(&mut stack, Symbols::And),
                b'!' => {
                    /*
                        if the previous character was an operator or if we are at the end of the
                        formula, we should add a new not node to the tree, otherwise we should
                        add the negation operator to the stack to use it again in the next iteration.
                     */
                    if is_oper || (formula.len() - processed == 1) {
                        self.add_not_node(&mut stack, formula.len() - processed == 1);
                    } else {
                        let top: char = stack.pop().unwrap_or_else(|| {
                            panic!("Invalid formula");
                        });

                        /*
                            if the top of the stack is not a negation operator we should push it again
                            to the stack and add a negation operator to use it again in the next iteration.
                         */
                        if top != '!' {
                            stack.push(top);
                            stack.push(*c as char);
                            self.not_cnt += 1;
                        }
                    }
                }
                b'^' => self.add_sub_tree(&mut stack, Symbols::Xor),
                b'>' => self.add_sub_tree(&mut stack, Symbols::MatCond),
                b'=' => self.add_sub_tree(&mut stack, Symbols::LogEq),
                _ => panic!("Invalid formula"),
            }
            /*
                This boolean is used to check if the previous character was an operator,
                we will use it to help us when we have a negation operator to decide if we
                should add a new node to the tree or just add the negation operator to the stack
                to use it again in the next iteration.
                for example:
                1. if we have a formula like "AB&!", we should add a new not node to the tree
                that will be a negation of the subtree with the parent node "&" and the children
                "A" and "B".
                2. if we have a formula like "A!!!!", when we find a negation operator, we just
                check if should add it to stack if the top of the stack is not a negation operator.
             */
            if let CharType::Var = var_type {
                is_oper = if (*c < b'A' || *c > b'Z') && *c != b'!' {
                    true
                } else {
                    false
                };
            } else {
                is_oper = if *c != b'1' && *c != b'0' && *c != b'!' {
                    true
                } else {
                    false
                };
            }
            processed += 1;
        }

        if !stack.is_empty() {
            panic!("Invalid formula");
        }
        /*
            Here if the stack that we used to store the subtrees contains more than one
            subtree, means that the formula is invalid, because we are at the end and the root
            should have one subtree only.
         */
        if self.stack.len() > 1 {
            panic!("Invalid formula");
        }
        let top: RcNode = self.stack.pop().unwrap_or_else(|| {
            panic!("Invalid formula");
        });

        // if all good we should link the root to the rest of the tree
        self.root = Rc::clone(&top);
    }

    fn get_top(&mut self, stack: &mut Vec<char>) -> RcNode {
        let mut top: char = stack.pop().unwrap_or_else(|| {
            panic!("Invalid formula");
        });
        let new_node: RcNode;

        /*
            If the top of the stack is a negation operator, we should create a new node
            with the negation operator and the right child will be the top of the stack.

            Otherwise we should create a new node with the top of the stack.
         */
        if top == '!' {
            self.not_cnt -= 1;
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
        let new_node: RcNode =
            Rc::new(RefCell::new(Some(Box::new(Node::new(Symbols::Not)))));

        if self.stack.is_empty() && pop {
            /*
                If the self.stack in which we store the subtrees is empty and we should pop the top
                element of the stack with the characters, we get the top element from the stack
                we add it as a right child to the not node and we push the subtree to the self.stack
                Note: this case happens when we have a formula with just '!' operator and a single
                character.
             */
            new_node.borrow_mut().as_mut().unwrap().right = self.get_top(stack);
            self.stack.push(new_node);
        } else {
            /*
                If the self.stack is not empty, we should add a not node as a root of the top
                subtree in the self.stack and push it as a new subtree.
                Note: because the not node has only one child we always add the subtree to the right
                of the not node.
             */
            let rhs: RcNode = self.stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });

            new_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&rhs);
            self.stack.push(new_node);
        }
    }

    fn add_sub_tree(&mut self, stack: &mut Vec<char>, symbol: Symbols) {
        if (stack.len() - self.not_cnt) > 1 {
            /*
                If the stack with the characters contains more than one character and we found
                a new operator, we should create a new node with that operator that will be the
                root of the subtree, and we should pop two elements from the stack that will be 
                the left and right children of the subtree.
             */
            let new_node: RcNode = 
                Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));
            self.insert_left = false;
            
            new_node.borrow_mut().as_mut().unwrap().right = self.get_top(stack);
            new_node.borrow_mut().as_mut().unwrap().left = self.get_top(stack);
            /*
                This is an edge case, if the stack is still not empty, means we have more than 
                two characters in the stack, we should set the insert_left to true to indicate
                to next operator that we should insert this character as a left child of the next
                subtree, and the right subtree will be the current subtree tha we are building in
                this case.
                Example: if we have the following formula "111|&", when we find the first operator
                which is '|', we will create a subtre with the root Symbols::Or and the left and right
                children will be Symbols::True, so at this point the stack still contains the '1'
                and when we find the next operator which is '&', we should insert the '1' as a 
                left child.
                But if the foumula is as follows "11|1&", when we find the first operator
                which is '|' the process will be the same, but when we find the next operator which is
                '&', we should insert the '1' comming after '|' as a right child.
             */
            if !stack.is_empty() {
                self.insert_left = true;
            }
            self.stack.push(new_node);
        } else  if (stack.len() - self.not_cnt) == 1 {
            /*
                If we found and operator and the stack with the characters contains only one character,
                we will create a subtree with the root as the operator and the left and right children
                will be respectively either : the top of the stack with characters, the top of the self.stack
                with subtrees, or the top of the self.stack with subtrees, the top of the stack with characters.
                Depending on the value of the insert_left boolean.
             */
            let top: RcNode = self.stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });
            let new_node: RcNode = 
                Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));

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
            /*
                If we found and operator and the stack with the characters is empty, we should create
                a subtree with the root as the operator and the left and right children will be the top
                two subtrees in the self.stack.
             */
            let rhs: RcNode = self.stack.pop().unwrap();
            let lhs: RcNode = self.stack.pop().unwrap_or_else(|| {
                panic!("Invalid formula");
            });
            let new_node: RcNode = 
                Rc::new(RefCell::new(Some(Box::new(Node::new(symbol)))));
            
            new_node.borrow_mut().as_mut().unwrap().right = Rc::clone(&rhs);
            new_node.borrow_mut().as_mut().unwrap().left = Rc::clone(&lhs);
            self.stack.push(new_node);
        } else {
            panic!("Invalid formula");
        }
    }

    /// Evaluate the AST calling the recursive function eval_tree
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
            Symbols::Char(c) => {
                match c {
                    '1' => return true,
                    '0' => return false,
                    _ => true,
                }
            }
        }
    }

}
