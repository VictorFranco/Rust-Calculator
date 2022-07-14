pub struct Node<T>{
    pub left  : Option<Box<Node<T>>>,
    pub right : Option<Box<Node<T>>>,
    pub value : T
}

pub struct Tree<T>{
    pub root  : Option<Box<Node<T>>>
}

impl<T: std::fmt::Debug + std::cmp::PartialOrd> Node<T>{

    pub fn pre_order(&self){
        print!("{:?}", self.value);
        match &self.left {
            Some(p) => {
                print!("(");
                Node::pre_order(&p);
            },
            None    => {},
        }
        match &self.right {
            Some(p) => {
                Node::pre_order(&p);
                print!(")");
            },
            None    => {},
        }
    }

    pub fn in_order(&self){
        match &self.left {
            Some(p) => {
                print!("(");
                Node::pre_order(&p);
            },
            None    => {},
        }
        print!("{:?}", self.value);
        match &self.right {
            Some(p) => {
                Node::pre_order(&p);
                print!(")");
            },
            None    => {},
        }
    }

    pub fn post_order(&self){
        match &self.left {
            Some(p) => {
                print!("(");
                Node::pre_order(&p);
            },
            None    => {},
        }
        match &self.right {
            Some(p) => {
                Node::pre_order(&p);
                print!(")");
            },
            None    => {},
        }
        print!("{:?}", self.value);
    }

    pub fn create_node(val:T)-> Node<T>{
        Node{
            left  : None,
            right : None,
            value : val
        }
    }

    pub fn insert_node_by_value(&mut self, val:T){
        if val<=self.value {
            match &mut self.left {
                Some(l) => Node::insert_node_by_value(l,val),
                None    => {
                    let node    = Node::create_node(val);
                    let pointer = Some(Box::new(node));
                    self.left   = pointer;
                }
            }
        }else{
            match &mut self.right {
                Some(r) => Node::insert_node_by_value(r,val),
                None    => {
                    let node    = Node::create_node(val);
                    let pointer = Some(Box::new(node));
                    self.right  = pointer;
                }
            }
        }
    }

}
impl<T: std::fmt::Debug + std::cmp::PartialOrd> Tree<T>{

    pub fn create_tree()-> Tree<T>{
        Tree{
            root: None
        }
    }

    pub fn pre_order(&self){
        match &self.root {
            Some(p) => Node::pre_order(&p),
            None    => println!("The Tree is empty")
        }
    }

    pub fn in_order(&self){
        match &self.root {
            Some(p) => Node::in_order(&p),
            None    => println!("The Tree is empty")
        }
    }

    pub fn post_order(&self){
        match &self.root {
            Some(p) => Node::post_order(&p),
            None    => println!("The Tree is empty")
        }
    }

    pub fn insert_node_by_value(&mut self,val:T){
        match &mut self.root {
            Some(p) => Node::insert_node_by_value(p,val),
            None    => {
                let node    = Node::create_node(val);
                let pointer = Some(Box::new(node));
                self.root   = pointer;
            }
        }
    }

}
