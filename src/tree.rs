pub struct Node<String>{
    pub left  : Option<Box<Node<String>>>,
    pub right : Option<Box<Node<String>>>,
    pub value : String
}

pub struct Tree<String>{
    pub root  : Option<Box<Node<String>>>
}

impl<String: std::fmt::Debug + std::cmp::PartialOrd> Node<String>{

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

    pub fn create_node(val:String)-> Node<String>{
        Node{
            left  : None,
            right : None,
            value : val
        }
    }

    pub fn insert_node(&mut self, val:String){
        if val<=self.value {
            match &mut self.left {
                Some(l) => Node::insert_node(l,val),
                None    => {
                    let node    = Node::create_node(val);
                    let pointer = Some(Box::new(node));
                    self.left   = pointer;
                }
            }
        }else{
            match &mut self.right {
                Some(r) => Node::insert_node(r,val),
                None    => {
                    let node    = Node::create_node(val);
                    let pointer = Some(Box::new(node));
                    self.right  = pointer;
                }
            }
        }
    }

}
impl<String: std::fmt::Debug + std::cmp::PartialOrd> Tree<String>{

    pub fn create_tree()-> Tree<String>{
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

    pub fn insert_node(&mut self,val:String){
        match &mut self.root {
            Some(p) => Node::insert_node(p,val),
            None    => {
                let node    = Node::create_node(val);
                let pointer = Some(Box::new(node));
                self.root   = pointer;
            }
        }
    }

}
