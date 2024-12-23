use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::*;

use miette::Result;

#[allow(clippy::similar_names)]
pub fn process(input: &str) -> Result<String> {
    let mut nodes_with_edges: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        nodes_with_edges.entry(a).or_default().push(b);
        nodes_with_edges.entry(b).or_default().push(a);
    }

    let nodes = nodes_with_edges.keys().collect_vec();
    let result = nodes
        .par_iter()
        .enumerate()
        .map(|(node1_index, node1)| {
            nodes
                .iter()
                .enumerate()
                .skip(node1_index + 1)
                .map(|(node2_index, node2)| {
                    nodes
                        .iter()
                        .skip(node2_index + 1)
                        .filter(|&node3| check(&[node1, node2, node3], &nodes_with_edges))
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(result.to_string())
}

fn check(node_slice: &[&str], nodes_with_edges: &FxHashMap<&str, Vec<&str>>) -> bool {
    node_slice.iter().any(|node| node.starts_with('t'))
        && node_slice
            .iter()
            .tuple_combinations()
            .all(|(node1, node2)| nodes_with_edges[node1].contains(node2))
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
        assert_eq!("7", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("1314", process(&input.replace('\r', ""))?);
        Ok(())
    }
}
