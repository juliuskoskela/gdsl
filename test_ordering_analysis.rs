use gdsl::digraph::*;
use gdsl::ungraph;

fn main() {
    println!("=== Testing Current Ordering Functionality ===");
    
    // Test digraph ordering
    println!("\n--- Digraph Ordering ---");
    let n1 = Node::new(1, "A");
    let n2 = Node::new(2, "B");
    let n3 = Node::new(3, "C");
    let n4 = Node::new(4, "D");
    
    n1.connect(&n2, "1->2");
    n1.connect(&n3, "1->3");
    n2.connect(&n4, "2->4");
    n3.connect(&n4, "3->4");
    
    println!("Graph structure:");
    println!("  1 -> 2 -> 4");
    println!("  1 -> 3 -> 4");
    
    // Test preorder
    let preorder_nodes = n1.preorder().search_nodes();
    println!("\nPreorder nodes: {:?}", preorder_nodes.iter().map(|n| n.key()).collect::<Vec<_>>());
    
    let preorder_edges = n1.preorder().search_edges();
    println!("Preorder edges: {:?}", preorder_edges.iter().map(|e| (e.0.key(), e.1.key(), e.2)).collect::<Vec<_>>());
    
    // Test postorder
    let postorder_nodes = n1.postorder().search_nodes();
    println!("\nPostorder nodes: {:?}", postorder_nodes.iter().map(|n| n.key()).collect::<Vec<_>>());
    
    let postorder_edges = n1.postorder().search_edges();
    println!("Postorder edges: {:?}", postorder_edges.iter().map(|e| (e.0.key(), e.1.key(), e.2)).collect::<Vec<_>>());
    
    // Test ungraph ordering
    println!("\n--- Ungraph Ordering ---");
    let u1 = ungraph::Node::new(1, "A");
    let u2 = ungraph::Node::new(2, "B");
    let u3 = ungraph::Node::new(3, "C");
    let u4 = ungraph::Node::new(4, "D");
    
    u1.connect(&u2, "1-2");
    u1.connect(&u3, "1-3");
    u2.connect(&u4, "2-4");
    u3.connect(&u4, "3-4");
    
    println!("Graph structure:");
    println!("  1 - 2 - 4");
    println!("  1 - 3 - 4");
    
    // Test preorder
    let u_preorder_nodes = u1.order().pre().search_nodes();
    println!("\nPreorder nodes: {:?}", u_preorder_nodes.iter().map(|n| n.key()).collect::<Vec<_>>());
    
    let u_preorder_edges = u1.order().pre().search_edges();
    println!("Preorder edges: {:?}", u_preorder_edges.iter().map(|e| (e.0.key(), e.1.key(), e.2)).collect::<Vec<_>>());
    
    // Test postorder
    let u_postorder_nodes = u1.order().post().search_nodes();
    println!("\nPostorder nodes: {:?}", u_postorder_nodes.iter().map(|n| n.key()).collect::<Vec<_>>());
    
    let u_postorder_edges = u1.order().post().search_edges();
    println!("Postorder edges: {:?}", u_postorder_edges.iter().map(|e| (e.0.key(), e.1.key(), e.2)).collect::<Vec<_>>());
}