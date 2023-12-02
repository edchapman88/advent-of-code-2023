use advent_of_code::day_2::{eveline, eveline_2};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|line| eveline(line))
            .fold(0, |mut acc, num| {
                if let Some(n) = num {
                    acc += n;
                };
                acc
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|line| eveline_2(line))
            .fold(0, |mut acc, num| {
                if let Some(n) = num {
                    acc += n;
                };
                acc
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
