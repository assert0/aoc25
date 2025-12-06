use itertools::Itertools;
use std::fs; 
use rangetools::{BoundedSet, Rangetools};

pub fn day5(args: &[String]) {
    println!("Day 5");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sections = contents.split("\n\n");
    let idranges: Vec<_> = sections.next().unwrap()
        .lines().map(|l| {
            let (l, r) = l.split("-")
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple().unwrap();
            l..=r
        })
        .collect();
    let ids: Vec<usize> = sections.next().unwrap()
        .lines().map(|l| l.parse().unwrap())
        .collect();

    let part1 = ids.iter()
        .filter(|id| idranges.iter().any(|r| r.contains(id))).count();
    println!("Part 1: {}", part1);

    let part2 = idranges.into_iter().fold(BoundedSet::empty(), | acc, r | acc.union(r)).into_iter().count();
    println!("Part 2: {}", part2);
}
