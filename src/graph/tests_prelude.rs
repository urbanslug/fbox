use crate::graph::types;
use std::collections::HashSet;

pub struct UnGraph {
    adj_list: Vec<Vec<u32>>,
}

pub fn make_ungraph() -> UnGraph {
    let g: UnGraph = UnGraph {
        adj_list: vec![
            vec![1],    // 0
            vec![2, 3], // 1
            vec![4],    // 2
            vec![4],    // 3
            vec![5],    // 4
        ],
    };

    g
}

pub struct DiGraph {
    adj_list: Vec<types::Edges>,
}

impl types::Graph for DiGraph {
    fn in_nodes(&self, node_id: u32) -> &HashSet<u32> {
        &self.adj_list[node_id as usize].incoming()
    }

    fn out_nodes(&self, node_id: u32) -> &HashSet<u32> {
        &self.adj_list[node_id as usize].outgoing()
    }

    fn start(&self) -> u32 {
        0
    }

    fn size(&self) -> u32 {
        self.adj_list.len() as u32
    }
}

pub fn make_dag() -> DiGraph {
    DiGraph {
        adj_list: vec![
            //                            incoming                    outgoing              node
            types::Edges::from_tuples(HashSet::from([]),     HashSet::<u32>::from([1])),    // 0
            types::Edges::from_tuples(HashSet::from([0]),    HashSet::<u32>::from([2, 3])), // 1
            types::Edges::from_tuples(HashSet::from([1]),    HashSet::<u32>::from([4])),    // 2
            types::Edges::from_tuples(HashSet::from([1]),    HashSet::<u32>::from([4])),    // 3
            types::Edges::from_tuples(HashSet::from([2, 3]), HashSet::<u32>::from([5])),    // 4
            types::Edges::from_tuples(HashSet::from([4]),    HashSet::<u32>::from([])),     // 5
        ]
    }
}
pub fn make_cyclic_digraph() -> DiGraph {
    DiGraph {
        adj_list: vec![
            //                            incoming                    outgoing              node
            types::Edges::from_tuples(HashSet::from([]),     HashSet::<u32>::from([1])),    // 0
            types::Edges::from_tuples(HashSet::from([0]),    HashSet::<u32>::from([2, 3])), // 1
            types::Edges::from_tuples(HashSet::from([1]),    HashSet::<u32>::from([4])),    // 2
            types::Edges::from_tuples(HashSet::from([1]),    HashSet::<u32>::from([4])),    // 3
            types::Edges::from_tuples(HashSet::from([2, 3]), HashSet::<u32>::from([5])),    // 4
            types::Edges::from_tuples(HashSet::from([4]),    HashSet::<u32>::from([6, 2])), // 5
            types::Edges::from_tuples(HashSet::from([5]),    HashSet::<u32>::from([])),     // 6
        ]
    }
}

pub fn make_sortable_digraph() -> DiGraph {
    DiGraph {
        adj_list: vec![
            //                            incoming                    outgoing              node nodes(sorted)
            types::Edges::from_tuples(HashSet::from([4]),    HashSet::<u32>::from([])),     // 0  6
            types::Edges::from_tuples(HashSet::from([3]),    HashSet::<u32>::from([2, 4])), // 1  3
            types::Edges::from_tuples(HashSet::from([1]),    HashSet::<u32>::from([4])),    // 2  4
            types::Edges::from_tuples(HashSet::from([5, 6]), HashSet::<u32>::from([1])),    // 3  2
            types::Edges::from_tuples(HashSet::from([1, 2]), HashSet::<u32>::from([0])),    // 4  5
            types::Edges::from_tuples(HashSet::from([]),     HashSet::<u32>::from([3])),    // 5  0
            types::Edges::from_tuples(HashSet::from([]),     HashSet::<u32>::from([3])),    // 6  1
        ]
    }
}
