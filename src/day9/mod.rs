use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let input = include_str!("./input");

    let mut locations = HashSet::new();
    let mut graph: HashMap<(&str, &str), usize> = HashMap::new();
    input.lines().for_each(|line| {
        // line structure: location to location = distance
        let mut split = line.split(' ');
        let from = split.next().unwrap();
        split.next();
        let to = split.next().unwrap();
        split.next();
        locations.insert(from);
        locations.insert(to);
        let distance = str::parse::<usize>(split.next().unwrap()).unwrap();
        // flight between two locations works both ways
        graph.insert((from, to), distance);
        graph.insert((to, from), distance);
    });

    let mut possible_routes: HashMap<Vec<&str>, usize> = HashMap::new();
    let perm = locations.iter().permutations(locations.len());
    perm.for_each(|perm| {
        let cost = perm.iter().fold((0, None), |acc, location| match acc {
            (_, None) => (0, Some(location)),
            (sum, Some(prev)) => match graph.get(&(prev, *location)) {
                Some(distance) => (sum + distance, Some(location)),
                None => unreachable!("input is not a complete graph"),
            },
        });
        possible_routes.insert(perm.iter().map(|s| **s).collect(), cost.0);
    });

    println!("\nday 9");
    println!(
        "shortest distance between locations {:?}",
        possible_routes.iter().map(|(_, cost)| cost).min()
    );
    println!(
        "longest distance between locations {:?}",
        possible_routes.iter().map(|(_, cost)| cost).max()
    )
}
