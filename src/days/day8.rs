use std::{collections::HashSet, fs};
use itertools::Itertools;

pub fn day8(args: &[String]) {
    println!("Day 8");
    if args.len() != 2 {
        println!("Missing input file and part1 connections");
        return;
    }
    let filename = &args[0];
    let part1_connections: usize = args[1].parse().unwrap();
    println!("In file {} part1 connections {}", filename, part1_connections);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let boxes: Vec<(i64, i64, i64)> = contents.lines()
        .map(|l| l.split(",")
            .map(|v| v.parse().unwrap()).collect_tuple().unwrap())
        .collect();

    let mut pairs_dist: Vec<_> = 
        boxes.iter().combinations(2)
            .map(|b| ((b[0], b[1]), distance(b[0], b[1]))).collect();

    pairs_dist.sort_by(|a, b| a.1.cmp(&b.1));

    let mut circuits: Vec<HashSet<(i64, i64, i64)>> = vec![];
    for (count, (pair, _)) in pairs_dist.iter().enumerate() {
        if count == part1_connections { 
            // Part 1
            let part1 = circuits.iter()
                .map(|c| c.len())
                .sorted()
                .rev()
                .take(3)
                .product::<usize>();
            println!("Part 1: {}", part1);
        }
        let mut next_circuits: Vec<HashSet<(i64, i64, i64)>> = vec![];
        circuits.push([*pair.0, *pair.1].into_iter().collect::<HashSet<(i64, i64, i64)>>());
        
        // Merge any intersecting circuits into a single circuit
        while let Some(mut c) = circuits.pop() {
            for i in (0..circuits.len()).rev() {
                if !c.is_disjoint(&circuits[i]) {
                    // intersecting circuits
                    c = c.union(&circuits.remove(i)).copied().collect();
                }
            }
            next_circuits.push(c);
        }
        circuits = next_circuits;
        
        // Part 2
        if boxes.len() == circuits.iter().map(|c| c.len()).sum() {
            println!("Part 2: {}", pair.0.0 * pair.1.0);
            break
        }
    }
}

fn distance(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    let (dx, dy, dz) = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
    (dx.pow(2) + dy.pow(2) + dz.pow(2)).isqrt()
}
