
#[test]
fn doctest_dinode_manual_bfs()
{
	use gdsl::digraph::*;
	use std::collections::{HashSet, VecDeque};

	type Node = DiNode<usize, (), ()>;

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
		for (_, v, _) in &node {
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
fn test_digraph_bfs() {
	use gdsl::*;

	let g = graph![
		(usize) =>
		(0) => [1, 2, 3]
		(1) => [3]
		(2) => [4]
		(3) => [2]
		(4) => []
	];

	let path = g[0]
		.bfs()
		.target(&4)
		.path_nodes()
		.unwrap();

	assert!(path[0] == g[0]);
	assert!(path[1] == g[2]);
	assert!(path[2] == g[4]);
}

#[test]
fn doctest_dinode()
{
	use gdsl::digraph::*;

	type Node<'a> = DiNode<usize, &'a str, f64>;

	let a = Node::new(0x1, "A");
	let b = Node::new(0x2, "B");
	let c = Node::new(0x4, "C");

	a.connect(&b, 0.42);
	a.connect(&c, 1.7);
	b.connect(&c, 0.09);
	c.connect(&b, 12.9);

	let (u, v, e) = a.iter_out().next().unwrap();

	assert!(u == a);
	assert!(v == b);
	assert!(e == 0.42);
}

#[test]
fn doctest_dinode_new()
{
	use gdsl::digraph::*;

	type Node = DiNode<usize, char, ()>;

	let n1 = Node::new(1, 'A');

	assert!(*n1.key() == 1);
	assert!(*n1.value() == 'A');
}

#[test]
fn doctest_dinode_connect()
{
	use gdsl::digraph::*;

	type Node = DiNode<usize, (), f64>;

	let n1 = Node::new(1, ());
	let n2 = Node::new(2, ());

	n1.connect(&n2, 4.20);

	assert!(n1.is_connected(n2.key()));
}

#[test]
fn doctest_dinode_try_connect()
{
	use gdsl::digraph::*;

	type Node = DiNode<usize, (), ()>;

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
fn doctest_dinode_disconnect()
{
	use gdsl::digraph::*;

	type Node = DiNode<usize, (), ()>;

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
fn doctest_dinode_isolate() {
	use gdsl::digraph::*;

	type Node = DiNode<usize, (), ()>;

	let n1 = Node::new(1, ());
	let n2 = Node::new(2, ());
	let n3 = Node::new(3, ());
	let n4 = Node::new(4, ());

	n1.connect(&n2, ());
	n1.connect(&n3, ());
	n1.connect(&n4, ());
	n2.connect(&n1, ());
	n3.connect(&n1, ());
	n4.connect(&n1, ());

	assert!(n1.is_connected(n2.key()));
	assert!(n1.is_connected(n3.key()));
	assert!(n1.is_connected(n4.key()));

	n1.isolate();

	assert!(n1.is_orphan());
}