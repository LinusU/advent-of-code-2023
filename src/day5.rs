use std::str::FromStr;

#[derive(Debug)]
struct SeedRange {
    start: u64,
    len: u64,
}

#[derive(Debug)]
struct MapRange {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl MapRange {
    fn source_intersection(&self, other: &SeedRange) -> Option<SeedRange> {
        let start = other.start.max(self.src_start);
        let end = (other.start + other.len).min(self.src_start + self.len);

        if start < end {
            Some(SeedRange {
                start,
                len: end - start,
            })
        } else {
            None
        }
    }
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dst_start, s) = s.split_once(' ').unwrap();
        let (src_start, len) = s.split_once(' ').unwrap();

        let dst_start = dst_start.parse::<u64>().unwrap();
        let src_start = src_start.parse::<u64>().unwrap();
        let len = len.parse::<u64>().unwrap();

        Ok(MapRange {
            dst_start,
            src_start,
            len,
        })
    }
}

#[derive(Debug)]
struct Map(Vec<MapRange>);

impl Map {
    fn new() -> Self {
        Map(Vec::new())
    }

    fn map_number(&self, number: u64) -> u64 {
        for range in self.0.iter() {
            if range.src_start <= number && number < range.src_start + range.len {
                return range.dst_start + (number - range.src_start);
            }
        }

        number
    }

    fn map_ranges(&self, ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        let mut queue = ranges;
        let mut result = Vec::new();

        'outer: while let Some(range) = queue.pop() {
            for map in self.0.iter() {
                if let Some(intersection) = map.source_intersection(&range) {
                    if intersection.start > range.start {
                        queue.push(SeedRange {
                            start: range.start,
                            len: intersection.start - range.start,
                        });
                    }

                    let intersection_end = intersection.start + intersection.len;
                    let range_end = range.start + range.len;

                    if intersection_end < range_end {
                        queue.push(SeedRange {
                            start: intersection_end,
                            len: range_end - intersection_end,
                        });
                    }

                    result.push(SeedRange {
                        start: map.dst_start + (intersection.start - map.src_start),
                        len: intersection.len,
                    });

                    continue 'outer;
                }
            }

            result.push(range);
        }

        result
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let mut result = seed;

        for map in self.maps.iter() {
            result = map.map_number(result);
        }

        result
    }

    fn map_seed_ranges_to_location(&self, seed_ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        let mut result = seed_ranges;

        for map in self.maps.iter() {
            result = map.map_ranges(result);
        }

        result
    }
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let seeds = lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(lines.next().unwrap(), "");

        let mut maps = Vec::new();

        while let Some(_header) = lines.next() {
            let mut map = Map::new();

            for line in lines.by_ref() {
                if line.is_empty() {
                    break;
                }

                map.0.push(MapRange::from_str(line).unwrap());
            }

            maps.push(map);
        }

        Ok(Almanac { seeds, maps })
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u64 {
    let almanac = input.parse::<Almanac>().unwrap();

    almanac
        .seeds
        .iter()
        .map(|seed| almanac.map_seed_to_location(*seed))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u64 {
    let almanac = input.parse::<Almanac>().unwrap();

    let mut seed_ranges = Vec::new();

    for start_len in almanac.seeds.chunks_exact(2) {
        seed_ranges.push(SeedRange {
            start: start_len[0],
            len: start_len[1],
        });
    }

    let location_ranges = almanac.map_seed_ranges_to_location(seed_ranges);

    location_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1("seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4"), 35);
}

#[test]
fn test_part2() {
    assert_eq!(part2("seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4"), 46);
}
