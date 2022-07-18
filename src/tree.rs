use crate::stack::Stack as Stack;

pub struct Node<T> {
    pub left  : Option<Box<Node<T>>>,
    pub right : Option<Box<Node<T>>>,
    pub value : T
}

pub struct Tree<T> {
    pub root  : Option<Box<Node<T>>>
}

impl<T: std::fmt::Debug + std::cmp::PartialOrd + std::marker::Copy> Node<T> {

    pub fn pre_order<const N: usize>(&self, stack: &mut Stack<T, N>) {
        (*stack).push(self.value).unwrap();
        match &self.left {
            Some(p) => {
                Node::pre_order(&p, stack);
            },
            None    => {},
        }
        match &self.right {
            Some(p) => {
                Node::pre_order(&p, stack);
            },
            None    => {},
        }
    }

    pub fn in_order<const N: usize>(&self, stack: &mut Stack<T, N>) {
        match &self.left {
            Some(p) => {
                Node::in_order(&p, stack);
            },
            None    => {},
        }
        (*stack).push(self.value).unwrap();
        match &self.right {
            Some(p) => {
                Node::in_order(&p, stack);
            },
            None    => {},
        }
    }

    pub fn post_order<const N: usize>(&self, stack: &mut Stack<T, N>) {
        match &self.left {
            Some(p) => {
                Node::post_order(&p, stack);
            },
            None    => {},
        }
        match &self.right {
            Some(p) => {
                Node::post_order(&p, stack);
            },
            None    => {},
        }
        (*stack).push(self.value).unwrap();
    }

    pub fn create_node(val: T) -> Node<T> {
        Node {
            left  : None,
            right : None,
            value : val
        }
    }

    pub fn insert_node_by_value(&mut self, val: T) {
        if val <= self.value {
            match &mut self.left {
                Some(l) => Node::insert_node_by_value(l, val),
                None    => {
                    let node    = Node::create_node(val);
                    let pointer = Some(Box::new(node));
                    self.left   = pointer;
                }
            }
        } else {
            match &mut self.right {
                Some(r) => Node::insert_node_by_value(r, val),
                None    => {
                    let node    = Node::create_node(val);
                    let pointer = Some(Box::new(node));
                    self.right  = pointer;
                }
            }
        }
    }

}
impl<T: std::fmt::Debug + std::cmp::PartialOrd + std::marker::Copy> Tree<T> {

    pub fn create_tree() -> Tree<T> {
        Tree{
            root: None
        }
    }

    #[allow(dead_code)]
    pub fn pre_order<const N: usize>(&self, stack: &mut Stack<T, N>) {
        match &self.root {
            Some(p) => Node::pre_order(&p, stack),
            None    => println!("The Tree is empty")
        }
    }

    #[allow(dead_code)]
    pub fn in_order<const N: usize>(&self, stack: &mut Stack<T, N>) {
        match &self.root {
            Some(p) => Node::in_order(&p, stack),
            None    => println!("The Tree is empty")
        }
    }

    pub fn post_order<const N: usize>(&self, stack: &mut Stack<T, N>) {
        match &self.root {
            Some(p) => Node::post_order(&p, stack),
            None    => println!("The Tree is empty")
        }
    }

    #[allow(dead_code)]
    pub fn insert_node_by_value(&mut self, val: T) {
        match &mut self.root {
            Some(p) => Node::insert_node_by_value(p, val),
            None    => {
                let node    = Node::create_node(val);
                let pointer = Some(Box::new(node));
                self.root   = pointer;
            }
        }
    }

    pub fn insert_first(&mut self, root: Option<Box<Node<T>>>) {
        self.root = root;
    }

}
