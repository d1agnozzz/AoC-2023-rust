use std::collections::BTreeMap;

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn generate_spelled_map() -> BTreeMap<&'static str, u8> {
    let mut spelled_map: BTreeMap<&str, u8> = BTreeMap::new();

    for i in 0..SPELLED_DIGITS.len() {
        spelled_map.insert(SPELLED_DIGITS.get(i).unwrap(), 1 + i as u8);
    }

    spelled_map
}

fn process_input(input: &str) -> usize {
    let spelled_digits = generate_spelled_map();

    dbg!(&spelled_digits);

    input
        .lines()
        .map(|line| {
            // BTreeMap is used just for the sake of debug purposes, so that when outputting the
            // Map, indexes are sorted. Could be replaced with HashMap without drawbacks
            let mut digit_by_index: BTreeMap<usize, u8> = BTreeMap::new();

            let findable = line.to_owned();

            for (spell, num) in &spelled_digits {
                if let Some(idx) = findable.find(spell) {
                    digit_by_index.insert(idx, *num);
                }
                if let Some(idx) = findable.rfind(spell) {
                    digit_by_index.insert(idx, *num);
                }
            }

            for (i, ch) in line.chars().enumerate().filter(|(_, ch)| ch.is_numeric()) {
                digit_by_index.insert(i, ch.to_digit(10).unwrap() as u8);
            }

            if digit_by_index.is_empty() {
                return 0;
            }
            let left = digit_by_index
                .get(digit_by_index.keys().min().unwrap_or_else(|| {
                    panic!("could not find min in digit_by_index: {digit_by_index:?}, {line}")
                }))
                .copied()
                .unwrap_or_default();
            let right = digit_by_index
                .get(digit_by_index.keys().max().unwrap())
                .copied()
                .unwrap_or_default();

            println!("{line}");
            println!("{:?}", &digit_by_index);
            println!("{left} : {right}");

            let left_order = 10;
            println!("result: {}", left * left_order + right);
            println!("########");

            left as usize * left_order as usize + right as usize
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/real/day1.txt");
    // let input = "eight";

    let answer: usize = process_input(input);

    println!("{answer}");
}
