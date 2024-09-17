use super::DEBUG;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Range;
#[derive(Debug)]
pub struct Seeds(pub Vec<usize>);

impl Seeds {
    pub fn to_seeds_ranges(&self) -> SeedsRanges {
        assert!(self.0.len() % 2 == 0);

        SeedsRanges(
            self.0
                .iter()
                .tuples::<(_, _)>()
                .map(|(start, len)| (*start..start + len + 1))
                .collect(),
        )
    }

    pub fn map_to_locations(&self, mappings: &[FarmingMap]) -> HashMap<ItemValue, ItemValue> {
        let mut mapped_locations = HashMap::<ItemValue, ItemValue>::new();
        for seed in &self.0 {
            let mut seed_item = ItemValue {
                item: Item::Seed,
                value: *seed,
            };

            let mut current_mapped = seed_item;
            for mapping in mappings {
                if DEBUG {
                    println!(
                        "{:?} to {:?}",
                        mapping.relation.from_type, mapping.relation.to_type
                    );
                }
                current_mapped = mapping.map(&current_mapped);
                if DEBUG {
                    println!("{current_mapped:?}");
                }
            }
            mapped_locations.insert(seed_item, current_mapped);
        }
        mapped_locations
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct SeedsRange {
    start: usize,
    len: usize,
}

impl SeedsRange {
    fn end(&self) -> usize {
        self.start + self.len
    }
    pub fn count_seeds(&self) -> usize {
        self.len
    }
    fn extend(&mut self, other: &Self) {
        assert!(other.start >= self.start && other.start <= self.end());

        let start_diff = other.start - self.start;

        self.len = other.len + start_diff;
    }
}

pub struct SeedsRanges(pub Vec<Range<usize>>);

impl SeedsRanges {
    pub fn remove_overlapping(&mut self) -> Self {
        self.0.sort_by(|a, b| a.start.cmp(&b.start));
        let mut result = vec![self.0.first().unwrap().clone()];

        for original_range in &self.0[1..] {
            let last_processed = result.last_mut().unwrap();

            if original_range.start >= last_processed.start
                && original_range.start <= last_processed.end
            {
                last_processed.end = std::cmp::max(original_range.end, last_processed.end);
            } else {
                result.push(original_range.clone());
            }
        }
        SeedsRanges(result)
    }
    pub fn map_to_locations_ranges(&self, mappings: &mut [AToBMap]) -> Vec<Range<usize>> {
        for seed_range in &self.0 {
            let mut current_mapped = vec![seed_range.clone()];
            for range_chunk in current_mapped {
                for mapping in &mut *mappings {
                    current_mapped = mapping.map_range(&range_chunk);
                }
            }
        }
        todo!()
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub enum Item {
    Undefined,
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

use std::str::FromStr;
impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err(()),
        }
    }
}

use derivative::Derivative;
#[derive(Derivative)]
#[derivative(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemValue {
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    pub item: Item,
    pub value: usize,
}

#[derive(Debug)]
pub struct MapRelation {
    pub from_type: Item,
    pub to_type: Item,
}

pub struct FarmingMap {
    pub relation: MapRelation,
    pub map_ranges: Vec<MapDetails>,
}
impl FarmingMap {
    fn map(&self, item_value: &ItemValue) -> ItemValue {
        assert!(item_value.item == self.relation.from_type);
        for map_range in &self.map_ranges {
            if map_range.source_start <= item_value.value
                && item_value.value <= map_range.source_end()
            {
                let result = item_value.value + map_range.dest_start - map_range.source_start;
                return ItemValue {
                    item: self.relation.to_type,
                    value: result,
                };
            }
        }
        ItemValue {
            item: self.relation.to_type,
            value: item_value.value,
        }
    }
}

#[derive(Debug)]
pub enum MapTypes {
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
    pub kind: MapTypes,
    pub remaps: Vec<MapDetails>,
}
impl AToBMap {
    fn map(&self, value: usize) -> usize {
        let mut result = value;
        for remap in &self.remaps {
            if remap.source_start <= value && value <= remap.source_start + remap.length {
                if DEBUG {
                    println!("{value} + ({} - {})", remap.dest_start, remap.source_start);
                }
                result = (value as i128 + (remap.dest_start as i128 - remap.source_start as i128))
                    as usize;
            }
        }
        result
    }
    fn map_range(&mut self, seed_range: &Range<usize>) -> Vec<Range<usize>> {
        self.remaps
            .sort_by(|a, b| a.source_start.cmp(&b.source_start));

        let mut effective_mappings = self
            .remaps
            .iter()
            .copied()
            .filter(|m| {
                let map_range = m.source_start..=m.source_start + m.length;
                map_range.contains(&seed_range.start) || map_range.contains(&(seed_range.end - 1))
            })
            .map(|m| m.dest_start..m.length + 1)
            .collect::<Vec<_>>();

        effective_mappings.first_mut().unwrap().start = seed_range.start;
        effective_mappings.last_mut().unwrap().end = seed_range.end;

        effective_mappings
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ClosedInterval {
    pub low: usize,
    pub high: usize,
}
impl ClosedInterval {
    pub fn new(first: usize, second: usize) -> Self {
        ClosedInterval {
            low: first,
            high: second,
        }
    }
    pub fn overlaps(&self, other: &ClosedInterval) -> bool {
        if self.low <= other.high && self.high >= other.low {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MapDetails {
    pub dest_start: usize,
    pub source_start: usize,
    pub length: usize,
}

impl MapDetails {
    fn source_interval(&self) -> ClosedInterval {
        ClosedInterval::new(self.source_start, self.source_end())
    }
    fn dest_interval(&self) -> ClosedInterval {
        ClosedInterval::new(self.dest_start, self.dest_end())
    }
    fn source_end(&self) -> usize {
        self.source_start + self.length
    }
    fn dest_end(&self) -> usize {
        self.dest_start + self.length
    }
    fn trim_start_to(&mut self, value: usize) {
        assert!(value >= self.source_start);
        assert!(value <= self.source_start + self.length);

        let diff = value - self.source_start;
        self.source_start += diff;
        self.dest_start += diff;
    }
    fn trim_end_to(&mut self, value: usize) {
        assert!(value <= self.source_start + self.length);
        assert!(value >= self.source_start);

        let diff = self.source_end() - value;
        self.length -= diff;
    }
}
