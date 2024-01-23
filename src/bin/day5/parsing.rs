use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1, newline, space1},
    combinator::map,
    multi::{many_till, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

use crate::types::*;

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

pub fn parse_input(i: &str) -> IResult<&str, (Seeds, Vec<AToBMap>)> {
    tuple((seeds, maps))(i)
}
