use itertools::iproduct;
use std::fs;
use lazy_static::lazy_static;

pub fn day4(args: &[String]) {
    println!("Day 4");
    if args.len() != 1 {
        println!("Missing input file");
        return;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

    let part1 = solve1(&map);
    println!("Part 1: {}", part1);

    let part2 = solve2(&map);
    println!("Part 2: {}", part2);
}

pub fn solve1(map: &Vec<Vec<char>>) -> usize {
    iproduct!(0..map.len() as isize, 0..map[0].len() as isize)
        .filter(|p| map_value(&map, *p) == '@')
        .map(| p | adjacent(p)
            .iter().map(|p| map_value(&map, *p))
            .filter(|v| *v == '@')
            .count()
        )
        .filter(|c| *c < 4)
        .count()
}

pub fn solve2(map: &Vec<Vec<char>>) -> usize {
    let mut map2 = map.clone();
    let mut part2 = 0;
    loop {
        let remove: Vec<_> = iproduct!(0..map2.len() as isize, 0..map2[0].len() as isize)
            .filter(|p| map_value(&map2, *p) == '@')
            .map(| p | (p, adjacent(p)
                .iter().map(|p| map_value(&map2, *p))
                .filter(|v| *v == '@')
                .count())
            )
            .filter(|(_, c)| *c < 4)
            .map(|(p, _)| p)
            .collect();
        if remove.len() == 0 { break }
        part2 += remove.len();
        for (y, x) in remove {
            map2[y as usize][x as usize] = '.';
        }
    }
    part2
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
}

pub fn adjacent(pos: (isize, isize)) -> Vec<(isize, isize)> {
    ADJ.iter()
        .map(|(dy, dx)| (pos.0 + dy, pos.1 + dx))
        .collect()
}

pub fn map_value(map: &Vec<Vec<char>>, p: (isize, isize)) -> char {
    if (0..map.len() as isize).contains(&p.0) && (0..map[0].len() as isize).contains(&p.1) {
        return map[p.0 as usize][p.1 as usize];
    }
    '#'
}
