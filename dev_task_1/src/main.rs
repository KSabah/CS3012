extern crate pathfinding;
extern crate petgraph;

use self::pathfinding::prelude::astar;
use self::petgraph::graph::NodeIndex;
use self::petgraph::Graph;
use std::collections::LinkedList;

/// Returns list of neighbors of a node with the corresponding cost.
fn neighbors<N, E>(graph: &Graph<N, E>, n: NodeIndex) -> LinkedList<(NodeIndex, u32)> {
    let mut list: LinkedList<(NodeIndex, u32)> = LinkedList::new();
    let mut neighbors = graph.neighbors(n).collect::<LinkedList<NodeIndex>>();
    for element in neighbors.iter_mut() {
        list.push_back((*element, 1));
    }
    return list;
}

/// A lowest common ancestor function for binary trees.
///
/// This function calculates the lowest common ancestor of two nodes in a graph that is structured as a binary tree.
///
/// * `graph` - Graph that the lowest common ancestor is applied on.
/// * `root`  - The root node of the binary tree.
/// * `node1` - The first node to calculate lca.
/// * `node2` - The second node to calculate lca.
pub fn lca<N, E>(
    graph: &Graph<N, E>,
    root: NodeIndex,
    node1: NodeIndex,
    node2: NodeIndex,
) -> Option<NodeIndex> {
    let path1 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == node1);
    let path2 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == node2);
    if path1.is_some() && path2.is_some() {
        let path1arr = path1.unwrap().0;
        let path2arr = path2.unwrap().0;

        let len;
        if path1arr.len() < path2arr.len() {
            len = path1arr.len();
        } else {
            len = path2arr.len();
        }

        let mut lca = root;
        for i in 0..len {
            if path1arr[i] == path2arr[i] {
                lca = path1arr[i]
            } else {
                break;
            }
        }
        return Some(lca);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::lca;
    use super::Graph;

    /// Tests the normal case for lca
    #[test]
    fn testlca() {
        let mut map = Graph::<&str, i32>::new();
        let root = map.add_node("root");
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        map.extend_with_edges(&[
            (root, n1),
            (root, n2),
            (n1, n3),
            (n1, n4),
            (n2, n5),
            (n2, n6),
        ]);
        assert_eq!(true, lca(&map, root, n1, n5).is_some());
        assert_eq!(root, lca(&map, root, n1, n5).unwrap());

        assert_eq!(true, lca(&map, root, n6, n5).is_some());
        assert_eq!(n2, lca(&map, root, n6, n5).unwrap());

        assert_eq!(true, lca(&map, root, n3, n4).is_some());
        assert_eq!(n1, lca(&map, root, n3, n4).unwrap());
    }

    /// Test for when nodes are not connected, should return None
    #[test]
    fn testlca_notconnected() {
        let mut map = Graph::<&str, i32>::new();
        let root = map.add_node("root");
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");

        assert_eq!(false, lca(&map, root, n1, n2).is_some());
        assert_eq!(false, lca(&map, root, n3, n4).is_some());
        assert_eq!(false, lca(&map, root, n5, n6).is_some());
    }

    /// Tests that the same node is returned when the two nodes entered are the same
    #[test]
    fn testlca_samenode() {
        let mut map = Graph::<&str, i32>::new();
        let root = map.add_node("root");

        assert_eq!(true, lca(&map, root, root, root).is_some());
        assert_eq!(root, lca(&map, root, root, root).unwrap());
    }

    /// Tests for a cycle in the graph, should return None
   #[test]
   fn testlca_structure() {
       let mut map = Graph::<&str, i32>::new();
       let root = map.add_node("root");
       let n1 = map.add_node("1");
       let n2 = map.add_node("2");
       let n3 = map.add_node("3");
       let n4 = map.add_node("4");
       let n5 = map.add_node("5");
       let n6 = map.add_node("6");
       map.extend_with_edges(&[
           (n4,root),
           (n5,root),
           (root, n1),
           (root, n2),
           (n1, n3),
           (n1, n4),
           (n2, n5),
           (n2, n6),
       ]);
       assert_eq!(false, lca(&map, root, n1, n5).is_some());
   }
}
