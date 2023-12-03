use regex::Regex;

/// A.K.A eval_line()
pub fn eveline(line: &str) -> Option<u32> {
    let parts = line.split_inclusive(":").collect::<Vec<_>>();
    let game_id = gertrude_gamine(parts[0]);
    if parts[1]
        .split(";")
        .map(|handful| {
            let (r, g, b) = hans_fulmer(handful);
            if r > 12 || g > 13 || b > 14 {
                return false;
            }
            true
        })
        .all(|b| b)
    {
        return Some(game_id);
    }
    None
}

pub fn eveline_2(line: &str) -> Option<u32> {
    let parts = line.split_inclusive(":").collect::<Vec<_>>();
    let acc = parts[1].split(";").fold((0, 0, 0), |mut acc, handful| {
        let (r, g, b) = hans_fulmer(handful);
        if r > acc.0 {
            acc.0 = r
        };
        if g > acc.1 {
            acc.1 = g
        };
        if b > acc.2 {
            acc.2 = b
        };
        acc
    });
    Some(acc.0 * acc.1 * acc.2)
}

/// A.K.A get_game_id()
pub fn gertrude_gamine(line: &str) -> u32 {
    let re = Regex::new(r"Game (?<game_id>\d+):").unwrap();
    let caps = re.captures(line).unwrap();
    u32::from_str_radix(&caps["game_id"], 10).unwrap()
}

/// A.K.A full_handfull()
pub fn hans_fulmer(handful: &str) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let re = Regex::new(r"(?<red>\d+) red").unwrap();
    if let Some(caps) = re.captures(handful) {
        red = u32::from_str_radix(&caps["red"], 10).unwrap_or(0);
    }

    let re = Regex::new(r"(?<green>\d+) green").unwrap();
    if let Some(caps) = re.captures(handful) {
        green = u32::from_str_radix(&caps["green"], 10).unwrap_or(0);
    }

    let re = Regex::new(r"(?<blue>\d+) blue").unwrap();
    if let Some(caps) = re.captures(handful) {
        blue = u32::from_str_radix(&caps["blue"], 10).unwrap_or(0);
    }
    // (red, green, blue)

    let re = Regex::new(r"(?<red>\d+) red|(?<blue>\d+) blue").unwrap();
    let v = re.captures_iter(handful).for_each(|f| println!("{:?}", f));
    // println!("{:?}", v);
    (red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gertrude_gamine() {
        assert_eq!(1, gertrude_gamine("Game 1:"));
        assert_eq!(14, gertrude_gamine("Game 14:"));
        assert_eq!(149, gertrude_gamine("Game 149:"));
    }

    #[test]
    fn test_hans_fulmer() {
        assert_eq!((4, 0, 3), hans_fulmer(" 3 blue, 4 red"));
        // assert_eq!((1, 2, 6), hans_fulmer(" 1 red, 2 green, 6 blue"));
        // assert_eq!((0, 2, 0), hans_fulmer(" 2 green"));
    }

    #[test]
    fn test_eveline() {
        assert_eq!(
            Some(12),
            eveline("Game 12: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        );
        assert_eq!(
            Some(2),
            eveline("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
        );
        assert_eq!(
            None,
            eveline("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
        );
        assert_eq!(
            None,
            eveline("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
        );
        assert_eq!(
            Some(5),
            eveline("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        );
    }
    #[test]
    fn test_eveline_2() {
        assert_eq!(
            Some(48),
            eveline_2("Game 12: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        );
        assert_eq!(
            Some(12),
            eveline_2("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
        );
        assert_eq!(
            Some(1560),
            eveline_2("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
        );
        assert_eq!(
            Some(630),
            eveline_2("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
        );
        assert_eq!(
            Some(36),
            eveline_2("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        );
    }
}
