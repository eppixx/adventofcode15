use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn main() {
    println!("\nday 13");
    let input = include_str!("./input");

    let mut rules: HashMap<(&str, &str), i32> = input
        .lines()
        .map(|line| {
            let mut split = line.split_terminator(&[' ', '.']);
            let first = split.next().unwrap();
            let change = match split.nth(1).unwrap() {
                "gain" => str::parse::<i32>(split.next().unwrap()).unwrap(),
                "lose" => -str::parse::<i32>(split.next().unwrap()).unwrap(),
                input => unreachable!("unexpected input: {}", input),
            };
            ((first, split.nth(6).unwrap()), change)
        })
        .collect();

    let persons: HashSet<String> = input
        .lines()
        .map(|line| String::from(line.split(' ').next().unwrap()))
        .collect();
    let permutations = persons.iter().permutations(persons.len());
    let optimal_score = permutations
        .into_iter()
        .map(|p| calc_score(p, &rules))
        .max()
        .unwrap();
    println!("The total change in happiness is {}", optimal_score);

    // part 2
    persons.iter().for_each(|p| {
        rules.insert(("Self", p), 0);
        rules.insert((p, "Self"), 0);
    });
    let mut persons: HashSet<String> = input
        .lines()
        .map(|line| String::from(line.split(' ').next().unwrap()))
        .collect();
    persons.insert(String::from("Self"));
    let permutations = persons.iter().permutations(persons.len());
    let optimal_score = permutations
        .into_iter()
        .map(|p| calc_score(p, &rules))
        .max()
        .unwrap();
    println!("The total change in happiness is {}", optimal_score);
}

fn calc_score(situation: Vec<&String>, rules: &HashMap<(&str, &str), i32>) -> i32 {
    let mut score = 0;
    // start with second person and its previos neighbor
    for (i, person) in situation.iter().enumerate().skip(1) {
        let prev_person = situation[i - 1];
        score += rules[&(prev_person.as_str(), person.as_str())];
        score += rules[&(person.as_str(), prev_person.as_str())];
    }
    // dont forget the last and first person
    score += rules[&(
        situation.last().unwrap().as_str(),
        situation.first().unwrap().as_str(),
    )];
    score += rules[&(
        situation.first().unwrap().as_str(),
        situation.last().unwrap().as_str(),
    )];

    score
}
