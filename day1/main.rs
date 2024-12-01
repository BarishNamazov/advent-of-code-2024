use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Input {
    a: Vec<i32>,
    b: Vec<i32>,
}

fn part1(mut input: Input) -> i64 {
    input.a.sort_unstable();
    input.b.sort_unstable();

    let res = input
        .a
        .iter()
        .zip(input.b.iter())
        .map(|(a, b)| (a - b).abs() as i64)
        .sum();

    res
}

fn part2(input: Input) -> i64 {
    let mut freq = HashMap::new();
    for x in input.b.iter() {
        *freq.entry(x).or_insert(0) += 1;
    }

    let similarity = input
        .a
        .iter()
        .map(|x| (*x as i64) * (*freq.get(x).unwrap_or(&0) as i64))
        .sum();

    similarity
}

fn main() {
    let mut input = Input {
        a: Vec::new(),
        b: Vec::new(),
    };

    let read_lines = |filename| -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    };

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    continue;
                }
                let mut parts = ip.split_whitespace();
                let a = parts.next().unwrap().parse().unwrap();
                let b = parts.next().unwrap().parse().unwrap();
                input.a.push(a);
                input.b.push(b);
            }
        }
    }

    println!("{}", part1(input.clone()));
    println!("{}", part2(input));
}
