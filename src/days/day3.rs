use std::fs;

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
    // println!("{:?}", batteries);

    let part1: usize = batteries.iter().map(|l| find_part1(l)).sum();
    println!("Part 1: {}", part1);

    let part2: usize = batteries.iter().map(|l| find_part2(l)).sum();
    println!("Part 2: {:?}", part2);
}


pub fn find_part1(l: &Vec<usize>) -> usize {
    let v1 = l[..l.len() - 1].iter().max().unwrap();
    let p = l.iter().position(|v| v == v1).unwrap();
    let v2 = l[p+1..].iter().max().unwrap();
    v1 * 10 + v2
}

pub fn find_part2(l: &Vec<usize>) -> usize {
    let mut l2 = l.clone();
    // println!("");
    let mut result = vec![];
    loop {
        if result.len() == 12 { 
            return result.iter().fold(0, |acc, d| acc * 10 + *d); 
        }
        let end = l2.len() - (12 - result.len());
        let d = l2[..=end].iter().max().unwrap();
        result.push(d.clone());
        let p = l2[..=end].iter().position(|v| v == d).unwrap();
        l2 = l2[p+1..].to_vec();
        // println!("{:?}", l2);
    }
}

// pub fn find_part2(l: &Vec<usize>) -> usize {
//     let mut result = l.clone();
//     for remove in 1..=9 {
//         loop {   
//             if result.len() == 12 { 
//                 return result.iter().fold(0, |acc, d| acc * 10 + d); 
//             }
//             let p = result.iter().position(|v| *v == remove);
//             if p.is_some() {
//                 result.remove(p.unwrap());
//             } else {
//                 break;
//             }
//         }
//     }
//     unreachable!();
// }


// pub fn find_part2(l: &Vec<usize>) -> usize {
//     let mut result = l.clone();

//     let mut m: HashMap<usize, usize> = HashMap::new();
//     for v in result {
//         *m.entry(v).or_default() += 1;
//     }
//     println!("{:?}", m);
//     0
// }