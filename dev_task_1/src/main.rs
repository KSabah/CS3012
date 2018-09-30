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

    fn insert(&mut self, node: Node) {
        insert_at!(self.root, node.value);
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
}

#[cfg(test)]
mod tests
{   // Importing functions names from outer (for mod tests) scope
    use super::*;
    //Test some arbitrary values in an empty tree
    #[test]
    fn test_empty_tree()
    {
        let tree = Tree::new();
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);
        assert_eq!(find_lca(node1, node2, node3), -1);
    }
    //Test for the root node ancestor
    #[test]
    fn test_ancestor()
    {
        let mut tree = Tree::new();
        let node1 = Node::new(1);
        let node1c = node1.clone();
        let node2 = Node::new(2);
        let node2c = node2.clone();
        let node3 = Node::new(3);
        let node3c = node3.clone();
        tree.insert(node1);
        tree.insert(node2);
        tree.insert(node3);
        assert_eq!(find_lca(node2c, node1c, node3c), 1);
    }
    //Test to make sure the algorithm works
    #[test]
    fn test_regular()
    {
        let mut tree = Tree::new();
        let node1 = Node::new(1);
        let node1c = node1.clone();
        let node2 = Node::new(2);
        let node2c = node2.clone();
        let node3 = Node::new(3);
        let node3c = node3.clone();
        let node4 = Node::new(4);
        let node4c = node4.clone();
        let node5 = Node::new(5);
        let node5c = node5.clone();
        let node6 = Node::new(6);
        let node6c = node6.clone();
        tree.insert(node3);
        tree.insert(node1);
        tree.insert(node2);
        tree.insert(node4);
        tree.insert(node5);
        tree.insert(node6);
        //testing the right side
        assert_eq!(find_lca(node4c, node5c, node6c), 4);
        //testing left side
        assert_eq!(find_lca(node3c, node2c, node1c), 3);
    }
    //Test what happens when a node that is not in the tree is entered
    #[test]
    #[should_panic]
    fn test_rogue_node()
    {
        let mut tree = Tree::new();
        let node1 = Node::new(1);
        let node1c = node1.clone();
        let node2 = Node::new(2);
        let node2c = node2.clone();
        let node3 = Node::new(3);
        let node4 = Node::new(4);
        tree.insert(node1);
        tree.insert(node2);
        tree.insert(node3);
        assert_eq!(find_lca(node1c, node4, node2c), -1);
    }
}
