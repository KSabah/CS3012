/*  Written by Sabah Khan, khans3@tcd.ie, 16320187
    Last modified at: 13:23, 28/09/2018
*/

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
