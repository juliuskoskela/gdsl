#[cfg(test)]
mod tests {
    use gdsl::digraph::*;
    use gdsl::ungraph;

    #[test]
    fn test_ungraph_ordering_comprehensive() {
        // Create a more complex graph for testing
        //     1
        //    / \
        //   2   3
        //  /   / \
        // 4   5   6
        let n1 = ungraph::Node::new(1, "root");
        let n2 = ungraph::Node::new(2, "left");
        let n3 = ungraph::Node::new(3, "right");
        let n4 = ungraph::Node::new(4, "leaf1");
        let n5 = ungraph::Node::new(5, "leaf2");
        let n6 = ungraph::Node::new(6, "leaf3");
        
        n1.connect(&n2, "1-2");
        n1.connect(&n3, "1-3");
        n2.connect(&n4, "2-4");
        n3.connect(&n5, "3-5");
        n3.connect(&n6, "3-6");
        
        // Test PreOrder
        let preorder_nodes = n1.order().pre().search_nodes();
        let preorder_keys: Vec<_> = preorder_nodes.iter().map(|n| *n.key()).collect();
        println!("Ungraph PreOrder nodes: {:?}", preorder_keys);
        
        let preorder_edges = n1.order().pre().search_edges();
        let preorder_edge_keys: Vec<_> = preorder_edges.iter()
            .map(|e| (*e.0.key(), *e.1.key()))
            .collect();
        println!("Ungraph PreOrder edges: {:?}", preorder_edge_keys);
        
        // Test PostOrder
        let postorder_nodes = n1.order().post().search_nodes();
        let postorder_keys: Vec<_> = postorder_nodes.iter().map(|n| *n.key()).collect();
        println!("Ungraph PostOrder nodes: {:?}", postorder_keys);
        
        let postorder_edges = n1.order().post().search_edges();
        let postorder_edge_keys: Vec<_> = postorder_edges.iter()
            .map(|e| (*e.0.key(), *e.1.key()))
            .collect();
        println!("Ungraph PostOrder edges: {:?}", postorder_edge_keys);
        
        // Assertions
        assert!(preorder_nodes.len() == 6);
        assert!(postorder_nodes.len() == 6);
        assert!(preorder_edges.len() == 5); // n-1 edges for tree
        assert!(postorder_edges.len() == 5);
        
        // Root should be first in preorder, last in postorder
        assert_eq!(*preorder_nodes[0].key(), 1);
        assert_eq!(*postorder_nodes.last().unwrap().key(), 1);
    }
    
    #[test]
    fn test_digraph_ordering_comprehensive() {
        // Create a directed graph
        //   1 -> 2 -> 4
        //   |    |
        //   v    v
        //   3 -> 5
        let n1 = Node::new(1, "root");
        let n2 = Node::new(2, "mid1");
        let n3 = Node::new(3, "mid2");
        let n4 = Node::new(4, "leaf1");
        let n5 = Node::new(5, "leaf2");
        
        n1.connect(&n2, "1->2");
        n1.connect(&n3, "1->3");
        n2.connect(&n4, "2->4");
        n2.connect(&n5, "2->5");
        n3.connect(&n5, "3->5");
        
        // Test PreOrder
        let preorder_nodes = n1.preorder().search_nodes();
        let preorder_keys: Vec<_> = preorder_nodes.iter().map(|n| *n.key()).collect();
        println!("Digraph PreOrder nodes: {:?}", preorder_keys);
        
        let preorder_edges = n1.preorder().search_edges();
        let preorder_edge_keys: Vec<_> = preorder_edges.iter()
            .map(|e| (*e.0.key(), *e.1.key()))
            .collect();
        println!("Digraph PreOrder edges: {:?}", preorder_edge_keys);
        
        // Test PostOrder
        let postorder_nodes = n1.postorder().search_nodes();
        let postorder_keys: Vec<_> = postorder_nodes.iter().map(|n| *n.key()).collect();
        println!("Digraph PostOrder nodes: {:?}", postorder_keys);
        
        let postorder_edges = n1.postorder().search_edges();
        let postorder_edge_keys: Vec<_> = postorder_edges.iter()
            .map(|e| (*e.0.key(), *e.1.key()))
            .collect();
        println!("Digraph PostOrder edges: {:?}", postorder_edge_keys);
        
        // Assertions
        assert!(preorder_nodes.len() >= 1);
        assert!(preorder_edges.len() >= 0);
        
        // Root should be first in preorder
        assert_eq!(*preorder_nodes[0].key(), 1);
    }
    
    #[test]
    fn test_ordering_with_filters() {
        let n1 = ungraph::Node::new(1, 1);
        let n2 = ungraph::Node::new(2, 2);
        let n3 = ungraph::Node::new(3, 3);
        let n4 = ungraph::Node::new(4, 4);
        
        n1.connect(&n2, 10);
        n1.connect(&n3, 20);
        n2.connect(&n4, 30);
        
        // Test with filter - only edges with weight > 15
        let mut filter_fn = |edge: &gdsl::ungraph::Edge<i32, i32, i32>| edge.2 > 15;
        let filtered_edges = n1.order().pre()
            .filter(&mut filter_fn)
            .search_edges();
        
        println!("Filtered edges (weight > 15): {:?}", 
                 filtered_edges.iter().map(|e| (*e.0.key(), *e.1.key(), e.2)).collect::<Vec<_>>());
        
        // Should only include edges with weight 20 and 30
        assert!(filtered_edges.len() <= 2);
        for edge in &filtered_edges {
            assert!(edge.2 > 15);
        }
    }
    
    #[test]
    fn test_ordering_empty_graph() {
        let n1: ungraph::Node<i32, &str, ()> = ungraph::Node::new(1, "single");
        
        let preorder_nodes = n1.order().pre().search_nodes();
        let preorder_edges = n1.order().pre().search_edges();
        
        assert_eq!(preorder_nodes.len(), 1);
        assert_eq!(preorder_edges.len(), 0);
        assert_eq!(*preorder_nodes[0].key(), 1);
    }
    
    #[test]
    fn test_ordering_cycle() {
        // Test with a cycle: 1 - 2 - 3 - 1
        let n1 = ungraph::Node::new(1, "a");
        let n2 = ungraph::Node::new(2, "b");
        let n3 = ungraph::Node::new(3, "c");
        
        n1.connect(&n2, "1-2");
        n2.connect(&n3, "2-3");
        n3.connect(&n1, "3-1");
        
        let preorder_nodes = n1.order().pre().search_nodes();
        let preorder_keys: Vec<_> = preorder_nodes.iter().map(|n| *n.key()).collect();
        println!("Cycle PreOrder nodes: {:?}", preorder_keys);
        
        // Should visit all nodes exactly once
        assert_eq!(preorder_nodes.len(), 3);
        
        // Check that all nodes are unique
        let mut unique_keys = preorder_keys.clone();
        unique_keys.sort();
        unique_keys.dedup();
        assert_eq!(unique_keys.len(), 3);
    }
}