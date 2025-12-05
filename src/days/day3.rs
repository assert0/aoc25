use std::fs;
use iter_first_max::IterFirstMaxExt as _;


pub fn day3(args: &[String]) {
    println!("Day 3");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let batteries: Vec<Vec<_>> = contents.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
        .collect();

    let part1: usize = batteries.iter().map(|l| find_part1(l)).sum();
    println!("Part 1: {}", part1);
    let part1_2: usize = batteries.iter().map(|l| find_part2(l, 2)).sum();
    println!("Part 1: {}", part1_2);

    let part2: usize = batteries.iter().map(|l| find_part2(l, 12)).sum();
    println!("Part 2: {:?}", part2);
}


pub fn find_part1(l: &Vec<usize>) -> usize {
    let (p1, v1) = l[..l.len()-1].into_iter().enumerate().first_max_by_key(|&(_, v)| v).unwrap();
    let v2 = l.into_iter().skip(p1 + 1).max().unwrap();
    v1 * 10 + v2
}

pub fn find_part2(l: &Vec<usize>, c: usize) -> usize {
    let l2: Vec<_> = l.into_iter().enumerate().collect();
    let mut result = 0;
    let mut start = 0;
    for end in l2.len()-c..l2.len() {
        let (p, v) = l2[..=end].iter().skip(start).first_max_by_key(|&(_, v)| v).unwrap();
        result = result * 10 + *v;
        start = *p + 1;
    }
    result
}
