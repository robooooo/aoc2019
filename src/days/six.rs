use crate::utils;
use err_derive::Error;
use petgraph::{algo, prelude::*};
use std::{str::FromStr, collections::HashMap};

pub fn solve() -> utils::Result<(i32, i32)> {
    let input: Vec<Orbit> = utils::get_lines(utils::path("six.txt"))?;
    let mut graph: Graph<(), i32, Undirected> = Graph::new_undirected();
    let mut map: HashMap<&str, NodeIndex> = HashMap::new();

    for orbit in &input {
        let src = *map.entry(&orbit.src).or_insert_with(|| graph.add_node(()));
        let dst = *map.entry(&orbit.dst).or_insert_with(|| graph.add_node(()));
        graph.add_edge(src, dst, 1);
    }

    let dji = algo::dijkstra(&graph, map["COM"], None, |e| *e.weight());
    let res_one: i32 = dji.into_iter().map(|kv| kv.1).sum();

    let san = algo::dijkstra(&graph, map["YOU"], Some(map["SAN"]), |e| *e.weight());
    let res_two = san[&map["SAN"]] - 2;

    Ok((res_one, res_two))
}

struct Orbit {
    src: String,
    dst: String,
}

impl FromStr for Orbit {
    type Err = OrbitParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(")");
        let src = iter.next().ok_or(OrbitParseErr::NoFirstArg)?;
        let dst = iter.next().ok_or(OrbitParseErr::NoSecondArg)?;
        match iter.next() {
            None => Ok(Orbit {
                src: src.into(),
                dst: dst.into(),
            }),
            Some(_) => Err(OrbitParseErr::TooManyArgs),
        }
    }
}

#[derive(Debug, Error)]
enum OrbitParseErr {
    #[error(display = "could not find first argument")]
    NoFirstArg,
    #[error(display = "could not find second argument")]
    NoSecondArg,
    #[error(display = "too many arguments")]
    TooManyArgs,
}