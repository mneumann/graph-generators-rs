extern crate rand;

use rand::{Rng};

pub struct Graph {
    pub nodes: Vec<usize>,
    pub edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self) -> usize {
        let node_id = self.nodes.len();
        self.nodes.push(node_id);
        node_id
    }

    pub fn add_edge(&mut self, edge: (usize, usize)) {
        match edge {
            (src, dst) => {
                assert!(src < self.nodes.len());
                assert!(dst < self.nodes.len());
                self.edges.push(edge);
            }
        }
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}


/// Generate a random, scale-free graph according to the
/// Barabási–Albert preferential attachment model.
///
/// rng: Random number generator to use.
/// n: Total number of nodes.
/// m: Number of edges to existing nodes for each newly added node.
///
/// TODO: Allow generation of undirected graphs.
///
pub fn barabasi_albert_graph<R:Rng>(rng: &mut R, n: usize, m: usize) -> Graph {
    assert!(n > m);
    assert!(m >= 1);

    let mut g = Graph::new();

    let mut repeated_nodes = Vec::new();
    let mut targets = Vec::new();

    // create m initial nodes.
    for _ in 0..m {
        targets.push(g.add_node());
    }

    for _ in m..n {
        // Invariant.
        assert!(targets.len() == m);

        let node = g.add_node();

        // from new node, draw `m` connections to the `targets`.
        for &target in &targets[..] {
            g.add_edge((node, target));
            repeated_nodes.push(target);
            repeated_nodes.push(node);
        }

        // select `m` nodes randomly as new targets for next round.
        targets = rand::sample(rng, repeated_nodes.iter().cloned(), m);
    }

    return g;
}

fn _test_barabasi_albert(n: usize, m: usize) {
    let mut rng = rand::thread_rng();
    let g = barabasi_albert_graph(&mut rng, n, m);
    assert_eq!(n, g.node_count());
    assert_eq!((n-m)*m, g.edge_count());
}
#[test]
fn test_barabasi_albert() {
    _test_barabasi_albert(100, 2);
    _test_barabasi_albert(100, 3);
    _test_barabasi_albert(200, 5);
}
