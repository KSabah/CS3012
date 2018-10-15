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
pub fn lca_btree<N, E>(
    graph: &Graph<N, E>,
    root: NodeIndex,
    node1: NodeIndex,
    node2: NodeIndex,
) -> Option<NodeIndex> {
    let path1 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == node1);
    let path2 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == node2);

    if node1 != node2 {
        let reverse1 = astar(&node1, |n| neighbors(&graph, *n), |_| 0, |n| *n == root);
        let reverse2 = astar(&node2, |n| neighbors(&graph, *n), |_| 0, |n| *n == root);

        if reverse1.is_some() || reverse2.is_some() {
            return None;
        }
    }

    if path1.is_some() && path2.is_some() {
        let path1arr = path1.unwrap().0;
        let path2arr = path2.unwrap().0;

        let len;
        if path1arr.len() < path2arr.len() {
            len = path1arr.len();
        } else {
            len = path2arr.len();
        }

        let mut lca_btree = root;
        for i in 0..len {
            if path1arr[i] == path2arr[i] {
                lca_btree = path1arr[i]
            } else {
                break;
            }
        }
        return Some(lca_btree);
    }
    return None;
}
