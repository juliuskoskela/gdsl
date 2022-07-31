
#[test]
fn doctest_dinode_connect()
{
	use gdsl::digraph::*;

	type Node = DiNode<usize, Empty, Empty>;

	let n1 = Node::new(1, Empty);
	let n2 = Node::new(2, Empty);

	n1.connect(&n2, Empty);

	assert!(n1.is_connected(n2.key()));
}

#[test]
fn doctest_dinode_try_connect()
{
	use gdsl::digraph::*;

	type Node = DiNode<usize, Empty, Empty>;

	let n1 = Node::new(1, Empty);
	let n2 = Node::new(2, Empty);

	match n1.try_connect(&n2, Empty) {
		Ok(_) => assert!(n1.is_connected(n2.key())),
		Err(_) => panic!("n1 should be connected to n2"),
	}

	match n1.try_connect(&n2, Empty) {
		Ok(_) => panic!("n1 should be connected to n2"),
		Err(_) => assert!(n1.is_connected(n2.key())),
	}
}

#[test]
fn doctest_dinode_disconnect()
{
	use gdsl::digraph::*;

	type Node = DiNode<usize, Empty, Empty>;

	let n1 = Node::new(1, Empty);
	let n2 = Node::new(2, Empty);

	n1.connect(&n2, Empty);

	assert!(n1.is_connected(n2.key()));

	if n1.disconnect(n2.key()).is_err() {
		panic!("n1 should be connected to n2");
	}

	assert!(!n1.is_connected(n2.key()));
}