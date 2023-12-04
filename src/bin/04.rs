use std::collections::HashMap;

use advent_of_code::day_4::{count_to_score, parse_scratch, process_card};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_scratch(input);

    let mut total = 0;
    for (id, card) in cards.iter() {
        let mut mtchs = 0;
        for h in card.hand.iter() {
            if card.win.contains(&h) {
                mtchs += 1
            }
        }
        total += count_to_score(mtchs);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_scratch(input);
    let mut cards_vec = Vec::new();
    for i in 1..=cards.clone().iter().count() {
        cards_vec.push(cards[&(i as u32)].clone());
    }

    let mut buffer = HashMap::new();
    let mut card_count = 0_u32;
    for card in cards_vec.iter() {
        card_count += 1;
        process_card(card, &mut buffer);
        if let Some(extra) = buffer.get(&card.id) {
            for _ in 0..*extra {
                process_card(card, &mut buffer)
            }
        }
    }
    let mut total = card_count;
    buffer.iter().for_each(|(id, buf)| {
        if id <= &card_count {
            total += buf
        }
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
