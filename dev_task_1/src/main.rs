extern crate pathfinding;
extern crate petgraph;

use self::pathfinding::prelude::{astar,topological_sort};
use self::petgraph::{graph::NodeIndex, Graph, Direction::Outgoing};

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

/// Returns list of neighbors of a node for use in topological sort function
fn neighbors_tsort<N, E>(graph: &Graph<N, E>, n: NodeIndex) -> LinkedList<(NodeIndex)> {
    graph.neighbors_directed(n,Outgoing).collect::<LinkedList<NodeIndex>>()
}

/// A lowest common ancestor function for graphs
///
/// This function calculates the lowest common ancestor of two nodes in a direct acyclic graph
///
/// * `graph` - Graph that the lowest common ancestor is applied on.
/// * `node1` - The first node to calculate lca.
/// * `node2` - The second node to calculate lca.
pub fn lca<N, E>(
    graph: &Graph<N, E>,
    node1: NodeIndex,
    node2: NodeIndex,
) -> Option<NodeIndex> {

    let nodes = graph.node_indices().collect::<Vec<NodeIndex>>();
    let sorted = topological_sort(&nodes, |n| neighbors_tsort(&graph, *n));

    if sorted.is_err() {
        return None;
    }

    let top_sort = sorted.unwrap();

    let mut lca = None;
    for n in top_sort {
        let path1 = astar(&n, |n| neighbors(&graph, *n), |_| 0, |n| *n == node1);
        let path2 = astar(&n, |n| neighbors(&graph, *n), |_| 0, |n| *n == node2);

        if path1.is_some() && path2.is_some() {
            lca = Some(n);
        }
    }
    return lca;
}

#[cfg(test)]
mod tests {
    use super::lca;
    use super::Graph;

    /// Tests the normal case for lca in a connected dag
    #[test]
    fn testlca() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        let n7 = map.add_node("7");
        let n8 = map.add_node("8");
        map.extend_with_edges(&[
            (n1, n2),
            (n2, n3),
            (n2, n4),
            (n3, n5),
            (n4, n6),
            (n5, n7),
            (n6, n7),
            (n7, n8),
        ]);
        assert_eq!(true, lca(&map, n1, n8).is_some());
        assert_eq!(n1, lca(&map, n1, n8).unwrap());

        assert_eq!(true, lca(&map, n7, n5).is_some());
        assert_eq!(n5, lca(&map, n7, n5).unwrap());

        assert_eq!(true, lca(&map, n3, n8).is_some());
        assert_eq!(n3, lca(&map, n3, n8).unwrap());
    }

    /// Test for when nodes are not connected, should return None
    #[test]
    fn testlca_notconnected() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");

        assert_eq!(false, lca(&map, n1, n2).is_some());
        assert_eq!(false, lca(&map, n3, n4).is_some());
        assert_eq!(false, lca(&map, n5, n6).is_some());
    }

    /// Tests that the same node is returned when the two nodes entered are the same
    #[test]
    fn testlca_samenode() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("root");

        assert_eq!(true, lca(&map, n1, n1).is_some());
        assert_eq!(n1, lca(&map, n1, n1).unwrap());
    }

    /// Tests for a cycle in the graph, should return None
   #[test]
   fn testlca_cycle() {
       let mut map = Graph::<&str, i32>::new();
       let n1 = map.add_node("1");
       let n2 = map.add_node("2");
       let n3 = map.add_node("3");
       let n4 = map.add_node("4");
       let n5 = map.add_node("5");
       let n6 = map.add_node("6");
       map.extend_with_edges(&[
           (n4,n1),
           (n5,n1),
           (n1, n3),
           (n1, n4),
           (n2, n5),
           (n2, n6),
       ]);
       assert_eq!(false, lca(&map, n1, n5).is_some());
   }
   ///Tests for separate connected graphs, should return None when nodes are not connected
   #[test]
  fn testlca_separategraph() {
      let mut map = Graph::<&str, i32>::new();
      let n1 = map.add_node("1");
      let n2 = map.add_node("2");
      let n3 = map.add_node("3");
      let n4 = map.add_node("4");
      let n5 = map.add_node("5");
      let n6 = map.add_node("6");
      let n7 = map.add_node("7");
      let n8 = map.add_node("8");

      map.extend_with_edges(&[
          (n1,n2),
          (n2,n3),
          (n2,n4),

          (n5,n6),
          (n5,n7),
          (n6,n8)
      ]);

      assert_eq!(true, lca(&map, n2, n4).is_some());
      assert_eq!(n2, lca(&map, n2, n4).unwrap());

      assert_eq!(true, lca(&map, n7, n8).is_some());
      assert_eq!(n5, lca(&map, n7, n8).unwrap());

      assert_eq!(false, lca(&map, n4, n6).is_some());
  }
}
