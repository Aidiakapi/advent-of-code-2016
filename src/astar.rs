#![allow(dead_code)]

use num_traits::Zero;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Add;

pub trait Node = Clone + Eq + Hash;
pub trait Cost = Clone + Ord + Add + Zero;

#[derive(Debug, Clone)]
pub struct AStar<N: Node, C: Cost> {
    meta: HashMap<N, Meta<N, C>>,
    open: BinaryHeap<Open<N, C>>,
    path: Vec<(N, C)>,
    open_tmp: Vec<Open<N, C>>,
}

#[derive(Debug, Clone)]
struct Meta<N: Node, C: Cost> {
    is_closed: bool,
    heuristic: C,
    path: C,
    parent: Option<N>,
}

#[derive(Debug, Clone, Eq)]
struct Open<N: Node, C: Cost> {
    cost: C,
    node: N,
}

impl<N: Node, C: Cost> PartialEq for Open<N, C> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N: Node, C: Cost> PartialOrd for Open<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<N: Node, C: Cost> Ord for Open<N, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<N: Node, C: Cost> AStar<N, C> {
    pub fn new() -> Self {
        AStar {
            meta: HashMap::new(),
            open: BinaryHeap::new(),
            path: Vec::new(),
            open_tmp: Vec::new(),
        }
    }

    pub fn into_last_path(self) -> Vec<(N, C)> {
        self.path
    }

    pub fn solve<FN, FH, FD, NI>(
        &mut self,
        init: N,
        next: FN,
        heuristic: FH,
        is_done: FD,
    ) -> Option<&Vec<(N, C)>>
    where
        FN: Fn(&N) -> NI,
        FH: Fn(&N) -> C,
        FD: Fn(&N) -> bool,
        NI: Iterator<Item = (N, C)>,
    {
        self.path.clear();
        let init_heuristic = heuristic(&init);
        let init_meta = Meta {
            is_closed: false,
            path: C::zero(),
            heuristic: init_heuristic.clone(),
            parent: None,
        };
        self.meta.insert(init.clone(), init_meta);
        let init_open = Open {
            node: init,
            cost: init_heuristic,
        };
        self.open.push(init_open);

        while let Some(open) = self.open.pop() {
            if is_done(&open.node) {
                // Reconstruct the path
                let mut current_node = Some(&open.node);
                while let Some(n) = current_node {
                    let meta = &self.meta[&n];
                    self.path.push((n.clone(), meta.path.clone()));
                    current_node = meta.parent.as_ref();
                }

                self.path.reverse();

                self.open.clear();
                self.meta.clear();
                return Some(&self.path);
            }
            // Move into closed, and query path cost
            let path_cost = {
                let meta = self.meta.get_mut(&open.node).unwrap();
                meta.is_closed = true;
                meta.path.clone()
            };

            // Explore neighbors
            for (node, edge_cost) in next(&open.node) {
                match self.meta.get_mut(&node) {
                    // Already seen
                    Some(meta) => {
                        // Already in closed
                        if meta.is_closed {
                            continue;
                        }
                        // Cheaper or same price through other path
                        let path_cost = edge_cost + path_cost.clone();
                        if meta.path <= path_cost {
                            continue;
                        }
                        // Update price
                        meta.path = path_cost.clone();
                        // PERF This isn't particularly efficient
                        self.open_tmp.extend(self.open.drain());
                        self.open
                            .extend(self.open_tmp.drain(..).filter(|open| &open.node != &node));
                        self.open.push(Open {
                            node,
                            cost: path_cost + meta.heuristic.clone(),
                        });
                    }
                    // New node
                    None => {
                        let path_cost = edge_cost + path_cost.clone();
                        let heuristic_cost = heuristic(&node);
                        self.meta.insert(
                            node.clone(),
                            Meta {
                                is_closed: false,
                                path: path_cost.clone(),
                                heuristic: heuristic_cost.clone(),
                                parent: Some(open.node.clone()),
                            },
                        );
                        self.open.push(Open {
                            node,
                            cost: path_cost + heuristic_cost,
                        });
                    }
                }
            }
        }

        self.open.clear();
        self.meta.clear();
        None
    }
}
