use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_while1},
    character::{
        complete::{alpha1, anychar, digit1, newline, space1},
        is_alphabetic,
    },
    combinator::map,
    multi::{many_till, separated_list1},
    sequence::{delimited, separated_pair, tuple},
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

fn remap(i: &str) -> IResult<&str, MapDetails> {
    let parse_remap = separated_list1(space1, digit1);
    let remap = |s: Vec<&str>| -> MapDetails {
        let parsed = s
            .iter()
            .map(|snum| snum.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        MapDetails {
            dest_start: parsed[0],
            source_start: parsed[1],
            length: parsed[2],
        }
    };

    map(parse_remap, remap)(i)
}

fn map_details(i: &str) -> IResult<&str, Vec<MapDetails>> {
    separated_list1(newline, remap)(i)
}

fn map_type(i: &str) -> IResult<&str, MapRelation> {
    let parse_types = separated_pair(alpha1, tag("-to-"), alpha1);
    let map_type = |(type1, type2)| -> MapRelation {
        MapRelation {
            from_type: Item::from_str(type1).unwrap(),
            to_type: Item::from_str(type2).unwrap(),
        }
    };
    map(parse_types, map_type)(i)
}

fn a_to_b_map(i: &str) -> IResult<&str, FarmingMap> {
    // let parse_map = many_till(anychar, map_details);
    let parse_map = tuple((map_type, tag(" map:\n"), map_details));
    let a_to_b_map = |(map_type, _, remaps): (MapRelation, _, Vec<MapDetails>)| -> FarmingMap {
        FarmingMap {
            relation: map_type,
            map_ranges: remaps,
        }
    };
    map(parse_map, a_to_b_map)(i)
}

fn maps(i: &str) -> IResult<&str, Vec<FarmingMap>> {
    separated_list1(tuple((newline, newline)), a_to_b_map)(i)
    // let maps = |undefined_maps: Vec<FarmingMap>| -> Vec<FarmingMap> {
    //     undefined_maps
    //         .into_iter()
    //         .enumerate()
    //         .map(|(idx, val)| match idx {
    //             0 => FarmingMap {
    //                 from_item: Item::Seed
    //                 to_item: Item::SeedToSoil
    //
    //                 kind: MapTypes::SeedToSoil,
    //                 remaps: val.remaps,
    //             },
    //             1 => FarmingMap {
    //                 kind: MapTypes::SoilToFertilizer,
    //                 remaps: val.remaps,
    //             },
    //             2 => FarmingMap {
    //                 kind: MapTypes::FertilizerToWater,
    //                 remaps: val.remaps,
    //             },
    //             3 => FarmingMap {
    //                 kind: MapTypes::WaterToLight,
    //                 remaps: val.remaps,
    //             },
    //             4 => FarmingMap {
    //                 kind: MapTypes::LightToTemperature,
    //                 remaps: val.remaps,
    //             },
    //             5 => FarmingMap {
    //                 kind: MapTypes::TemperatureToHumidity,
    //                 remaps: val.remaps,
    //             },
    //             6 => FarmingMap {
    //                 kind: MapTypes::HumidityToLocation,
    //                 remaps: val.remaps,
    //             },
    //             _ => panic!("non-existent kind of map"),
    //         })
    //         .collect()
    // };
    // map(parse_maps, maps)(i)
}

pub fn parse_input(i: &str) -> IResult<&str, (Seeds, Vec<FarmingMap>)> {
    tuple((seeds, maps))(i)
}
