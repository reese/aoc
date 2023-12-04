use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

#[aoc(day1, part1)]
fn part1(lines: &Vec<String>) -> u32 {
    lines
        .into_iter()
        .map(|s| {
            let digits = s.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn str_to_v(s: &str) -> Option<u32> {
    if let Some(d) = s.chars().next().unwrap().to_digit(10) {
        Some(d)
    } else {
        if s.starts_with("one") {
            Some(1)
        } else if s.starts_with("two") {
            Some(2)
        } else if s.starts_with("three") {
            Some(3)
        } else if s.starts_with("four") {
            Some(4)
        } else if s.starts_with("five") {
            Some(5)
        } else if s.starts_with("six") {
            Some(6)
        } else if s.starts_with("seven") {
            Some(7)
        } else if s.starts_with("eight") {
            Some(8)
        } else if s.starts_with("nine") {
            Some(9)
        } else if s.starts_with("zero") {
            Some(0)
        } else {
            None
        }
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<String>) -> u32 {
    let mut s = 0;
    for line in input.iter() {
        let mut vs = vec![];
        for (c, _) in line.char_indices() {
            let ss = line.split_at(c).1;
            if let Some(d) = str_to_v(ss) {
                vs.push(d);
            }
        }
        s += format!("{}{}", vs.first().unwrap(), vs.last().unwrap())
            .parse::<u32>()
            .unwrap();
    }
    s
}
