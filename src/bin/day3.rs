use std::{
    collections::HashSet,
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

const DEBUG: bool = false;

#[derive(Debug, Clone)]
struct SymbolInfo {
    location: Position,
    // affected_rows: RangeInclusive<usize>,
    // affected_columns: RangeInclusive<usize>,
    value: String,
    adjacent_numbers: Vec<LocatedNumber>,
}

// impl Hash for Sy

#[derive(Debug, Clone, Copy)]
struct LocatedNumber {
    start_location: Position,
    len: usize,
    row: usize,
    value: usize,
}

impl LocatedNumber {
    fn with_line_length(token: Token, length: usize) -> Self {
        let row = token.position.location_line() as usize - 1;
        let column = token.position.location_offset() % length;

        LocatedNumber {
            start_location: Position { row, column },
            len: token.value.len(),
            row,
            value: token
                .value
                .parse::<usize>()
                .expect("failed to parse LocatedNumber value: {value.value}"),
        }
    }

    fn get_column_range(&self) -> Range<usize> {
        self.start_location.column..self.start_location.column + self.len
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    row: usize,
    column: usize,
}

impl SymbolInfo {
    fn with_line_length(token: Token, length: usize) -> Self {
        let row = token.position.location_line() as usize - 1;
        let column = token.position.location_offset() % length;

        SymbolInfo {
            location: Position { row, column },
            // affected_rows: row.saturating_sub(1)..=row + 1,
            // affected_columns: column.saturating_sub(1)..=column + 1,
            value: token.value.to_owned(),
            adjacent_numbers: Vec::new(),
        }
    }
    fn get_affect_range_rows(&self) -> RangeInclusive<usize> {
        self.location.row.saturating_sub(1)..=self.location.row + 1
    }
    fn get_affect_range_columns(&self) -> RangeInclusive<usize> {
        self.location.column.saturating_sub(1)..=self.location.column + 1
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Row(usize);
#[derive(Hash, PartialEq, Eq)]
struct Column(usize);

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

    let number_locations = tokens
        .iter()
        .filter(|token| token.kind == TokenType::Number)
        .map(|token| LocatedNumber::with_line_length(*token, line_length))
        .collect::<Vec<LocatedNumber>>();

    if DEBUG {
        for located_number in &number_locations {
            println!("{located_number:?}");
        }
        println!();
    }

    let symbol_infos = tokens
        .clone()
        .iter()
        .filter(|token| token.kind == TokenType::Symbol)
        .map(|token| SymbolInfo::with_line_length(*token, line_length))
        .collect::<Vec<SymbolInfo>>();

    if DEBUG {
        for symbol_info in &symbol_infos {
            println!("{symbol_info:?}");
        }
        println!();
    }

    let mut affected_grid_positions: HashSet<Position> = HashSet::new();
    // let mut affected_grid_rows: HashSet<Row> = HashSet::new();
    // let mut affected_grid_columns: HashSet<Column> = HashSet::new();

    for symbol_info in &symbol_infos {
        for row in symbol_info.get_affect_range_rows() {
            for column in symbol_info.get_affect_range_columns() {
                affected_grid_positions.insert(Position { row, column });
            }
        }
        // for row in affect_range.rows {
        //     affected_grid_rows.insert(Row(row));
        // }
        // for column in affect_range.columns {
        //     affected_grid_columns.insert(Column(column));
        // }
    }

    let answer_p1 = number_locations
        .iter()
        .filter(|located_number| {
            // let is_affected_row = affected_grid_rows.contains(&Row(located_number.row));
            //
            // let mut is_affected_column = false;
            // for column in located_number.column_start..=located_number.column_end {
            //     if affected_grid_columns.contains(&Column(column)) {
            //         is_affected_column = true;
            //     }
            // }

            for column in located_number.get_column_range() {
                if affected_grid_positions.contains(&Position {
                    row: located_number.row,
                    column,
                }) {
                    return true;
                }
            }

            false
        })
        .map(|located_number| located_number.value)
        .sum::<usize>();
    let gears_affect_range = symbol_infos
        .clone()
        .into_iter()
        .filter(|symbol_range| symbol_range.value == "*")
        .collect::<Vec<SymbolInfo>>();

    println!("{answer_p1}");
}
