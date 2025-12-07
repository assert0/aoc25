use itertools::iproduct;
use std::fs;
use std::collections::{HashMap, HashSet};


pub fn day7(args: &[String]) {
    println!("Day 7");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

    let splitters = solve1(&map, start_position(&map));
    println!("Part 1: {:?}", splitters.len());

    let part2 = solve2(&map, start_position(&map), &mut HashMap::new());
    println!("Part 2: {:?}", part2);
}

pub fn start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    let v = iproduct!(0..map.len(), 0..map[0].len())
        .find(|(y, x)| map[*y][*x] == 'S')
        .unwrap();
    (v.0 as usize, v.1 as usize)
}

pub fn solve1(map: &Vec<Vec<char>>, position: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut splitters = HashSet::new();
    let mut beams = vec![position];
    let mut seen = HashSet::new();

    while let Some((y, x)) = beams.pop() {
        if y == map.len() { continue; }
        if !seen.insert((y, x)) { continue; }
        match map[y as usize][x as usize] {
            '^' => {
                splitters.insert((y, x));
                beams.push((y, x-1));
                beams.push((y, x+1));
            },
            '.' | 'S' => {
                beams.push((y+1, x))
            },
            _ => unreachable!()
        }
    }
    splitters
}

pub fn solve2(map: &Vec<Vec<char>>, position: (usize, usize), counts: &mut HashMap<(usize, usize), usize>) -> usize {
    let (mut y, x) = position;
    loop {
        if y == map.len() {
            return 1;
        }
        let count = counts.get(&(y, x));
        if count.is_some() { 
            return *count.unwrap();
        }
        match map[y][x] {
            '^' => {
                let c = solve2(&map, (y, x-1), counts) + solve2(&map, (y, x+1), counts);
                counts.insert((y, x), c);
                return c;
            },
            '.' | 'S' => {
                y += 1;
            },
            _ => unreachable!()
        }
    }
}

