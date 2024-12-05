use smallvec::SmallVec;

// the graph used to model the goes-before relationships in the rules.
// the puzzle input only seems to have 2-digit vertex numbers, so we can
// use u128 bitsets for vertex subsets and a dense 100 element adjacency list.
struct Graph {
    nodes: u128,
    out_edges: [u128; 100],
    in_edges: [u128; 100],
}

impl Graph {
    fn parse(input: &str) -> Self {
        let mut graph = Self { nodes: 0, out_edges: [0; 100], in_edges: [0; 100] };

        for line in input.lines() {
            let (a, b) = line.split_once('|').unwrap();
            let (a, b) = (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap());
            assert!(a < 100 && b < 100);

            graph.nodes |= (1u128 << a) | (1u128 << b);
            graph.out_edges[a as usize] |= 1u128 << b;
            graph.in_edges[b as usize] |= 1u128 << a;
        }

        graph
    }

    fn subgraph(&self, nodes: u128) -> Graph {
        assert!(nodes & self.nodes == nodes, "node set is not a subset of the graphs nodes");

        let mut out = Graph { nodes, out_edges: [0; 100], in_edges: [0; 100] };
        for node in bit_iter(nodes).map(|n| n as usize) {
            out.out_edges[node] = self.out_edges[node] & nodes;
            out.in_edges[node] = self.in_edges[node] & nodes;
        }

        out
    }

    fn has_edge(&self, from: u8, to: u8) -> bool {
        (self.out_edges[from as usize] & (1u128 << to)) != 0
    }
}

// an iterator that yields all the indices `i` such that
// the `i`-th bit in `n` is set, in ascending order.
fn bit_iter(mut n: u128) -> impl Iterator<Item = u32> {
    std::iter::from_fn(move || {
        if n == 0 {
            None
        } else {
            let idx = n.trailing_zeros();
            // clear the lowest set bit of `n`.
            n &= n - 1;
            Some(idx)
        }
    })
}

struct Update {
    // in my input, the longest update is 23 long. longer updates would
    // just fall back to being heap allocated.
    nodes: SmallVec<[u8; 23]>,
    node_bitset: u128,
}

fn parse_updates(input: &str) -> Vec<Update> {
    input
        .lines()
        .map(|line| {
            let nodes: SmallVec<_> = line.split(',').map(|s| s.parse::<u8>().unwrap()).collect();
            let node_bitset = nodes.iter().fold(0u128, |acc, i| acc | (1u128 << i));

            Update { nodes, node_bitset }
        })
        .collect()
}

fn parse_input(input: &str) -> (Graph, Vec<Update>) {
    let (g, u) = input.split_once("\n\n").unwrap();
    (Graph::parse(g), parse_updates(u))
}

// this is a stupid implementation in O(n^2) but the
// updates are short so it shouldn't matter too much.
fn is_well_ordered(update: &Update, g: &Graph) -> bool {
    for (i, &n) in update.nodes.iter().enumerate() {
        if update.nodes[..i].iter().any(|&m| g.has_edge(n, m)) {
            return false;
        }
    }

    true
}

// topological sort using kahn's algorithm
fn sort_topo(update: &Update, g: &Graph) -> Update {
    let mut g = g.subgraph(update.node_bitset);
    let mut out = Update { node_bitset: update.node_bitset, nodes: SmallVec::new() };

    let mut s: SmallVec<[u8; 23]> =
        bit_iter(g.nodes).filter(|i| g.in_edges[*i as usize] == 0).map(|n| n as u8).collect();

    while let Some(n) = s.pop() {
        out.nodes.push(n);
        for m in bit_iter(g.out_edges[n as usize]) {
            g.in_edges[m as usize] &= !(1u128 << n);
            if g.in_edges[m as usize] == 0 {
                s.push(m as u8);
            }
        }

        g.out_edges[n as usize] = 0;
    }

    debug_assert_eq!(out.nodes.len(), update.nodes.len());
    out
}

pub fn part1(input: &str) -> String {
    let (graph, updates) = parse_input(input);

    updates
        .iter()
        .filter(|u| is_well_ordered(u, &graph))
        .map(|u| u.nodes[u.nodes.len() / 2] as usize)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (graph, updates) = parse_input(input);

    updates
        .iter()
        .filter(|u| !is_well_ordered(u, &graph))
        .map(|u| sort_topo(u, &graph))
        .map(|u| u.nodes[u.nodes.len() / 2] as usize)
        .sum::<usize>()
        .to_string()
}
