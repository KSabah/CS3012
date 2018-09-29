#[derive(Debug)]
pub struct Tree{
    pub root: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Node{
    pub value: u8,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

macro_rules! insert_at {
    ($node:expr, $value:expr) => {
        {
            (move || {
                if let Some(ref mut node) = $node.as_mut() {
                    node.insert($value);
                    return
                }

                $node = Some(Box::new(Node::new($value)));
            })()
        }
    };
}

impl Tree{
    fn new() -> Self {
        Tree {
            root: None,
        }
    }

    fn insert(&mut self, value: u8) {
        insert_at!(self.root, value);
    }
}

impl Node{
    fn new(value: u8) -> Self {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: u8) {
        if value <= self.value {
            insert_at!(self.left, value);
        } else {
            insert_at!(self.right, value);
        }
    }
}

fn find_path (root: Node, path: &mut Vec<u8>, value: u8) -> bool{
    let mut path = Vec::new();
    if root.value == 0 {return false;}

    path.push(root.value);

    if root.value == value {return true;}

    if (root.left.is_some() && find_path(*root.left.unwrap(), &mut path, value)) ||
    (root.right.is_some() && find_path(*root.right.unwrap(), &mut path, value)){
         return true;}

    path.pop();
    return false;
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(6);
    tree.insert(10);
    tree.insert(1);
    tree.insert(5);
    tree.insert(8);
    println!("{:#?}", tree);
}
