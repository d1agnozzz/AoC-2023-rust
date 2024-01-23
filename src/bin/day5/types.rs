#[derive(Debug)]
pub struct Seeds(pub Vec<usize>);

pub struct SeedsRange {
    range_start: usize,
    range_len: usize,
}

use super::DEBUG;
use itertools::Itertools;
impl Seeds {
    fn to_seeds_ranges(&self) -> Vec<SeedsRange> {
        assert!(self.0.len() % 2 == 0);

        self.0
            .iter()
            .tuples::<(_, _)>()
            .map(|(start, len)| SeedsRange {
                range_start: *start,
                range_len: *len,
            })
            .collect()
    }

    fn map_to_locations(&self, mappings: &[AToBMap]) -> Vec<usize> {
        let mut mapped_locations = Vec::<usize>::new();
        for seed in &self.0 {
            let mut result = *seed;
            for mapping in mappings {
                if DEBUG {
                    println!("{:?}", mapping.kind);
                }
                'remaps: for remap in &mapping.remaps {
                    if remap.source_start <= result
                        && remap.source_start + remap.length >= result as usize
                    {
                        if DEBUG {
                            println!("{result} + ({} - {})", remap.dest_start, remap.source_start);
                        }
                        result = (result as i64
                            + (remap.dest_start as i64 - remap.source_start as i64))
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
}

#[derive(Debug)]
pub enum MapType {
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
pub struct AToBMap {
    pub kind: MapType,
    pub remaps: Vec<Remap>,
}
#[derive(Debug)]
pub struct Remap {
    pub dest_start: usize,
    pub source_start: usize,
    pub length: usize,
}
