use std::collections::HashMap;

use regex::Regex;

pub fn parse_scratch(hay: &str) -> HashMap<u32, Card> {
    // .* is a catch all, because . is anything and * is the repetition operator
    let re = Regex::new(r"Card\s*(?<idx>\d+): (?<win>.*) \| (?<hand>.*)").unwrap();
    let cards = re.captures_iter(hay);
    // println!("{:?}", caps.count());
    let mut map = HashMap::new();
    for card in cards {
        let id = u32::from_str_radix(&card["idx"], 10).unwrap();
        let win: Vec<u32> = card["win"]
            .split_whitespace()
            .map(|s| u32::from_str_radix(s, 10).unwrap())
            .collect();
        let hand: Vec<u32> = card["hand"]
            .split_whitespace()
            .map(|s| u32::from_str_radix(s, 10).unwrap())
            .collect();

        // println!("_______________");
        // println!("card:  {:?}", id);
        // println!("win:  {:?}", win);
        // println!("hand:  {:?}", hand);

        map.insert(id, Card { id, win, hand }); 
    }
    map
}

pub fn process_card(card: &Card, buffer: &mut HashMap<u32, u32>) {
    let mut mtchs = 0_u32;
    for h in card.hand.iter() {
        if card.win.contains(&h) {
            mtchs += 1
        }
    }
    for mtch_idx in 1..=mtchs {
        let id_to_add = card.id + mtch_idx;
        if let Some(buf) = buffer.get_mut(&id_to_add) {
            *buf += 1
        } else {
            buffer.insert(id_to_add, 1_u32);
        }
    }
    // println!("processing: {:?}", card.id);
}

pub fn count_to_score(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    return 2_u32.pow(count - 1);
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Card {
    pub id: u32,
    pub win: Vec<u32>,
    pub hand: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frag() {
        let subject = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let map = parse_scratch(subject);
        assert_eq!(vec![13, 32, 20, 16, 61], map[&2].win);
        assert_eq!(vec![61, 30, 68, 82, 17, 32, 24, 19], map[&2].hand);
        // println!("{:?}", map["symbol"]);
        // assert_eq!(10, map[""].len());
        // test exclusion of \n char
        // assert_eq!(3, map["symbol"].len());
    }

    #[test]
    fn test_parse_frag_1() {
        let subject = 
        "Card  98: 55 73 67 88 41 44  9 95 14 10 | 32 75 93 26 59 79 77 73 19 71  9 18 90 33 84 80 10 15 95 21 62 34 58 37 81
        Card  99: 98 27 60 28  7 45 25 19 82 76 | 90 64 23  4 32 67 45 37 18  7 65 61 78 25 14 28 81 39 48 69  8 66 60 82 76";
        let map = parse_scratch(subject);
        // assert_eq!(vec![13, 32, 20, 16, 61], map[&2].win);
        // assert_eq!(vec![61, 30, 68, 82, 17, 32, 24, 19], map[&2].hand);
        // println!("{:?}", map["symbol"]);
        // assert_eq!(10, map[""].len());
        // test exclusion of \n char
        // assert_eq!(3, map["symbol"].len());
    }

    #[test]
    fn test_count_to_score() {
        assert_eq!(0, count_to_score(0));
        assert_eq!(1, count_to_score(1));
        assert_eq!(2, count_to_score(2));
        assert_eq!(8, count_to_score(4));
        assert_eq!(16, count_to_score(5));
        assert_eq!(32, count_to_score(6));

    }
}
