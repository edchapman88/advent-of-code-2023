mod day;
pub mod template;
pub use day::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

pub fn parse_hay(hay: &str) -> HashMap<&str, Vec<MyMatch>> {
    let re = Regex::new(r"(?<num>\d+)|(?<symbol>[^0-9\n\.])").unwrap();
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

#[derive(Debug, Clone, PartialEq)]
pub struct MyMatch {
    pub start: usize,
    pub end: usize,
    pub string: String,
}

pub fn dims(hay: &str) -> (usize, usize) {
    let chunks = hay.split_inclusive("\n").collect::<Vec<_>>();
    (chunks.len(), chunks[0].chars().count())
}

pub fn coords(i: usize, dims: (usize, usize)) -> Coord {
    let y = i / dims.1;
    let x = i - (y * dims.1);
    Coord {
        x: i64::try_from(x).unwrap(),
        y: i64::try_from(y).unwrap(),
        dims,
    }
}

/// zero indexed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
    pub dims: (usize, usize),
}

impl Coord {
    pub fn adj(&self) -> HashSet<Coord> {
        let (x, y) = (self.x, self.y);
        let mut set = HashSet::new();
        for c in [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ] {
            let coord_cand = Coord {
                x: c.0,
                y: c.1,
                dims: self.dims,
            };
            if self.on_grid(&coord_cand) {
                set.insert(coord_cand);
            }
        }
        set
    }

    fn on_grid(&self, coord: &Coord) -> bool {
        if coord.x < 0 || coord.y < 0 {
            return false;
        }
        // coords are 0-indexed, dims are 1-indexed
        if coord.x + 1 > i64::try_from(self.dims.1).unwrap()
            || coord.y + 1 > i64::try_from(self.dims.0).unwrap()
        {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frag() {
        let subject = "16+....89\n16...#.89";
        let map = parse_hay(subject);
        // println!("{:?}", map["symbol"]);
        assert_eq!(4, map["num"].len());
        // test exclusion of \n char
        assert_eq!(2, map["symbol"].len());
    }

    #[test]
    fn test_dims() {
        let subject = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....";
        let dims = dims(subject);
        // dims includes the \n char, so this unit test looks like it should
        // be (9,10), because the \n chars are hidden by the code editor.
        assert_eq!((9, 11), dims);
    }

    #[test]
    fn test_cords() {
        assert_eq!(
            Coord {
                y: 0,
                x: 0,
                dims: (5, 5)
            },
            coords(0, (5, 5))
        );
        assert_eq!(
            Coord {
                y: 0,
                x: 3,
                dims: (5, 5)
            },
            coords(3, (5, 5))
        );
        assert_eq!(
            Coord {
                y: 0,
                x: 4,
                dims: (5, 5)
            },
            coords(4, (5, 5))
        );
        assert_eq!(
            Coord {
                y: 1,
                x: 0,
                dims: (5, 5)
            },
            coords(5, (5, 5))
        );
        assert_eq!(
            Coord {
                y: 1,
                x: 1,
                dims: (5, 5)
            },
            coords(6, (5, 5))
        );
        assert_eq!(
            Coord {
                y: 1,
                x: 4,
                dims: (5, 5)
            },
            coords(9, (5, 5))
        );
    }

    #[test]
    fn test_adj() {
        let coord = Coord {
            x: 1,
            y: 1,
            dims: (3, 3),
        };
        let adj = coord.adj();
        for c in adj.iter() {
            println!("{:?}", [c.x, c.y])
        }
    }
}
