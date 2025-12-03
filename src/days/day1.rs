use std::fs;
use iter_accumulate::IterAccumulate;

pub fn day1(args: &[String]) {
    println!("Day 1");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let rotations: Vec<i32> = contents
        .lines()
        .map(|l| {
            let v = l[1..].parse::<i32>().unwrap();
            match l[..1].as_ref() {
                "L" => -v,
                "R" => v,
                _ => unreachable!(),
            }
        })
        .collect();

    let part1 = rotations.iter()
        .accumulate(50, |acc, r| (acc + r) % 100)
        .filter(|p| *p == 0)
        .count();
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    let mut pointing = 50;
    for r in rotations {
        for _ in 0..r.abs() {
            if pointing == 0 {
                part2 += 1;
            }
            pointing += if r < 0 { 99 } else { 1 };
            pointing %= 100;
        } 
    }
    println!("Part 2: {}", part2);
}
