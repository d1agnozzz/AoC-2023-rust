use core::panic;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1, newline, space1},
    combinator::map,
    multi::{many_till, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn seeds(i: &str) -> IResult<&str, Seeds> {
    let parse_seeds = delimited(
        tuple((tag("seeds:"), space1)),
        separated_list1(space1, digit1),
        tuple((newline, newline)),
    );

    let seeds = |s: Vec<&str>| -> Seeds {
        Seeds(
            s.iter()
                .map(|seed| seed.parse::<usize>().unwrap())
                .collect(),
        )
    };

    map(parse_seeds, seeds)(i)
}

fn remap(i: &str) -> IResult<&str, Remap> {
    let parse_remap = separated_list1(space1, digit1);
    let remap = |s: Vec<&str>| -> Remap {
        let parsed = s
            .iter()
            .map(|snum| snum.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Remap {
            dest_start: parsed[0],
            source_start: parsed[1],
            length: parsed[2],
        }
    };

    map(parse_remap, remap)(i)
}

fn map_details(i: &str) -> IResult<&str, Vec<Remap>> {
    separated_list1(newline, remap)(i)
}

fn a_to_b_map(i: &str) -> IResult<&str, AToBMap> {
    let parse_map = many_till(anychar, map_details);
    let a_to_b_map = |(_, remaps): (_, Vec<Remap>)| -> AToBMap {
        AToBMap {
            kind: MapType::None,
            remaps,
        }
    };
    map(parse_map, a_to_b_map)(i)
}

fn maps(i: &str) -> IResult<&str, Vec<AToBMap>> {
    let parse_maps = separated_list1(tuple((newline, newline)), a_to_b_map);
    let maps = |undefined_maps: Vec<AToBMap>| -> Vec<AToBMap> {
        undefined_maps
            .into_iter()
            .enumerate()
            .map(|(idx, val)| match idx {
                0 => AToBMap {
                    kind: MapType::SeedToSoil,
                    remaps: val.remaps,
                },
                1 => AToBMap {
                    kind: MapType::SoilToFertilizer,
                    remaps: val.remaps,
                },
                2 => AToBMap {
                    kind: MapType::FertilizerToWater,
                    remaps: val.remaps,
                },
                3 => AToBMap {
                    kind: MapType::WaterToLight,
                    remaps: val.remaps,
                },
                4 => AToBMap {
                    kind: MapType::LightToTemperature,
                    remaps: val.remaps,
                },
                5 => AToBMap {
                    kind: MapType::TemperatureToHumidity,
                    remaps: val.remaps,
                },
                6 => AToBMap {
                    kind: MapType::HumidityToLocation,
                    remaps: val.remaps,
                },
                _ => panic!("non-existent kind of map"),
            })
            .collect()
    };
    map(parse_maps, maps)(i)
}

fn parse_input(i: &str) -> IResult<&str, (Seeds, Vec<AToBMap>)> {
    tuple((seeds, maps))(i)
}

#[derive(Debug)]
struct Seeds(Vec<usize>);
#[derive(Debug)]
enum MapType {
    None,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct AToBMap {
    kind: MapType,
    remaps: Vec<Remap>,
}
#[derive(Debug)]
struct Remap {
    dest_start: usize,
    source_start: usize,
    length: usize,
}
const DEBUG: bool = false;
fn main() {
    let input = include_str!("../../inputs/real/day5.txt");

    let (_, (seeds, maps)) = parse_input(input).unwrap();

    println!("{seeds:?}");
    if DEBUG {
        for map in &maps {
            println!("{:?}", map.kind);
            for remap in &map.remaps {
                println!("{remap:?}");
            }
            println!();
        }
    }

    let locations = seeds_to_locations(seeds, maps);
    let answer_p1 = locations.iter().min().unwrap();
    println!("{answer_p1}");
}

fn seeds_to_locations(seeds: Seeds, maps: Vec<AToBMap>) -> Vec<usize> {
    let mut mapped_locations = Vec::<usize>::new();
    for seed in &seeds.0 {
        let mut result = *seed;
        for map in &maps {
            if DEBUG {
                println!("{:?}", map.kind);
            }
            'remaps: for remap in &map.remaps {
                if remap.source_start <= result
                    && remap.source_start + remap.length >= result as usize
                {
                    if DEBUG {
                        println!("{result} + ({} - {})", remap.dest_start, remap.source_start);
                    }
                    result = (result as i64 + (remap.dest_start as i64 - remap.source_start as i64))
                        as usize;
                    if DEBUG {
                        println!("{result}");
                    }
                    break 'remaps;
                }
                if DEBUG {
                    println!("{result}");
                }
            }
        }
        mapped_locations.push(result);
    }
    mapped_locations
}

fn process_seeds(seeds: Seeds) -> Vec<usize> {
    // TODO: remove overlapping ranges
    todo!()
}
