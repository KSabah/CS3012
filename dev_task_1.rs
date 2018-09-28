/*  Written by Sabah Khan, khans3@tcd.ie, 16320187
    Last modified at: , 28/09/2018
*/
mod LCA_algorithm
{
    use std::fmt;

    #[derive(Debug)]
    struct Tree<T>
    where T: Ord{
        root: Option<Box<Node<T>>>,}

    #[derive(Debug)]
    struct Node<T>
    where T: Ord
    {
        value: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    }

    macro_rules! insert_at
    {
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

    macro_rules! fmt_node
    {
        ($fmt:expr, $node:expr) => {
            if let Some(ref node) = $node.as_ref() {
                try!(node.fmt($fmt));
            }
        };
    }

    impl<T>Tree<T>
    where T: Ord
    {
        fn new() -> Self {
            Tree {
                root: None,
            }
        }
        fn insert(&mut self, value: T) {
            insert_at!(self.root, value);
        }
    }

    impl<T>Node<T>
    where T: Ord
    {
        fn new(value: T) -> Self {
            Node
            {
                value: value,
                left: None,
                right: None,
            }
        }
        fn insert(&mut self, value: T)
        {
            if value <= self.value
                insert_at!(self.left, value);
            else
                insert_at!(self.right, value);
        }
    }
}

#[cfg(test)]
mod tests
{   // Importing functions names from outer (for mod tests) scope
    use super::*;
    //Test to make sure the algorithm works
    #[test]
    fn test_regular()
    {
        lca(6, 7);
        lca(10, 20);
        lca(3, 3);
    }
    //Test for the root node ancestor
    #[test]
    fn test_ancestor()
    {
        assert_eq!(lca(2, 3), 1);
    }
    //Test some arbitrary values in an empty tree
    #[test]
    fn test_empty_tree()
    {
        assert_eq!(lca(1, 2), -1);
    }
    //Test what happens when a node that is not in the tree is entered
    #[test]
    #[should_panic]
    fn test_rogue_node()
    {
        lca(1, 42);
    }
}
