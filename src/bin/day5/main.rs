mod parsing;
mod types;

use parsing::parse_input;
use types::*;

const DEBUG: bool = false;
fn main() {
    let input = include_str!("../../../inputs/real/day5.txt");

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
