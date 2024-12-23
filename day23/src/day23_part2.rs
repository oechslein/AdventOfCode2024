use std::iter::once;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use miette::Result;
use rayon::prelude::*;
use utils::cache_it_with_fxhashmap;

type NodeSet<'a> = FxHashSet<&'a str>;
type NodesWithEdges<'a> = FxHashMap<&'a str, Vec<&'a str>>;

pub fn process(input: &str) -> Result<String> {
    let nodes_with_edges = create_nodes_with_edges(input);
    let largest_node_set = largest_node_set(NodeSet::default(), &nodes_with_edges);
    Ok(largest_node_set.1)
}

fn largest_node_set(node_set: NodeSet, nodes_with_edges: &NodesWithEdges) -> (usize, String) {
    cache_it_with_fxhashmap!(
        String,
        (usize, String),
        node_set.iter().sorted().join(","),
        {
            nodes_with_edges
                .keys()
                .par_bridge()
                .filter(|&node| !node_set.contains(node))
                .filter(|&node| is_connected(node, &node_set, nodes_with_edges))
                .map(|&node| {
                    let new_node_set = node_set.iter().copied().chain(once(node)).collect();
                    largest_node_set(new_node_set, nodes_with_edges)
                })
                .max_by_key(|(size, _node_set_as_str)| *size)
                .unwrap_or_else(|| (node_set.len(), node_set.into_iter().sorted().join(",")))
        }
    )
}

#[inline]
fn is_connected(node: &str, node_set: &NodeSet, nodes_with_edges: &NodesWithEdges) -> bool {
    node_set
        .iter()
        .all(|node2| nodes_with_edges[node].contains(node2))
}

fn create_nodes_with_edges(input: &str) -> NodesWithEdges {
    let mut nodes_with_edges: NodesWithEdges = FxHashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        nodes_with_edges.entry(a).or_default().push(b);
        nodes_with_edges.entry(b).or_default().push(a);
    }
    nodes_with_edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("co,de,ka,ta", process(input)?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(
            "bg,bu,ce,ga,hw,jw,nf,nt,ox,tj,uu,vk,wp",
            process(&input.replace('\r', ""))?
        );
        Ok(())
    }
}
