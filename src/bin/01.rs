use std::collections::HashMap;

use advent_of_code::day_1::parse_frag;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    for char in input.chars() {
        if char == '-' {
            panic!("negative numbers!")
        }
    }

    let sum = input
        .split("\n")
        .map(|frag| {
            let mut front = '_';
            let mut back = '_';
            let mut search_front = true;
            let mut search_back = true;
            let mut char_iter = frag.chars();
            while search_front {
                if let Some(char) = char_iter.next() {
                    if char.is_numeric() {
                        front = char;
                        search_front = false;
                    }
                } else {
                    panic!("no numbers in line!");
                }
            }
            while search_back {
                if let Some(char) = char_iter.next_back() {
                    if char.is_numeric() {
                        back = char;
                        search_back = false;
                    }
                } else {
                    back = front;
                    search_back = false;
                }
            }
            u64::from_str_radix(&format!("{}{}", front, back), 10).unwrap()
        })
        .fold(0, |mut acc, num| {
            acc += num;
            acc
        });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut digits = HashMap::new();
    digits.insert("one", 1);
    digits.insert("two", 2);
    digits.insert("three", 3);
    digits.insert("four", 4);
    digits.insert("five", 5);
    digits.insert("six", 6);
    digits.insert("seven", 7);
    digits.insert("eight", 8);
    digits.insert("nine", 9);

    let sum = input
        .split("\n")
        .map(|frag| parse_frag(frag, &digits))
        .fold(0, |mut acc, num| {
            acc += num;
            acc
        });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
