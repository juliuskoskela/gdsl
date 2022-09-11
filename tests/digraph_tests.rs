#[test]
fn ut_digraph_manual_bfs() {
    use gdsl::digraph::*;
    use std::collections::{HashSet, VecDeque};

    let g = vec![
        Node::new(0, ()),
        Node::new(1, ()),
        Node::new(2, ()),
        Node::new(3, ()),
        Node::new(4, ()),
        Node::new(5, ()),
    ];

    g[0].connect(&g[1], ());
    g[0].connect(&g[2], ());
    g[0].connect(&g[3], ());
    g[1].connect(&g[4], ());
    g[2].connect(&g[5], ());
    g[3].connect(&g[4], ());
    g[3].connect(&g[5], ());

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(g[0].clone());
    visited.insert(g[0].key().clone());

    while let Some(node) = queue.pop_front() {
        for Edge(_, v, _) in &node {
            if !visited.contains(v.key()) {
                if v == g[4] {
                    return;
                }
                visited.insert(v.key().clone());
                queue.push_back(v);
            }
        }
    }
    panic!();
}

#[test]
fn ut_digraph_bfs() {
    use gdsl::*;

    let g = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2]
        (4) => []
    ];

    let path = g[0].bfs().target(&4).search_path().unwrap().to_vec_nodes();

    assert!(path[0] == g[0]);
    assert!(path[1] == g[2]);
    assert!(path[2] == g[4]);
}

#[test]
fn ut_digraph() {
    use gdsl::digraph::*;

    let a = Node::new(0x1, "A");
    let b = Node::new(0x2, "B");
    let c = Node::new(0x4, "C");

    a.connect(&b, 0.42);
    a.connect(&c, 1.7);
    b.connect(&c, 0.09);
    c.connect(&b, 12.9);

    let Edge(u, v, e) = a.iter_out().next().unwrap();

    assert!(u == a);
    assert!(v == b);
    assert!(e == 0.42);
}

#[test]
fn ut_digraph_new() {
    use gdsl::digraph::*;

    let n1 = Node::<i32, char, ()>::new(1, 'A');

    assert!(*n1.key() == 1);
    assert!(*n1.value() == 'A');
}

#[test]
fn ut_digraph_connect() {
    use gdsl::digraph::*;

    let n1 = Node::new(1, ());
    let n2 = Node::new(2, ());

    n1.connect(&n2, 4.20);

    assert!(n1.is_connected(n2.key()));
}

#[test]
fn ut_digraph_try_connect() {
    use gdsl::digraph::*;

    let n1 = Node::new(1, ());
    let n2 = Node::new(2, ());

    match n1.try_connect(&n2, ()) {
        Ok(_) => assert!(n1.is_connected(n2.key())),
        Err(_) => panic!("n1 should be connected to n2"),
    }

    match n1.try_connect(&n2, ()) {
        Ok(_) => panic!("n1 should be connected to n2"),
        Err(_) => assert!(n1.is_connected(n2.key())),
    }
}

#[test]
fn ut_digraph_disconnect() {
    use gdsl::digraph::*;

    let n1 = Node::new(1, ());
    let n2 = Node::new(2, ());

    n1.connect(&n2, ());

    assert!(n1.is_connected(n2.key()));

    if n1.disconnect(n2.key()).is_err() {
        panic!("n1 should be connected to n2");
    }

    assert!(!n1.is_connected(n2.key()));
}

#[test]
fn ut_digraph_isolate() {
    use gdsl::digraph::*;

    let n1 = Node::new(1, ());
    let n2 = Node::new(2, ());
    let n3 = Node::new(3, ());
    let n4 = Node::new(4, ());

    n1.connect(&n2, ());
    n1.connect(&n3, ());
    n2.connect(&n1, ());
    n3.connect(&n1, ());
    n4.connect(&n3, ());
    n3.connect(&n2, ());

    n1.isolate();

    assert!(n3.is_connected(n2.key()));
    assert!(n4.is_connected(n3.key()));
    assert!(!n1.is_connected(n2.key()));
    assert!(!n1.is_connected(n3.key()));
    assert!(n1.is_orphan());
}

// TEST Dfs

#[test]
fn ut_digraph_dfs_find_1() {
    use gdsl::*;

    let g = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2, 0]
        (4) => []
    ];

    let target = g[0].dfs().target(&4).search().unwrap();

    let source = g[4].dfs().target(&0).transpose().search().unwrap();

    assert!(target == g[4]);
    assert!(source == g[0]);
}

#[test]
fn ut_digraph_dfs_cycle_1() {
    use gdsl::*;

    let g = digraph![
        (usize)
        (0) => [0, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2, 0]
        (4) => []
    ];

    let cycle = g[0].dfs().search_cycle().unwrap().to_vec_nodes();

    assert!(cycle[0] == g[0]);
    assert!(cycle[1] == g[0]);
}

#[test]
fn ut_digraph_dfs_cycle_2() {
    use gdsl::*;

    let g = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2, 0]
        (4) => []
    ];

    let cycle = g[0].dfs().search_cycle().unwrap().to_vec_nodes();

    assert!(cycle[0] == g[0]);
    assert!(cycle.last().unwrap() == &g[0]);
}

// TEST Bfs

#[test]
fn ut_digraph_bfs_find_1() {
    use gdsl::*;

    let g = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2, 0]
        (4) => []
    ];

    let target = g[0].bfs().target(&4).search().unwrap();

    let source = g[4].bfs().target(&0).transpose().search().unwrap();

    assert!(target == g[4]);
    assert!(source == g[0]);
}

#[test]
fn ut_digraph_bfs_cycle_1() {
    use gdsl::*;

    let g = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2, 0]
        (4) => []
    ];

    let cycle = g[0].bfs().search_cycle().unwrap().to_vec_nodes();

    assert!(cycle[0] == g[0]);
    assert!(cycle[1] == g[3]);
    assert!(cycle[2] == g[0]);
}

#[test]
fn ut_digraph_bfs_cycle_2() {
    use gdsl::*;

    let g = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2, 0]
        (4) => []
    ];

    let cycle = g[0].bfs().search_cycle().unwrap().to_vec_nodes();

    assert!(cycle[0] == g[0]);
    assert!(cycle.last().unwrap() == &g[0]);
}

#[test]
fn ut_digraph_sizes() {
    use gdsl::digraph::*;

    type N1 = Node;
    type N2 = Node<usize>;
    type N3 = Node<usize, usize>;
    type N4 = Node<usize, usize, usize>;

    let n1 = N1::new(1, ());
    let n2 = N2::new(2, ());
    let n3 = N3::new(3, 42);
    let n4 = N4::new(4, 42);

    assert!(n1.sizeof() == 72);
    assert!(n2.sizeof() == 72);
    assert!(n3.sizeof() == 80);
    assert!(n4.sizeof() == 80);
}

#[test]
fn ut_digraph_deref_node() {
    use gdsl::digraph::*;

    let n1 = Node::<char, i32, f64>::new('A', 42);
    let n2 = Node::<char, i32, f64>::new('B', 6);

    n1.connect(&n2, 0.5);

    assert!(*n1 == 42);
    assert!(n2.key() == &'B');

    let Edge(u, v, e) = n1.iter_out().next().unwrap();

    assert!(u.key() == &'A');
    assert!(v == n2);
    assert!(e == 0.5);
}

#[test]
fn ut_serde_json() {
    use gdsl::digraph::*;
    use gdsl::*;

    let graph = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2]
        (4) => []
    ];

    let json = serde_json::to_string(&graph).unwrap();

    let de: Graph<usize, (), ()> = serde_json::from_str(&json).unwrap();

    let mut graph_vec = graph.to_vec();
    let mut de_vec = de.to_vec();

    graph_vec.sort_by(|a, b| a.key().cmp(b.key()));
    de_vec.sort_by(|a, b| a.key().cmp(b.key()));

    for (a, b) in graph_vec.iter().zip(de_vec.iter()) {
        assert!(a == b);
        for (Edge(u, v, e), Edge(u2, v2, e2)) in a.iter_out().zip(b.iter_out()) {
            assert!(u == u2);
            assert!(v == v2);
            assert!(e == e2);
        }
    }
}

#[test]
fn ut_serde_cbor() {
    use gdsl::digraph::*;
    use gdsl::*;

    let graph = digraph![
        (usize)
        (0) => [1, 2, 3]
        (1) => [3]
        (2) => [4]
        (3) => [2]
        (4) => []
    ];

    let cbor = serde_cbor::to_vec(&graph).unwrap();

    let de: Graph<usize, (), ()> = serde_cbor::from_slice(&cbor).unwrap();

    let mut graph_vec = graph.to_vec();
    let mut de_vec = de.to_vec();

    graph_vec.sort_by(|a, b| a.key().cmp(b.key()));
    de_vec.sort_by(|a, b| a.key().cmp(b.key()));

    for (a, b) in graph_vec.iter().zip(de_vec.iter()) {
        assert!(a == b);
        for (Edge(u, v, e), Edge(u2, v2, e2)) in a.iter_out().zip(b.iter_out()) {
            assert!(u == u2);
            assert!(v == v2);
            assert!(e == e2);
        }
    }
}

#[test]
fn ut_serde_cbor_big() {
    use gdsl::digraph::*;
    use gdsl::*;
    use std::cell::Cell;

    let graph = digraph![
        (char, Cell<u64>) => [u64]
        ('A', Cell::new(u64::MAX)) => [ ('B', 4), ('H', 8) ]
        ('B', Cell::new(u64::MAX)) => [ ('A', 4), ('H', 11), ('C', 8) ]
        ('C', Cell::new(u64::MAX)) => [ ('B', 8), ('C', 2), ('F', 4), ('D', 7) ]
        ('D', Cell::new(u64::MAX)) => [ ('C', 7), ('F', 14), ('E', 9) ]
        ('E', Cell::new(u64::MAX)) => [ ('D', 9), ('F', 10) ]
        ('F', Cell::new(u64::MAX)) => [ ('G', 2), ('C', 4), ('D', 14), ('E', 10) ]
        ('G', Cell::new(u64::MAX)) => [ ('H', 1), ('I', 6), ('F', 2) ]
        ('H', Cell::new(u64::MAX)) => [ ('A', 8), ('B', 11), ('I', 7), ('G', 1) ]
        ('I', Cell::new(u64::MAX)) => [ ('H', 7), ('C', 2), ('G', 6) ]
    ];

    let cbor = serde_cbor::to_vec(&graph).unwrap();

    let de: Graph<char, Cell<u64>, u64> = serde_cbor::from_slice(&cbor).unwrap();

    let mut graph_vec = graph.to_vec();
    let mut de_vec = de.to_vec();

    graph_vec.sort_by(|a, b| a.key().cmp(b.key()));
    de_vec.sort_by(|a, b| a.key().cmp(b.key()));

    for (a, b) in graph_vec.iter().zip(de_vec.iter()) {
        assert!(a == b);
        for (Edge(u, v, e), Edge(u2, v2, e2)) in a.iter_out().zip(b.iter_out()) {
            assert!(u == u2);
            assert!(v == v2);
            assert!(e == e2);
        }
    }
}

#[test]
fn ut_digraph_order() {
    use gdsl::digraph::*;

    let n1 = Node::new(1, ());
    let n2 = Node::new(2, ());
    let n3 = Node::new(3, ());

    n1.connect(&n2, ());
    n2.connect(&n3, ());
    n3.connect(&n1, ());

    let order = n1.preorder().search_nodes();

    assert!(order[0] == n1);
    assert!(order[1] == n2);
    assert!(order[2] == n3);

    let order = n1.postorder().search_nodes();

    assert!(order[0] == n3);
    assert!(order[1] == n2);
    assert!(order[2] == n1);
}

#[test]
fn doc_header_digraph() {
    use gdsl::digraph::*;

    let mut g: Graph<usize, (), ()> = Graph::new();

    g.insert(Node::new(0, ()));
    g.insert(Node::new(1, ()));
    g.insert(Node::new(2, ()));
    g.insert(Node::new(3, ()));
    g.insert(Node::new(4, ()));

    g[0].connect(&g[1], ());
    g[0].connect(&g[2], ());
    g[0].connect(&g[3], ());
    g[1].connect(&g[3], ());
    g[2].connect(&g[4], ());
    g[3].connect(&g[2], ());
    g[3].connect(&g[0], ()); // 3 points back to 0 creating a cycle

    let cycle = g[0] // We start at node 0
        .bfs() // We use a breadth-first search
        .search_cycle() // We search for a cycle
        .unwrap() // Returns `Option<Path<usize, (), ()>>`
        .to_vec_nodes(); // Path is converted to a vector of nodes

    assert!(cycle[0] == g[0]);
    assert!(cycle[1] == g[3]);
    assert!(cycle[2] == g[0]);
}

#[test]
fn ut_digraph_scc() {
    use gdsl::digraph::*;

    let mut g: Graph<usize, (), ()> = Graph::new();

    g.insert(Node::new(0, ()));
    g.insert(Node::new(1, ()));
    g.insert(Node::new(2, ()));
    g.insert(Node::new(3, ()));
    g.insert(Node::new(4, ()));
    g.insert(Node::new(5, ()));
    g.insert(Node::new(6, ()));
    g.insert(Node::new(7, ()));
    g.insert(Node::new(8, ()));
    g.insert(Node::new(9, ()));

    g[0].connect(&g[1], ()); // ---- C1
    g[1].connect(&g[2], ()); //
    g[2].connect(&g[0], ()); //
    g[3].connect(&g[4], ()); // ---- C2
    g[4].connect(&g[5], ()); //
    g[5].connect(&g[3], ()); //
    g[6].connect(&g[7], ()); // ---- C3
    g[7].connect(&g[8], ()); //
    g[8].connect(&g[6], ()); //
    g[9].connect(&g[9], ()); // ---- C4

    let mut scc = g.scc();

    // Since the graph container is a hash map, the order of the SCCs is not
    // deterministic. We sort the SCCs by their size to make the test
    // deterministic.
    scc.sort_by(|a, b| a.len().cmp(&b.len()));

    assert!(scc.len() == 4);
    assert!(scc[0].len() == 1);
    assert!(scc[1].len() == 3);
    assert!(scc[2].len() == 3);
    assert!(scc[3].len() == 3);
}
