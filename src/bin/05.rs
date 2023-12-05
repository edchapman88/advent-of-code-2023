use advent_of_code::day_5::{mapper, parse_soil, ranges_mapper, seed_chunk_expand};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, map) = parse_soil(input);
    let mut locs = Vec::new();
    for seed in seeds {
        let soil = mapper(seed, &map["sts"]);
        let fert = mapper(soil, &map["stf"]);
        let water = mapper(fert, &map["ftw"]);
        let light = mapper(water, &map["wtl"]);
        let temp = mapper(light, &map["ltt"]);
        let hum = mapper(temp, &map["tth"]);
        let loc = mapper(hum, &map["htl"]);
        locs.push(loc);
    }
    locs.sort();
    Some(locs[0])
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds_raw, map) = parse_soil(input);
    let seeds = seed_chunk_expand(seeds_raw);
    println!("{:?}", seeds.len());
    // let mut locs = Vec::new();
    // for seed in &seeds {
    let soil = ranges_mapper(seeds, &map["sts"]);
    let fert = ranges_mapper(soil, &map["stf"]);
    let water = ranges_mapper(fert, &map["ftw"]);
    let light = ranges_mapper(water, &map["wtl"]);
    let temp = ranges_mapper(light, &map["ltt"]);
    let hum = ranges_mapper(temp, &map["tth"]);
    let locs = ranges_mapper(hum, &map["htl"]);
    // locs.extend(loc);
    // }
    let mut loc_starts = locs
        .iter()
        .map(|loc_chunk| loc_chunk.start)
        .collect::<Vec<_>>();
    loc_starts.sort();
    Some(loc_starts[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two_sub() {
        let (seeds_raw, map) = parse_soil(&advent_of_code::template::read_file("examples", DAY));
        let soil = mapper(82, &map["sts"]);
        let fert = mapper(soil, &map["stf"]);
        let water = mapper(fert, &map["ftw"]);
        let light = mapper(water, &map["wtl"]);
        let temp = mapper(light, &map["ltt"]);
        let hum = mapper(temp, &map["tth"]);
        let loc = mapper(hum, &map["htl"]);
        println!(
            "{} {} {} {} {} {} {}",
            soil, fert, water, light, temp, hum, loc
        );

        assert_eq!(loc, 46);
    }
}
