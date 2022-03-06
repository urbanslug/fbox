/*!
Perform DFS
 */

use crate::graph::types;
use std::collections::HashSet;

/// Explore every node in a graph
pub fn dfs<G: types::Graph>(graph: &G) {
    let size = graph.size() as usize;
    let mut visited: Vec<bool> = vec![false; size]; // boolean column matrix
    let mut stack = Vec::<u32>::new(); // stack

    let mut previsit = Vec::<u32>::new();
    let mut postvisit = Vec::<u32>::new();

    for start_node in 0..size {
        if visited[start_node] {
            continue;
        }

        stack.push(start_node as u32);
        loop {
            let current_node = match stack.pop() {
                Some(n) => n,
                _ => break,
            } as usize;

            // if it hasn't been visited explore it
            visited[current_node] = true;
            let outgoing_nodes = graph.out_nodes(current_node as u32);

            previsit.push(current_node as u32);

            for o in outgoing_nodes {
                if !visited[*o as usize] {
                    stack.push(current_node as u32);
                    stack.push(*o);
                    break;
                }
            }

            // outgoing are empty or are all visited
            if outgoing_nodes.is_empty() || outgoing_nodes.iter().all(|x| visited[*x as usize]) {
                postvisit.push(current_node as u32);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tests_prelude::*;

    #[test]
    fn test_dfs_dag() {
        let g = make_dag();
        dfs(&g);
    }
}
