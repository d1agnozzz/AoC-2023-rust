use std::{collections::HashMap, hash::Hash};

const SPELLED_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn generate_spelled_map() -> HashMap<&'static str, u8> {
    let mut spelled_map: HashMap<&str, u8> = HashMap::new();

    for i in 0..SPELLED_DIGITS.len() {
        spelled_map.insert(SPELLED_DIGITS.get(i).unwrap(), i as u8);
    }

    spelled_map
}

fn process_input(input: &str) -> usize {
    let spelled_digits = generate_spelled_map();

    dbg!(&spelled_digits);

    input
        .lines()
        .map(|line| {
            let mut digit_by_index: HashMap<usize, u8> = HashMap::new();

            let findable = line.to_owned();

            for (spell, num) in &spelled_digits {
                if let Some(idx) = findable.find(spell) {
                    digit_by_index.insert(idx, *num);
                }
                if let Some(idx) = findable.rfind(spell) {
                    digit_by_index.insert(idx, *num);
                }
            }

            for (i, ch) in line.chars().enumerate().filter(|(i, ch)| ch.is_numeric()) {
                digit_by_index.insert(i, ch.to_digit(10).unwrap() as u8);
            }

            let left = digit_by_index
                .get(digit_by_index.keys().min().unwrap())
                .copied()
                .unwrap_or_default();
            let right = digit_by_index
                .get(digit_by_index.keys().max().unwrap())
                .copied()
                .unwrap_or_default();

            dbg!(&digit_by_index);

            println!("{} : {}", left, right);

            let number_order =
                if digit_by_index.keys().min().unwrap() == digit_by_index.keys().max().unwrap() {
                    0
                } else {
                    10
                };
            println!("{}", number_order);
            println!("########");

            (left * number_order + right) as usize
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/real/day1p2.txt");
    // let input = "kdzrjbh2txzz5hbone96one";

    let answer: usize = process_input(input);

    println!("{}", answer);
}
