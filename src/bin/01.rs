use std::ops::Mul;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    /* this first example is copied from [matthsp](https://github.com/matthsp/aoc-2024-rust/tree/main)
    I have zero Rust experience, aside from reading the first 4 chapters of "The Book" this was a little flight check to make
    sure I had AOC configured correctly */

    let (mut left, mut right) = input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let left = split.next();
            let right = split.next();

            (
                left.unwrap().parse::<u32>().unwrap(),
                right.unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>();

    left.sort();
    right.sort();

    let mut diff: u32 = 0;

    for (i, l) in left.iter().enumerate() {
        diff += l.abs_diff(right[i]);
    }

    Some(left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let left = split.next().unwrap().parse::<u32>().unwrap();
            let right = split.next().unwrap().parse::<u32>().unwrap();
            (left, right)
        })
        .unzip();

    let counts: u32 = left
        .iter()
        .map(|l| l * right.iter().filter(|&x| x == l).count() as u32)
        .sum();

    println!("{}", counts);
    Some(counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
