mod day;
pub mod template;
pub use day::*;
use regex::Regex;
use std::collections::HashMap;

pub fn parse_frag(frag: &str, digits: &HashMap<&str, u64>) -> u64 {
    let first_str = find_first(frag).unwrap();
    let first = if let Ok(num) = u64::from_str_radix(first_str, 10) {
        num
    } else {
        *digits.get(&first_str).unwrap()
    };
    let last_str = find_last(frag).unwrap();
    let last = if let Ok(num) = u64::from_str_radix(&last_str, 10) {
        num
    } else {
        *digits.get(last_str.as_str()).unwrap()
    };
    u64::from_str_radix(&format!("{}{}", first.to_string(), last.to_string()), 10).unwrap()
}

pub fn find_first(frag: &str) -> Option<&str> {
    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    Some(re.find(frag)?.as_str())
}

pub fn find_last(frag: &str) -> Option<String> {
    let re = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    Some(
        re.find(&frag.chars().rev().collect::<String>())?
            .as_str()
            .chars()
            .rev()
            .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first() {
        assert_eq!("5", find_first("fh5one7twokkk").unwrap());
        assert_eq!("three", find_first("fhoonthreetwokkk").unwrap());
        assert_eq!("two", find_first("jdhitwone").unwrap());
    }

    #[test]
    fn test_find_last() {
        assert_eq!("two", &find_last("fh5one7twokkk").unwrap());
        assert_eq!("one", &find_last("fhoonthreetwonek").unwrap());
        assert_eq!("7", &find_last("7jdhitwpnnn").unwrap());
    }

    #[test]
    fn test_parse_frag() {
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
        let subject = "1twone3foursixiii";
        assert_eq!(16_u64, parse_frag(subject, &digits));
    }
}
