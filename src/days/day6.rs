use std::fs;
use std::iter::zip;

pub fn day6(args: &[String]) {
    println!("Day 6");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<_> = contents.lines().collect();
    let ops: Vec<_> = lines.iter().last().unwrap().split_ascii_whitespace().collect();

    let groups: Vec<Vec<usize>> = lines[..lines.len()-1].iter()
        .map(|l| l.split_ascii_whitespace().map(|v| v.parse().unwrap()).collect())
        .collect();
    let groups_rotated: Vec<Vec<_>> = (0..groups[0].len())
        .map(|x| (0..groups.len())
            .map(|y| groups[y][x]).collect()).collect();
    println!("Part 1: {}", solve(&ops, &groups_rotated));

    let input: Vec<Vec<_>> = lines[..lines.len()-1].iter().map(|l| l.chars().collect()).collect();
    let rotated: Vec<Vec<_>> = (0..input[0].len()).map(|x| (0..input.len()).map(| y| input[y][x]).collect()).collect();
    let nums: Vec<_> = rotated.iter()
        .map(|n| n.iter().collect::<String>().trim().to_string().parse::<usize>())
        .collect();
    let groups2: Vec<Vec<_>> = nums
        .split(| n | n.is_err() )
        .map(|s| s.iter().map(|v| v.clone().unwrap()).collect())
        .collect();
    // println!("{:?}", groups2);
    println!("Part 2: {}", solve(&ops, &groups2));
}

pub fn solve(ops: &Vec<&str>, groups: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for (op, g) in zip(ops, groups) {
        // println!("{} {:?}", op, g);
        let r = match *op {
            "+" => g.iter().sum(),
            "*" => g.iter().product(),
            _ => 0
        };
        result += r;
    }
    result
}
