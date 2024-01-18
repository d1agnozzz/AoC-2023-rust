use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Range, RangeInclusive},
};

use nom::{
    self,
    branch::alt,
    bytes::complete::{is_a, tag, take},
    character::complete::{digit1, newline},
    combinator::{map, not},
    multi::{many1, separated_list1},
    IResult, Slice,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenType {
    Number,
    Symbol,
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct Token<'a> {
    kind: TokenType,
    position: Span<'a>,
    value: &'a str,
}

fn number<'a>(s: Span<'a>) -> IResult<Span<'a>, Token<'a>> {
    let parse_number = digit1;

    let token_number = |s: Span<'a>| -> Token<'a> {
        Token {
            kind: TokenType::Number,
            position: s,
            value: s.fragment(),
        }
    };

    map(parse_number, token_number)(s)
}

fn symbol<'a>(s: Span<'a>) -> IResult<Span<'a>, Token<'a>> {
    let check_symbol = not(alt((digit1, tag("."), tag("\n"))))(s)?;

    let parse_symbol = take(1usize);

    let token_symbol = |s: Span<'a>| -> Token<'a> {
        Token {
            kind: TokenType::Symbol,
            position: s,
            value: s.fragment().slice(0..1),
        }
    };
    map(parse_symbol, token_symbol)(check_symbol.0)
}

fn empty<'a>(s: Span<'a>) -> IResult<Span<'a>, Token<'a>> {
    let parse_empty = is_a(".");
    let token_empty = |s: Span<'a>| -> Token<'a> {
        Token {
            kind: TokenType::Empty,
            position: s,
            value: s.fragment(),
        }
    };
    map(parse_empty, token_empty)(s)
}

fn single_line_tokens(s: Span<'_>) -> IResult<Span<'_>, Vec<Token<'_>>> {
    many1(alt((number, symbol, empty)))(s)
}

fn parse_lines<'a>(s: Span<'a>) -> IResult<Span<'a>, Vec<Token<'a>>> {
    let parse_lines = separated_list1(newline, single_line_tokens);
    let tokens = |single_line_tokens: Vec<Vec<Token<'a>>>| -> Vec<Token<'a>> {
        single_line_tokens.into_iter().flatten().collect()
    };
    map(parse_lines, tokens)(s)
}

#[derive(Debug, Clone)]
struct SymbolInfo {
    location: Location,
    value: String,
}

impl SymbolInfo {
    fn with_line_length(token: Token, length: usize) -> Self {
        let row = token.position.location_line() as usize - 1;
        let column = token.position.location_offset() % length;

        SymbolInfo {
            location: Location { row, column },
            value: token.value.to_owned(),
        }
    }
    fn get_affect_range_rows(&self) -> RangeInclusive<usize> {
        self.location.row.saturating_sub(1)..=self.location.row + 1
    }
    fn get_affect_range_columns(&self) -> RangeInclusive<usize> {
        self.location.column.saturating_sub(1)..=self.location.column + 1
    }
    fn get_adjacent_numbers(
        &self,
        digit_locations: HashMap<Location, LocatedNumber>,
    ) -> HashSet<LocatedNumber> {
        let mut adjacent_ratios: HashSet<LocatedNumber> = HashSet::new();

        for row in self.get_affect_range_rows() {
            for column in self.get_affect_range_columns() {
                if let Some(located_number) = digit_locations.get(&Location { row, column }) {
                    adjacent_ratios.insert(*located_number);
                }
            }
        }

        return adjacent_ratios;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct LocatedNumber {
    start_location: Location,
    len: usize,
    // row: usize,
    value: usize,
}

impl LocatedNumber {
    fn with_line_length(token: Token, length: usize) -> Self {
        let row = token.position.location_line() as usize - 1;
        let column = token.position.location_offset() % length;

        LocatedNumber {
            start_location: Location { row, column },
            len: token.value.len(),
            // row,
            value: token
                .value
                .parse::<usize>()
                .expect("failed to parse LocatedNumber value: {value.value}"),
        }
    }

    fn get_occupied_columns_range(&self) -> Range<usize> {
        self.start_location.column..self.start_location.column + self.len
    }
    fn fill_occupied_locations(&self, digit_locations: &mut HashMap<Location, LocatedNumber>) {
        for column in self.get_occupied_columns_range() {
            digit_locations.insert(
                Location {
                    row: self.start_location.row,
                    column,
                },
                *self,
            );
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Location {
    row: usize,
    column: usize,
}

#[derive(Hash, PartialEq, Eq)]
struct Row(usize);
#[derive(Hash, PartialEq, Eq)]
struct Column(usize);

const DEBUG: bool = false;
fn main() {
    let input = include_str!("../../inputs/real/day3.txt");
    let line_length = input.lines().next().unwrap().len() + 1;

    let (_, tokens) = parse_lines(input.into()).expect("could not parse tokens: {input}");

    if DEBUG {
        for token in &tokens {
            if token.kind != TokenType::Empty {
                println!("{token:?}");
            }
        }
    }

    let mut digit_locations: HashMap<Location, LocatedNumber> = HashMap::new();
    let number_locations = tokens
        .iter()
        .filter(|token| token.kind == TokenType::Number)
        .map(|token| {
            let located_number = LocatedNumber::with_line_length(*token, line_length);
            located_number.fill_occupied_locations(&mut digit_locations);
            located_number
        })
        .collect::<Vec<LocatedNumber>>();

    if DEBUG {
        for located_number in &number_locations {
            println!("{located_number:?}");
        }
        println!();
    }

    let symbols = tokens
        .clone()
        .iter()
        .filter(|token| token.kind == TokenType::Symbol)
        .map(|token| SymbolInfo::with_line_length(*token, line_length))
        .collect::<Vec<SymbolInfo>>();

    if DEBUG {
        for symbol_info in &symbols {
            println!("{symbol_info:?}");
        }
        println!();
    }

    let mut affected_grid_positions: HashSet<Location> = HashSet::new();

    for symbol_info in &symbols {
        for row in symbol_info.get_affect_range_rows() {
            for column in symbol_info.get_affect_range_columns() {
                affected_grid_positions.insert(Location { row, column });
            }
        }
    }

    let gears = symbols
        .clone()
        .into_iter()
        .filter(|symbol_range| symbol_range.value == "*")
        .collect::<Vec<SymbolInfo>>();

    let answer_p1 = number_locations
        .iter()
        .filter(|located_number| {
            for column in located_number.get_occupied_columns_range() {
                if affected_grid_positions.contains(&Location {
                    row: located_number.start_location.row,
                    column,
                }) {
                    return true;
                }
            }

            false
        })
        .map(|located_number| located_number.value)
        .sum::<usize>();

    let answer_p2 = gears
        .iter()
        .map(|gear| gear.get_adjacent_numbers(digit_locations.clone()))
        .filter(|adjacents| adjacents.len() == 2)
        .map(|adjacents| {
            adjacents
                .iter()
                .map(|located_number| located_number.value)
                .product::<usize>()
        })
        .sum::<usize>();

    println!("{answer_p1}");
    println!("{answer_p2}");
}
