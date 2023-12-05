use std::collections::{HashMap, HashSet};

use regex::{Captures, Regex};

pub fn parse_soil(hay: &str) -> (Vec<u64>, HashMap<String, Vec<MapItem>>) {
    let re = Regex::new(r"seeds: (?<seeds>.*)\n\nseed-to-soil map:\n(?<sts>(.*\n)*)\nsoil-to-fertilizer map:\n(?<stf>(.*\n)*)\nfertilizer-to-water map:\n(?<ftw>(.*\n)*)\nwater-to-light map:\n(?<wtl>(.*\n)*)\nlight-to-temperature map:\n(?<ltt>(.*\n)*)\ntemperature-to-humidity map:\n(?<tth>(.*\n)*)\nhumidity-to-location map:\n(?<htl>(.*|\n)*)").unwrap();
    let cap = re.captures(hay).unwrap();

    let mut map = HashMap::new();
    let seeds = cap["seeds"]
        .split_whitespace()
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect::<Vec<u64>>();

    for t in ["sts", "stf", "ftw", "wtl", "ltt", "tth", "htl"] {
        insert_type(t, &mut map, &cap);
    }

    fn insert_type(t: &str, map: &mut HashMap<String, Vec<MapItem>>, caps: &Captures<'_>) {
        map.insert(
            t.to_string(),
            caps[t]
                .split("\n")
                .map(|set| {
                    set.split_whitespace()
                        .map(|s| u64::from_str_radix(s, 10).unwrap())
                        .collect::<Vec<u64>>()
                })
                .fold(Vec::new(), |mut acc, set| {
                    if !set.is_empty() {
                        acc.push(MapItem::from_set(set));
                    }
                    acc
                }),
        );
    }
    (seeds, map)
}

#[derive(Debug, Clone)]
pub struct MapItem {
    pub source_start: u64,
    pub dest_start: u64,
    pub range: u64,
}

impl MapItem {
    pub fn from_set(set: Vec<u64>) -> Self {
        assert_eq!(set.len(), 3);
        Self {
            source_start: set[1],
            dest_start: set[0],
            range: set[2],
        }
    }
    pub fn is_covering(&self, source: u64) -> bool {
        if source >= self.source_start && source < self.source_start + self.range {
            return true;
        }
        false
    }
    pub fn map(&self, source: u64) -> u64 {
        self.dest_start + source - self.source_start
    }
    pub fn map_chunk(&self, chunk: &Chunk) -> Chunk {
        assert!(chunk.start >= self.source_start);
        assert!(chunk.end <= self.source_start + self.range);
        let transform: i128 = self.dest_start as i128 - self.source_start as i128;
        Chunk {
            start: (chunk.start as i128 + transform) as u64,
            end: (chunk.end as i128 + transform) as u64,
        }
    }
}

pub fn mapper(source: u64, map: &Vec<MapItem>) -> u64 {
    for map_item in map {
        if map_item.is_covering(source) {
            return map_item.map(source);
        }
    }
    source
}

pub fn range_mapper(source_range: Chunk, map: &Vec<MapItem>) -> Vec<Chunk> {
    let mut res = Vec::new();
    let mut transformed = Vec::new();
    for map_item in map {
        let map_chunk = Chunk {
            start: map_item.source_start,
            end: map_item.source_start + map_item.range,
        };
        if let Some(inter) = map_chunk.intersection(&source_range) {
            res.push(map_item.map_chunk(&inter));
            transformed.push(inter);
        }
    }

    let mut untransformed = vec![source_range];
    for exclude in transformed.iter() {
        let mut new_untransformed = Vec::new();
        for keep in untransformed.iter() {
            if let Some(new_keep) = exclude.non_overlap(&keep) {
                new_untransformed.extend(new_keep)
            }
        }
        untransformed = new_untransformed;
    }
    res.extend(untransformed);
    res
}

pub fn ranges_mapper(source_ranges: Vec<Chunk>, map: &Vec<MapItem>) -> Vec<Chunk> {
    let mut res = Vec::new();
    for source in source_ranges {
        res.extend(range_mapper(source, map));
    }
    res
}

pub fn seed_expand(seeds: Vec<u64>) -> Vec<u64> {
    let mut res: Vec<Chunk> = Vec::new();
    assert_eq!(seeds.len() % 2, 0);
    let mut seeds_itr = seeds.iter();
    for _ in 0..seeds.len() / 2 {
        let start = seeds_itr.next().unwrap();
        let range = seeds_itr.next().unwrap();
        let end = start + range;
        let mut candidate_chunks = HashSet::new();
        candidate_chunks.insert(Chunk { start: *start, end });

        // check for conflict with each chunk in the result built so far
        for chunk in res.iter() {
            // check all candidates (one to begin with, but it will become split up)
            let mut next_candidates = HashSet::new();
            for cand in candidate_chunks.into_iter() {
                if let Some(n_o_chunks) = chunk.non_overlap(&cand) {
                    for n_o_chunk in n_o_chunks {
                        next_candidates.insert(n_o_chunk);
                    }
                }
            }
            candidate_chunks = next_candidates
        }
        // push non-overlapping candidates to result
        for c in candidate_chunks {
            res.push(c);
        }
    }
    res.iter().fold(Vec::new(), |mut acc, c| {
        acc.extend(c.to_u64s());
        acc
    })
}

pub fn seed_chunk_expand(seeds: Vec<u64>) -> Vec<Chunk> {
    let mut res: Vec<Chunk> = Vec::new();
    assert_eq!(seeds.len() % 2, 0);
    let mut seeds_itr = seeds.iter();
    for _ in 0..seeds.len() / 2 {
        let start = seeds_itr.next().unwrap();
        let range = seeds_itr.next().unwrap();
        let end = start + range;
        res.push(Chunk { start: *start, end });
    }
    res
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chunk {
    pub start: u64,
    // non-inclusive end
    pub end: u64,
}

impl Chunk {
    pub fn non_overlap(&self, other: &Chunk) -> Option<Vec<Chunk>> {
        // other is subset
        if other.start >= self.start && other.end <= self.end {
            return None;
        }
        // other is superset
        if other.start < self.start && other.end > self.end {
            return Some(vec![
                Chunk {
                    start: other.start,
                    end: self.start,
                },
                Chunk {
                    start: self.end,
                    end: other.end,
                },
            ]);
        }
        // other earlier start partial
        if other.start < self.start && other.end > self.start {
            return Some(vec![Chunk {
                start: other.start,
                end: self.start,
            }]);
        }
        // other late end partial
        if other.end > self.end && other.start < self.end {
            return Some(vec![Chunk {
                start: self.end,
                end: other.end,
            }]);
        }
        // no overlap
        return Some(vec![Chunk {
            start: other.start,
            end: other.end,
        }]);
        panic!("overlapping case missed")
    }

    pub fn to_u64s(&self) -> Vec<u64> {
        let mut res = Vec::new();
        for idx in self.start..self.end {
            res.push(idx);
        }
        res
    }

    pub fn intersection(&self, other: &Chunk) -> Option<Chunk> {
        // no intersection
        if other.end <= self.start || other.start >= self.end {
            return None;
        }

        // full intersection
        if other.start >= self.start && other.end <= self.end {
            return Some(other.clone());
        }

        // over intersection
        if other.start < self.start && other.end > self.end {
            return Some(self.clone());
        }

        // partial other lower
        if other.start < self.start {
            return Some(Chunk {
                start: self.start,
                end: other.end,
            });
        } else {
            //partial other higher
            return Some(Chunk {
                start: other.start,
                end: self.end,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_soil() {
        let subject = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let (seeds, map) = parse_soil(subject);
        print!("{:?}", seeds);
        print!("{:?}", map);
    }

    #[test]
    fn test_seed_expand() {
        let sub = vec![79, 14, 55, 13];
        let res = vec![
            79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61, 62,
            63, 64, 65, 66, 67,
        ];
        // assert_eq!(res.len(), 27);
        assert_eq!(seed_expand(sub), res)
    }

    #[test]
    fn test_non_overlap_1() {
        let chunk = Chunk { start: 4, end: 6 };
        let mut other = Chunk { start: 2, end: 5 };
        assert_eq!(
            chunk.non_overlap(&other).unwrap(),
            vec![Chunk { start: 2, end: 4 }]
        );
        assert_eq!(
            chunk.intersection(&other).unwrap(),
            Chunk { start: 4, end: 5 }
        );
    }
    #[test]
    fn test_non_overlap_2() {
        let chunk = Chunk { start: 4, end: 6 };
        let mut other = Chunk { start: 2, end: 8 };
        assert_eq!(
            chunk.non_overlap(&other).unwrap(),
            vec![Chunk { start: 2, end: 4 }, Chunk { start: 6, end: 8 }]
        );
        assert_eq!(
            chunk.intersection(&other).unwrap(),
            Chunk { start: 4, end: 6 }
        );
    }
    #[test]
    fn test_non_overlap_3() {
        let chunk = Chunk { start: 4, end: 6 };
        let mut other = Chunk { start: 4, end: 6 };
        assert_eq!(chunk.non_overlap(&other), None);
        assert_eq!(
            chunk.intersection(&other).unwrap(),
            Chunk { start: 4, end: 6 }
        );
    }
    #[test]
    fn test_non_overlap_4() {
        let chunk = Chunk { start: 4, end: 6 };
        let mut other = Chunk { start: 5, end: 8 };
        assert_eq!(
            chunk.non_overlap(&other).unwrap(),
            vec![Chunk { start: 6, end: 8 }]
        );
        assert_eq!(
            chunk.intersection(&other).unwrap(),
            Chunk { start: 5, end: 6 }
        );
    }
}
