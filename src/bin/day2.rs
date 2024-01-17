use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[derive(Debug)]
struct Bag(CubesSubset);

#[derive(Debug, PartialEq)]
struct CubesSubset {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

impl CubesSubset {
    fn is_within_range(&self, other: &CubesSubset) -> bool {
        let (r1, g1, b1) = match self {
            CubesSubset { red, green, blue } => (
                red.unwrap_or_default(),
                green.unwrap_or_default(),
                blue.unwrap_or_default(),
            ),
        };
        let (r2, g2, b2) = match other {
            CubesSubset { red, green, blue } => (
                red.unwrap_or_default(),
                green.unwrap_or_default(),
                blue.unwrap_or_default(),
            ),
        };
        r1 <= r2 && g1 <= g2 && b1 <= b2
    }
}

impl PartialOrd for CubesSubset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.red.partial_cmp(&other.red) {
            Some(core::cmp::Ordering::Equal | core::cmp::Ordering::Less) => {}
            ord => return ord,
        }
        match self.green.partial_cmp(&other.green) {
            Some(core::cmp::Ordering::Equal | core::cmp::Ordering::Less) => {}
            ord => return ord,
        }
        self.blue.partial_cmp(&other.blue)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    cubes_subsets: Vec<CubesSubset>,
}

fn game_id(input: &str) -> IResult<&str, usize> {
    let parse_game_id = delimited(tag("Game "), digit1, tag(": "));

    let game_id = |s: &str| s.parse::<usize>().expect("could not parse game id:: {s}");

    map(parse_game_id, game_id)(input)
}

fn cubes(input: &str) -> IResult<&str, Cube> {
    let parse_cube = separated_pair(
        digit1,
        nom::character::complete::space1,
        alt((tag("red"), tag("green"), tag("blue"))),
    );

    let cube = |(quantity, color): (&str, &str)| -> Cube {
        let quantity = quantity
            .parse::<usize>()
            .expect("could not parse cube quantity: {quantity}");
        match color {
            "red" => Cube::Red(quantity),
            "green" => Cube::Green(quantity),
            "blue" => Cube::Blue(quantity),
            _ => panic!("unknown cube color: {color}"),
        }
    };

    map(parse_cube, cube)(input)
}

fn cubes_subset(input: &str) -> IResult<&str, CubesSubset> {
    let parse_cubes_subset = separated_list0(tag(", "), cubes);

    let cubes_subset = |cubes: Vec<Cube>| {
        let mut cubes_subset = CubesSubset {
            red: None,
            green: None,
            blue: None,
        };

        for cube in cubes {
            match cube {
                Cube::Red(q) => cubes_subset.red = Some(q),
                Cube::Green(q) => cubes_subset.green = Some(q),
                Cube::Blue(q) => cubes_subset.blue = Some(q),
            }
        }

        cubes_subset
    };

    map(parse_cubes_subset, cubes_subset)(input)
}

fn game_subsets(input: &str) -> IResult<&str, Vec<CubesSubset>> {
    separated_list0(tag("; "), cubes_subset)(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let parse_game = tuple((game_id, game_subsets));

    let game = |(game_id, game_subsets): (usize, Vec<CubesSubset>)| Game {
        id: game_id,
        cubes_subsets: game_subsets,
    };

    map(parse_game, game)(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, game)(input)
}

const DEBUG: bool = false;

fn main() {
    let actual_bag = Bag(CubesSubset {
        red: Some(12),
        green: Some(13),
        blue: Some(14),
    });

    let input = include_str!("../../inputs/real/day2.txt");

    let (_, games) = parse_games(input).expect("could not parse games");

    if DEBUG {
        for game in &games {
            println!("{game:?}");
        }
    }

    let answer_p1 = games
        .iter()
        .filter_map(|game| {
            if game
                .cubes_subsets
                .iter()
                .all(|cube_subset| cube_subset.is_within_range(&actual_bag.0))
            {
                if DEBUG {
                    println!("{} is possible", game.id);
                }
                Some(game.id)
            } else {
                if DEBUG {
                    println!("{} is NOT possible", game.id);
                }
                None
            }
        })
        .sum::<usize>();

    let answer_p2 = games
        .iter()
        .map(|game| {
            let mut min_possible_bag = Bag(CubesSubset {
                red: Some(0),
                green: Some(0),
                blue: Some(0),
            });

            for cubes_subset in &game.cubes_subsets {
                let CubesSubset { red, green, blue } = *cubes_subset;

                if red > min_possible_bag.0.red {
                    min_possible_bag.0.red = red;
                }
                if green > min_possible_bag.0.green {
                    min_possible_bag.0.green = green;
                }
                if blue > min_possible_bag.0.blue {
                    min_possible_bag.0.blue = blue;
                }
            }
            if DEBUG {
                println!("game {}: min possible bag {:?}", game.id, min_possible_bag);
            }
            min_possible_bag.0.red.unwrap_or(1)
                * min_possible_bag.0.green.unwrap_or(1)
                * min_possible_bag.0.blue.unwrap_or(1)
        })
        .sum::<usize>();
    println!("{answer_p1}");
    println!("{answer_p2}");
}
