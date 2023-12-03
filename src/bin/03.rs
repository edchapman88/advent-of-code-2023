use std::{
    collections::{HashMap, HashSet},
    iter,
};

use advent_of_code::{coords, day_3::parse_hay_2, dims, parse_hay, Coord, MyMatch};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let dims = dims(input);
    let mtchs = parse_hay(input);

    let mut s_locs = HashSet::new();
    for mtch in mtchs["symbol"].iter() {
        s_locs.insert(coords(mtch.start, dims));
    }

    let mut total = 0;
    for mtch in mtchs["num"].iter() {
        for digit_idx in mtch.start..mtch.end {
            let coord = coords(digit_idx, dims);
            let adj = coord.adj();
            let mut inter = adj.intersection(&s_locs);
            println!("{} {:?}", mtch.string, inter);

            if inter.next().is_some() {
                total += u64::from_str_radix(&mtch.string, 10).unwrap();
                break;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let dims = dims(input);
    let mtchs = parse_hay_2(input);

    let mut s_locs = HashSet::new();
    for mtch in mtchs["symbol"].iter() {
        s_locs.insert(coords(mtch.start, dims));
    }

    let mut gears: HashMap<Coord, Vec<MyMatch>> = HashMap::new();

    let mut total = 0;
    for mtch in mtchs["num"].iter() {
        for digit_idx in mtch.start..mtch.end {
            let coord = coords(digit_idx, dims);
            let adj = coord.adj();
            let mut inter = adj.intersection(&s_locs);
            println!("{} {:?}", mtch.string, inter);

            // number adjacents of a number might intersect with multiple gears
            for gear in inter {
                if let Some(existing_nums) = gears.get_mut(gear) {
                    if !existing_nums.contains(mtch) {
                        existing_nums.push(mtch.clone());
                    }
                } else {
                    gears.insert(gear.clone(), vec![mtch.clone()]);
                }
            }

            // if inter.next().is_some() {
            //     total += u64::from_str_radix(&mtch.string, 10).unwrap();
            //     break;
            // }
        }
    }

    for (gear, nums) in gears {
        if nums.len() == 2 {
            total += u64::from_str_radix(&nums[0].string, 10).unwrap()
                * u64::from_str_radix(&nums[1].string, 10).unwrap()
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(13542));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
