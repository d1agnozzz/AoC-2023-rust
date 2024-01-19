use std::collections::{BTreeMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn card_number(i: &str) -> IResult<&str, usize> {
    let parse_card_number = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    );
    let card_number =
        |s: &str| -> usize { s.parse::<usize>().expect("couldn't parse card number: {s}") };
    map(parse_card_number, card_number)(i)
}

fn number_list(i: &str) -> IResult<&str, Vec<usize>> {
    let parse_number_list = preceded(opt(space1), separated_list1(space1, digit1));

    let number_list = |v: Vec<&str>| -> Vec<usize> {
        v.iter()
            .map(|e| {
                e.parse::<usize>()
                    .expect("couldn't parse number from list: {e}")
            })
            .collect()
    };

    map(parse_number_list, number_list)(i)
}

fn card(i: &str) -> IResult<&str, Card> {
    let parse_card = tuple((card_number, number_list, tag(" |"), number_list));

    let card = |(id, win, _, game): (usize, Vec<usize>, &str, Vec<usize>)| -> Card {
        Card {
            id,
            winning_numbers: win,
            game_numbers: game,
        }
    };
    map(parse_card, card)(i)
}

fn parse_input(i: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, card)(i)
}

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct CardId(usize);

#[derive(Debug, Hash, Eq, PartialEq)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    game_numbers: Vec<usize>,
}

impl Card {
    fn game_win_count(&self) -> usize {
        let win_set: HashSet<&usize> = HashSet::from_iter(&self.winning_numbers);
        let game_set: HashSet<&usize> = HashSet::from_iter(&self.game_numbers);

        win_set.intersection(&game_set).count()
    }
}

const DEBUG: bool = true;

fn main() {
    let input = include_str!("../../inputs/real/day4.txt");

    let (_, cards) = parse_input(input).unwrap();

    if DEBUG {
        for card in &cards {
            println!("{card:?}");
        }
    }

    let answer_p1 = cards
        .iter()
        .map(|card| {
            let game_score = card.game_win_count();

            if DEBUG {
                println!("{}: {}", card.id, game_score.saturating_sub(1));
            }

            match game_score {
                0 => 0,
                _ => 1 << (game_score - 1),
            }
        })
        .sum::<usize>();

    let mut cards_copies: BTreeMap<CardId, usize> = cards
        .iter()
        .map(|card| (CardId(card.id - 1), 1))
        .collect::<BTreeMap<_, _>>();

    for (index, card) in cards.iter().enumerate() {
        let count = card.game_win_count();

        for i in index + 1..=index + count {
            if i < cards_copies.len() {
                *cards_copies.entry(CardId(i)).or_insert(1) += cards_copies[&CardId(index)];
            }
        }
    }

    dbg!(&cards_copies);

    let answer_p2 = cards_copies.values().sum::<usize>();

    println!("{answer_p1}");
    println!("{answer_p2}");
}
