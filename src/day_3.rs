use std::collections::HashMap;

use regex::Regex;

use crate::MyMatch;

pub fn parse_hay_2(hay: &str) -> HashMap<&str, Vec<MyMatch>> {
    let re = Regex::new(r"(?<num>\d+)|(?<symbol>\*)").unwrap();
    let mut mtchs = HashMap::new();
    mtchs.insert("num", vec![]);
    mtchs.insert("symbol", vec![]);
    re.captures_iter(hay).for_each(|c| {
        for name in ["num", "symbol"] {
            if let Some(mtch) = c.name(name) {
                mtchs.get_mut(name).unwrap().push(MyMatch {
                    start: mtch.start(),
                    end: mtch.end(),
                    string: mtch.as_str().to_string(),
                });
            }
        }
    });
    mtchs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frag() {
        let subject = "467..114..
                        ...*......
                        ..35..633.
                        ......#...
                        617*......
                        .....+.58.
                        ..592.....
                        ......755.
                        ...$.*....
                        .664.598..";
        let map = parse_hay_2(subject);
        // println!("{:?}", map["symbol"]);
        assert_eq!(10, map["num"].len());
        // test exclusion of \n char
        assert_eq!(3, map["symbol"].len());
    }
}
