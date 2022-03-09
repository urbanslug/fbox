use gfa;
use std::collections::HashSet;

use crate::graph::types;

// TODO: make this a reference
/// Takes a GFA and decomposes it to an adjacency list of its nodes,
/// identifying the nodes by node id.
pub fn decompose_gfa<T>(gfa: gfa::gfa::GFA<usize, T>) -> types::GraphList
where
    T: gfa::optfields::OptFields,
{
    // furthest_node value in nxn matrix
    let mut furthest_node: usize = 0;

    // Each path is a vector in this matrix
    let mut decomposed_graph: Vec<Vec<(usize, gfa::gfa::Orientation)>> = Vec::new();

    // Populate the decomposed_path using `gfa.paths`
    for path in gfa.paths.iter() {
        let mut decomposed_path: Vec<(usize, gfa::gfa::Orientation)> = Vec::new();
        for step in path.iter() {
            let node_id: usize = step.0;
            let orientation: gfa::gfa::Orientation = step.1;
            // zero index the node id
            decomposed_path.push((node_id - 1, orientation));

            if node_id > furthest_node {
                furthest_node = node_id;
            }
        }
        decomposed_graph.push(decomposed_path);
    }

    let empty_edges = types::Edges::new();
    let mut forward = vec![empty_edges.clone(); furthest_node];
    let mut reverse = vec![empty_edges.clone(); furthest_node];

    for decomposed_path in decomposed_graph {
        for i in 0..decomposed_path.len() - 1 {
            let from_node = decomposed_path[i].0;
            let to_node = decomposed_path[i + 1].0;

            // from direction
            match decomposed_path[i].1 {
                gfa::gfa::Orientation::Forward => {
                    forward[from_node].set_outgoing(to_node as u32); // to
                    forward[to_node].set_incoming(from_node as u32); // from
                }

                gfa::gfa::Orientation::Backward => {
                    reverse[from_node].set_outgoing(to_node as u32); // to
                    reverse[to_node].set_incoming(from_node as u32); // from
                }
            };
        }
    }

    types::GraphList {
        forward,
        reverse,
        furthest_node,
    }
}
