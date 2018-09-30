#[derive(Debug)]
pub struct Tree {
    pub root: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

macro_rules! insert_at {
    ($node:expr, $value:expr) => {{
        (move || {
            if let Some(ref mut node) = $node.as_mut() {
                node.insert($value);
                return;
            }

            $node = Some(Box::new(Node::new($value)));
        })()
    }};
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        insert_at!(self.root, value);
    }
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value <= self.value {
            insert_at!(self.left, value);
        } else {
            insert_at!(self.right, value);
        }
    }
}

fn find_path(root: Node, path: &mut Vec<i32>, value: i32) -> bool {
    if root.value == 0 {
        return false;
    }

    path.push(root.value);

    if root.value == value {
        return true;
    }

    if (root.left.is_some() && find_path(*root.left.unwrap(), path, value))
        || (root.right.is_some() && find_path(*root.right.unwrap(), path, value))
    {
        return true;
    }

    path.pop();

    return false;
}

fn find_lca(root: Node, n1: Node, n2: Node) -> i32 {
    let mut path1 = Vec::new();
    let mut path2 = Vec::new();

    let root1 = root.clone();
    let root2 = root.clone();
    if !find_path(root1, &mut path1, n1.value) || !find_path(root2, &mut path2, n2.value) {
        return -1;
    }

    let mut i = 0;
    while i < path1.len() && i < path2.len() {
        if path1[i] != path2[i] {
            break;
        }
        i += 1;
        return path1[i - 1];
    }
    return -1;
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
