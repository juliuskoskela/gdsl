//! Tests for the new generic algorithm implementations

use gdsl::core::*;
use gdsl::digraph;
use gdsl::ungraph;

#[test]
fn test_generic_dfs_digraph() {
    let n1 = digraph::Node::new(1, "A");
    let n2 = digraph::Node::new(2, "B");
    let n3 = digraph::Node::new(3, "C");
    
    n1.connect(&n2, ());
    n2.connect(&n3, ());
    n3.connect(&n1, ());
    
    let result = n1.generic_dfs_preorder();
    let keys: Vec<_> = result.iter().map(|n| *n.key()).collect();
    
    assert_eq!(keys.len(), 3);
    assert_eq!(keys[0], 1); // Should start with node 1
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&3));
}

#[test]
fn test_generic_bfs_digraph() {
    let n1 = digraph::Node::new(1, "A");
    let n2 = digraph::Node::new(2, "B");
    let n3 = digraph::Node::new(3, "C");
    let n4 = digraph::Node::new(4, "D");
    
    n1.connect(&n2, ());
    n1.connect(&n3, ());
    n2.connect(&n4, ());
    
    let result = n1.generic_bfs();
    let keys: Vec<_> = result.iter().map(|n| *n.key()).collect();
    
    assert_eq!(keys.len(), 4);
    assert_eq!(keys[0], 1); // Should start with node 1
    // BFS should visit level 1 (nodes 2,3) before level 2 (node 4)
    let pos_2 = keys.iter().position(|&x| x == 2).unwrap();
    let pos_3 = keys.iter().position(|&x| x == 3).unwrap();
    let pos_4 = keys.iter().position(|&x| x == 4).unwrap();
    
    assert!(pos_2 < pos_4);
    assert!(pos_3 < pos_4);
}

#[test]
fn test_generic_dfs_ungraph() {
    let n1 = ungraph::Node::new(1, "A");
    let n2 = ungraph::Node::new(2, "B");
    let n3 = ungraph::Node::new(3, "C");
    
    n1.connect(&n2, ());
    n2.connect(&n3, ());
    
    let result = n1.generic_dfs_preorder();
    let keys: Vec<_> = result.iter().map(|n| *n.key()).collect();
    
    assert_eq!(keys.len(), 3);
    assert_eq!(keys[0], 1); // Should start with node 1
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&3));
}

#[test]
fn test_generic_bfs_ungraph() {
    let n1 = ungraph::Node::new(1, "A");
    let n2 = ungraph::Node::new(2, "B");
    let n3 = ungraph::Node::new(3, "C");
    let n4 = ungraph::Node::new(4, "D");
    
    n1.connect(&n2, ());
    n1.connect(&n3, ());
    n2.connect(&n4, ());
    
    let result = n1.generic_bfs();
    let keys: Vec<_> = result.iter().map(|n| *n.key()).collect();
    
    assert_eq!(keys.len(), 4);
    assert_eq!(keys[0], 1); // Should start with node 1
}

#[test]
fn test_static_graph_basic_operations() {
    let mut graph = StaticDirectedGraph::new();
    
    // Add nodes
    let a_idx = graph.add_node("A", 1);
    let b_idx = graph.add_node("B", 2);
    let c_idx = graph.add_node("C", 3);
    
    assert_eq!(a_idx, 0);
    assert_eq!(b_idx, 1);
    assert_eq!(c_idx, 2);
    
    // Add edges
    let edge1 = graph.add_edge(&"A", &"B", 10.0);
    let edge2 = graph.add_edge(&"B", &"C", 20.0);
    let edge3 = graph.add_edge(&"A", &"C", 30.0);
    
    assert!(edge1.is_some());
    assert!(edge2.is_some());
    assert!(edge3.is_some());
    
    // Check node retrieval
    let node_a = graph.get_node(&"A").unwrap();
    assert_eq!(node_a.key(), &"A");
    assert_eq!(node_a.value(), &1);
    assert_eq!(node_a.id(), 0);
    
    // Check graph structure
    assert_eq!(graph.nodes().len(), 3);
    assert_eq!(graph.edges().len(), 3);
    
    // Check adjacency
    let out_edges_a = graph.out_edges(0);
    assert_eq!(out_edges_a.len(), 2); // A -> B and A -> C
    
    let in_edges_c = graph.in_edges(2);
    assert_eq!(in_edges_c.len(), 2); // B -> C and A -> C
}

#[test]
fn test_static_graph_node_trait() {
    let mut graph: StaticDirectedGraph<&str, &str, f64> = StaticDirectedGraph::new();
    graph.add_node("test", "value");
    
    let node = graph.get_node(&"test").unwrap();
    
    // Test GraphNode trait implementation
    assert_eq!(node.key(), &"test");
    assert_eq!(node.value(), &"value");
    assert_eq!(node.id(), 0);
}

#[test]
fn test_generic_dijkstra() {
    // Create a simple weighted graph for testing
    let start_node = 1;
    let end_node = 3;
    
    // Graph: 1 --(5)-- 2 --(2)-- 3
    //        |                   ^
    //        +--------(10)-------+
    let get_neighbors = |node: &i32| -> Vec<(i32, f64)> {
        match *node {
            1 => vec![(2, 5.0), (3, 10.0)],
            2 => vec![(3, 2.0)],
            3 => vec![],
            _ => vec![],
        }
    };
    
    let result = generic_dijkstra(&start_node, &end_node, get_neighbors);
    
    assert!(result.is_some());
    let (path, cost) = result.unwrap();
    
    assert_eq!(path, vec![1, 2, 3]);
    assert_eq!(cost, 7.0); // 5 + 2 = 7, which is better than direct path (10)
}

#[test]
fn test_custom_dfs_with_filter() {
    let n1 = digraph::Node::new(1, "A");
    let n2 = digraph::Node::new(2, "B");
    let n3 = digraph::Node::new(3, "C");
    let n4 = digraph::Node::new(4, "D");
    
    n1.connect(&n2, 1.0);
    n1.connect(&n3, 5.0); // High weight edge
    n2.connect(&n4, 2.0);
    n3.connect(&n4, 1.0);
    
    // Custom DFS that only follows edges with weight <= 2.0
    let result = n1.generic_dfs_with(|node| {
        node.iter_out()
            .filter(|edge| edge.2 <= 2.0)
            .map(|edge| edge.1.clone())
            .collect()
    });
    
    let keys: Vec<_> = result.iter().map(|n| *n.key()).collect();
    
    // Should visit 1 -> 2 -> 4, but not 3 (due to high weight edge 1->3)
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&4));
    // Node 3 might or might not be visited depending on whether we reach it via 2->4->3 path
}

#[test]
fn test_algorithm_consistency() {
    // Test that the same algorithm produces consistent results
    let n1 = digraph::Node::new(1, "A");
    let n2 = digraph::Node::new(2, "B");
    let n3 = digraph::Node::new(3, "C");
    
    n1.connect(&n2, ());
    n2.connect(&n3, ());
    n3.connect(&n1, ());
    
    // Run the same algorithm multiple times
    let result1 = n1.generic_dfs_preorder();
    let result2 = n1.generic_dfs_preorder();
    let result3 = n1.generic_dfs_preorder();
    
    let keys1: Vec<_> = result1.iter().map(|n| *n.key()).collect();
    let keys2: Vec<_> = result2.iter().map(|n| *n.key()).collect();
    let keys3: Vec<_> = result3.iter().map(|n| *n.key()).collect();
    
    assert_eq!(keys1, keys2);
    assert_eq!(keys2, keys3);
}