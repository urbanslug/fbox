/*!
Detect back edges in a graph
 */

use std::collections::HashSet;
use crate::graph::types;


pub fn find_back_edges<G: types::Graph>(graph: &G) -> Vec<types::Edge>  {
    let size = graph.size() as usize;
    let mut visited: Vec<bool> = vec![false; size]; // boolean column matrix
    let mut in_stack: Vec<bool> = vec![false; size]; // boolean column matrix
    let mut stack = Vec::<u32>::new();
    let mut backedges = Vec::<types::Edge>::new();

    for v in 0..size {
        if visited[v] {
            continue;
        }

        stack.push(v as u32);
        in_stack[v] = true;

        loop {
            let current_node = match stack.pop() {
                Some(n) => n,
                _ => {
                    eprintln!("Done");
                    break
                },
            } as usize;

            in_stack[current_node] = false;
            visited[current_node] = true;

            // -------
            // explore
            // -------
            for o in graph.out_nodes(current_node as u32) {
                // an outgoing edge is in the stack
                if in_stack[*o as usize] {
                    backedges.push(types::Edge::new(current_node as u32, *o));
                }

                if !visited[*o as usize] {
                    in_stack[current_node] = true;
                    in_stack[*o as usize] = true;
                    stack.push(current_node as u32);
                    stack.push(*o);
                    break;
                }
            }
        }
    }

    backedges
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests_prelude::*;

    #[test]
    fn test_dfs_cycle() {
        let g = make_cyclic_digraph();
        let _back_edges = find_back_edges(&g);
    }
}
