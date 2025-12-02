use std::fs;
use itertools::Itertools;


pub fn day2(args: &[String]) {
    println!("Day 2");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let ranges: Vec<_> = contents
        .split(',')
        .map(|l| {
            let (f, l) = l.split("-").map(|i| i.parse::<usize>().unwrap()).collect_tuple().unwrap();
            f..=l
        })
        .collect();

    let part1: usize = ranges.iter().map(|r| r.clone().filter(|v| is_invalid(*v))).flatten().sum();
    println!("Part 1: {}", part1);

    let part2: usize = ranges.iter().map(|r| r.clone().filter(|v| is_invalid_part2(*v))).flatten().sum();
    println!("Part 2: {}", part2);
}

pub fn is_invalid(v: usize) -> bool {
    let c = v.checked_ilog10().unwrap_or(0) + 1;
    let d = 10_usize.pow(c / 2);
    if c % 2 == 0 {
        v / d == v % d
    } else {
        false
    }
}

pub fn is_invalid_part2(v: usize) -> bool {
    let digits = v.checked_ilog10().unwrap_or(0) + 1;
    for c in 1..digits {
        if digits % c != 0 { continue; }
        let mut invalid = true;
        let d = 10_usize.pow(c);
        let n = v % d;
        let mut v2 = v / d;
        while v2 > 0 {
            // println!("{} {}", v2, n);
            if v2 % d != n {
                invalid = false;
                break;
            }
            v2 /= d;
        }
        if invalid {
            return true
        }
    }
    false
}
