use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use petgraph::{
    dot::{Config, Dot},
    Graph, Undirected,
};

pub struct Day25Solution {}

pub fn day25(input: &str) -> Result<f32> {
    solve_linear::<Day25Solution, _, _, _>(input)
}

/// update to include bridges to remove
const FILTER: [(&str, &str); 3] = [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")];

impl SolutionLinear<Graph<String, usize, Undirected>, usize, String> for Day25Solution {
    fn load(input: &str) -> Result<Graph<String, usize, Undirected>> {
        let str_map: HashMap<String, Vec<String>> =
            input.lines().fold(HashMap::new(), |mut hm, l| {
                let (tag, oth) = l.split_once(": ").unwrap();
                oth.split_whitespace().for_each(|o| {
                    hm.entry(tag.to_owned()).or_default().push(o.to_owned());
                    hm.entry(o.to_owned()).or_default().push(tag.to_owned());
                });
                hm
            });

        let mut g = Graph::new_undirected();
        let mut ixs = HashMap::new();
        for k in str_map.keys() {
            ixs.insert(k, g.add_node(k.clone()));
        }

        for (k, vs) in str_map.iter() {
            for v in vs {
                if FILTER.contains(&(k, v)) || FILTER.contains(&(v, k)) {
                    continue;
                }
                if !g.contains_edge(*ixs.get(v).unwrap(), *ixs.get(k).unwrap()) {
                    g.add_edge(*ixs.get(k).unwrap(), *ixs.get(v).unwrap(), 0);
                }
            }
        }

        // turn this into something readable with `dot -Tsvg -Kneato test.dot > test.svg`
        let mut f = File::create("test.dot").unwrap();
        let output = format!("{}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
        let _ = f.write_all(output.as_bytes());

        Ok(g)
    }

    fn part1(input: &mut Graph<String, usize, Undirected>) -> Result<usize> {
        // pick node at random
        let n = input.node_indices().next().unwrap();
        let mut visited = HashSet::new();
        let mut to_visit = vec![n];

        while let Some(v) = to_visit.pop() {
            visited.insert(v);
            to_visit.extend(input.neighbors(v).filter(|n| !visited.contains(n)));
        }

        Ok(visited.len() * (input.node_count() - visited.len()))
    }

    fn part2(
        _input: &mut Graph<String, usize, Undirected>,
        _part_1_solution: usize,
    ) -> Result<String> {
        Ok("Merry Christmas :)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Day25Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
",
        54,
        "Merry Christmas :)".to_string()
    )]
    fn validate_day25(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: String) {
        let mut input = Day25Solution::load(input).unwrap();

        let p1 = Day25Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day25Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}
