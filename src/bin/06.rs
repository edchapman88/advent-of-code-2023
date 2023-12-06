use std::iter::zip;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let time = vec![41, 66, 72, 66];
    let dist = vec![244, 1047, 1228, 1040];

    let mut ranges = Vec::new();
    for (t_, d_) in zip(time, dist) {
        let (t, d) = (t_ as f64, d_ as f64);
        let mut one = (t + f64::sqrt(t.powi(2) - (4.0 * d))) / 2.0;
        let mut two = (t - f64::sqrt(t.powi(2) - (4.0 * d))) / 2.0;
        let solutions = if one < two {
            if one.fract() == 0.0 {
                one += 1.0;
            }
            if two.fract() == 0.0 {
                two -= 1.0;
            }
            (one.ceil() as u32, two.floor() as u32)
        } else {
            if one.fract() == 0.0 {
                one -= 1.0;
            }
            if two.fract() == 0.0 {
                two += 1.0;
            }
            (two.ceil() as u32, one.floor() as u32)
        };
        // println!("{:?}", solutions);
        ranges.push((solutions.0..=solutions.1).count() as u64);
    }
    Some(ranges.iter().fold(1_u64, |mut acc, n| {
        acc *= n;
        acc
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let t = 41667266_f64;
    let d = 244104712281040_f64;
    // let t = 71530_f64;
    // let d = 940200_f64;

    let mut one = (t + f64::sqrt(t.powi(2) - (4.0 * d))) / 2.0;
    let mut two = (t - f64::sqrt(t.powi(2) - (4.0 * d))) / 2.0;
    let solutions = if one < two {
        if one.fract() == 0.0 {
            one += 1.0;
        }
        if two.fract() == 0.0 {
            two -= 1.0;
        }
        (one.ceil() as u32, two.floor() as u32)
    } else {
        if one.fract() == 0.0 {
            one -= 1.0;
        }
        if two.fract() == 0.0 {
            two += 1.0;
        }
        (two.ceil() as u32, one.floor() as u32)
    };
    // println!("{:?}", solutions);
    Some((solutions.0..=solutions.1).count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let result = part_one("not_needed");
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two("not_needed");
        assert_eq!(result, Some(71503));
    }
}
