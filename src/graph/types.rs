use std::collections::HashSet;

/// Not a singlular of [Edges]
/// the node that an edge goes out of and into
#[derive(Debug)]
pub struct Edge(u32, u32);
impl Edge {
    pub fn new(from: u32, to: u32) -> Self {
        Self(from, to)
    }

    /// The node that an edge is coming from
    pub fn from(&self) -> u32 {
        self.0
    }

    /// The node an edge is going to
    pub fn to(&self) -> u32 {
        self.1
    }
}

/// A tuple of edges going in and coming out of a vertex
/// 0 => from (incoming nodes)
/// 1 => to (outgoing nodes)
#[derive(Debug, Clone)]
pub struct Edges(HashSet<u32>, HashSet<u32>);

impl Edges {
    pub fn new() -> Self {
        Edges(HashSet::<u32>::new(), HashSet::<u32>::new())
    }

    /// Create edges from tuples
    pub fn from_tuples(from: HashSet<u32>, to: HashSet<u32>) -> Self {
        Edges(from, to)
    }

    // incoming edges
    pub fn incoming(&self) -> &HashSet<u32> {
        &self.0
    }

    // outgoing edges
    pub fn outgoing(&self) -> &HashSet<u32> {
        &self.1
    }

    // incoming edges
    pub fn set_incoming(&mut self, n: u32) {
        self.0.insert(n);
    }

    // outgoing edges
    pub fn set_outgoing(&mut self, n: u32) {
        self.1.insert(n);
    }
}


type AdjacencyList = Vec<Edges>;

// TODO: merge forward and reverse
/// We have different representations or graphs for the forward and
/// reverse edges
/// Genome graph as a list
pub struct GraphList {
	pub forward: AdjacencyList,
	pub reverse: AdjacencyList,
    pub furthest_node: usize
}


// Makes no sense for undirected graph
pub trait Graph {
    fn in_nodes(&self, node_id: u32) -> &HashSet<u32>;
    fn out_nodes(&self, node_id: u32) -> &HashSet<u32>;
    fn start(&self) -> u32;
    fn size(&self) -> u32; // optional
}
