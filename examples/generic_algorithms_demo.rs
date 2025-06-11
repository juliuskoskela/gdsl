//! # Generic Algorithms Demonstration
//!
//! This example demonstrates how the new generic algorithm implementations
//! eliminate code duplication and provide a unified interface for different
//! graph types.

use gdsl::core::*;
use gdsl::digraph;
use gdsl::ungraph;

fn main() {
    println!("=== Generic Graph Algorithms Demo ===\n");
    
    // Demonstrate code deduplication with generic algorithms
    demonstrate_code_deduplication();
    
    // Demonstrate static graph performance
    demonstrate_static_graph();
    
    // Show how traits solve circular dependencies
    demonstrate_trait_system();
}

fn demonstrate_code_deduplication() {
    println!("1. Code Deduplication with Generic Algorithms");
    println!("==============================================");
    
    // Create a directed graph
    let d1 = digraph::Node::new("A", "Node A");
    let d2 = digraph::Node::new("B", "Node B");
    let d3 = digraph::Node::new("C", "Node C");
    let d4 = digraph::Node::new("D", "Node D");
    
    d1.connect(&d2, 1.0);
    d2.connect(&d3, 2.0);
    d3.connect(&d4, 3.0);
    d4.connect(&d1, 4.0);
    d1.connect(&d3, 5.0);
    
    // Create an undirected graph with the same structure
    let u1 = ungraph::Node::new("A", "Node A");
    let u2 = ungraph::Node::new("B", "Node B");
    let u3 = ungraph::Node::new("C", "Node C");
    let u4 = ungraph::Node::new("D", "Node D");
    
    u1.connect(&u2, 1.0);
    u2.connect(&u3, 2.0);
    u3.connect(&u4, 3.0);
    u4.connect(&u1, 4.0);
    u1.connect(&u3, 5.0);
    
    // Use the SAME generic algorithm implementation for both graph types
    println!("Directed Graph DFS from A:");
    let digraph_dfs = d1.generic_dfs_preorder();
    for node in &digraph_dfs {
        println!("  -> {} ({})", node.key(), node.value());
    }
    
    println!("\nUndirected Graph DFS from A:");
    let ungraph_dfs = u1.generic_dfs_preorder();
    for node in &ungraph_dfs {
        println!("  -> {} ({})", node.key(), node.value());
    }
    
    println!("\nDirected Graph BFS from A:");
    let digraph_bfs = d1.generic_bfs();
    for node in &digraph_bfs {
        println!("  -> {} ({})", node.key(), node.value());
    }
    
    println!("\nUndirected Graph BFS from A:");
    let ungraph_bfs = u1.generic_bfs();
    for node in &ungraph_bfs {
        println!("  -> {} ({})", node.key(), node.value());
    }
    
    println!("\n✅ Same algorithm code works for both directed and undirected graphs!");
    println!("   This eliminates the need for separate DFS/BFS implementations.\n");
}

fn demonstrate_static_graph() {
    println!("2. Static Graph Performance (No Smart Pointers)");
    println!("===============================================");
    
    let mut static_graph = StaticDirectedGraph::new();
    
    // Add nodes
    let a_idx = static_graph.add_node("A", "Node A");
    let b_idx = static_graph.add_node("B", "Node B");
    let c_idx = static_graph.add_node("C", "Node C");
    let d_idx = static_graph.add_node("D", "Node D");
    
    // Add edges
    static_graph.add_edge(&"A", &"B", 1.0);
    static_graph.add_edge(&"B", &"C", 2.0);
    static_graph.add_edge(&"C", &"D", 3.0);
    static_graph.add_edge(&"D", &"A", 4.0);
    static_graph.add_edge(&"A", &"C", 5.0);
    
    // Note: optimize() method would be available in a full implementation
    
    println!("Static Graph Structure:");
    println!("  Nodes: {}", static_graph.nodes().len());
    println!("  Edges: {}", static_graph.edges().len());
    
    for node in static_graph.nodes() {
        println!("  Node {}: {} (index: {})", node.key(), node.value(), node.id());
        let out_edges = static_graph.out_edges(node.id());
        println!("    Outbound edges: {} edges", out_edges.len());
        let in_edges = static_graph.in_edges(node.id());
        println!("    Inbound edges: {} edges", in_edges.len());
    }
    
    println!("\n✅ Static graph uses indices instead of Rc/RefCell for better performance!");
    println!("   Memory layout is more cache-friendly and has less overhead.\n");
}

fn demonstrate_trait_system() {
    println!("3. Trait System Solving Circular Dependencies");
    println!("=============================================");
    
    // Create a static node to show trait implementation
    let static_graph: StaticDirectedGraph<&str, &str, f64> = StaticDirectedGraph::new();
    let mut graph = static_graph.clone();
    graph.add_node("test", "Test Node");
    
    if let Some(node) = graph.get_node(&"test") {
        println!("Static Node implementing GraphNode trait:");
        println!("  Key: {}", node.key());
        println!("  Value: {}", node.value());
        println!("  ID: {}", node.id());
    }
    
    // Show how traits allow generic programming
    fn print_node_info<N: GraphNode>(node: &N) 
    where 
        N::NodeId: std::fmt::Debug,
    {
        println!("  Generic function - Key: {}, ID: {:?}", node.key(), node.id());
    }
    
    if let Some(node) = graph.get_node(&"test") {
        print_node_info(node);
    }
    
    println!("\n✅ Traits solve circular dependency issues!");
    println!("   GraphNode trait allows generic algorithms without circular imports.");
    println!("   Associated types provide type safety without runtime overhead.\n");
}

fn demonstrate_algorithm_flexibility() {
    println!("4. Algorithm Flexibility with Custom Neighbor Functions");
    println!("=======================================================");
    
    let d1 = digraph::Node::new(1, "Start");
    let d2 = digraph::Node::new(2, "Middle");
    let d3 = digraph::Node::new(3, "End");
    
    d1.connect(&d2, 1.0);
    d2.connect(&d3, 2.0);
    d3.connect(&d1, 3.0);
    
    // Custom traversal: only follow edges with weight < 2.5
    let custom_dfs = d1.generic_dfs_with(|node| {
        node.iter_out()
            .filter(|edge| edge.2 < 2.5)
            .map(|edge| edge.1.clone())
            .collect()
    });
    
    println!("Custom DFS (edges with weight < 2.5):");
    for node in &custom_dfs {
        println!("  -> {}", node.key());
    }
    
    println!("\n✅ Generic algorithms support custom neighbor selection!");
    println!("   Same algorithm, different traversal logic.\n");
}