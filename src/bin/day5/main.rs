mod parsing;
mod types;

use parsing::parse_input;
use types::*;

const DEBUG: bool = true;
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

    let locations = seeds.map_to_locations(&maps);
    let answer_p1 = locations.values().min().unwrap();
    let mut seeds_ranges = seeds.to_seeds_ranges();
    let seeds_count = seeds_ranges.0.iter().map(|r| r.len()).sum::<usize>();
    let merged = seeds_ranges.remove_overlapping();
    let merged_count = merged.0.iter().map(|r| r.len()).sum::<usize>();
    dbg!(&locations);
    println!("{answer_p1}");
    println!("seeds count: {seeds_count}");
    println!("after merging overlaps: {merged_count}");
}

fn process_seeds(seeds: Seeds) -> Vec<usize> {
    // TODO: remove overlapping ranges
    todo!()
}
