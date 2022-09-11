use super::*;
use serde::{
    de::{self, Visitor},
    ser::{Serialize, SerializeTuple, Serializer},
    Deserialize,
};

fn graph_serde_decompose<'de, K, N, E>(g: &Graph<K, N, E>) -> (Vec<(K, N)>, Vec<(K, K, E)>)
where
    K: Clone + Hash + PartialEq + Eq + Display + Serialize,
    N: Clone + Serialize,
    E: Clone + Serialize,
{
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for (_, n) in g.iter() {
        nodes.push((n.key().clone(), n.value().clone()));

        for Edge(u, v, e) in n {
            edges.push((u.key().clone(), v.key().clone(), e));
        }
    }
    (nodes, edges)
}

impl<K, N, E> Serialize for Graph<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display + Serialize,
    N: Clone + Serialize,
    E: Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        let (nodes, edges) = graph_serde_decompose(self);
        tuple.serialize_element(&nodes)?;
        tuple.serialize_element(&edges)?;
        tuple.end()
    }
}

impl<'de, K, N, E> Deserialize<'de> for Graph<K, N, E>
where
    K: Clone + Hash + PartialEq + Eq + Display + Deserialize<'de>,
    N: Clone + Deserialize<'de>,
    E: Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct GraphVisitor<K, N, E>
        where
            K: Clone + Hash + PartialEq + Eq + Display,
            N: Clone,
            E: Clone,
        {
            _phantom: std::marker::PhantomData<(K, N, E)>,
        }

        impl<'de, K, N, E> Visitor<'de> for GraphVisitor<K, N, E>
        where
            K: Clone + Hash + PartialEq + Eq + Display + Deserialize<'de>,
            N: Clone + Deserialize<'de>,
            E: Clone + Deserialize<'de>,
        {
            type Value = Graph<K, N, E>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("node and edge lists")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut nodes = Vec::new();
                let mut edges: Vec<(K, K, E)> = Vec::new();

                if let Some(node_seq) = seq.next_element()? {
                    nodes = node_seq;
                }

                if let Some(edge_seq) = seq.next_element()? {
                    edges = edge_seq;
                }

                let mut g = Graph::new();

                for (k, v) in nodes {
                    g.insert(Node::new(k, v));
                }

                for (u, v, e) in edges {
                    let un = g.get(&u).ok_or_else(|| {
                        de::Error::custom(&format!(
                            "Can't connect {} => {} because {} doesn't exist!",
                            u, v, u
                        ))
                    })?;
                    let vn = g.get(&v).ok_or_else(|| {
                        de::Error::custom(&format!(
                            "Can't connect {} => {} because {} doesn't exist!",
                            u, v, v
                        ))
                    })?;
                    Node::connect(&un, &vn, e);
                }

                Ok(g)
            }
        }

        deserializer.deserialize_seq(GraphVisitor {
            _phantom: std::marker::PhantomData,
        })
    }
}
