use crate::utils;
use std::path::Path;
use std::collections::HashMap;

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn camel_cards_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut winnings = 0;
    let sorted_game_hands = sort_game_hands(&lines);
    for (i, hand) in sorted_game_hands.iter().enumerate() {
        let hand_split = hand.split(" ").collect::<Vec<&str>>();
        let bid = hand_split[1].parse::<u32>().unwrap();
        winnings += bid * (i as u32 + 1);
    }

    println!("Camel cards part one: {}", winnings);
}

pub fn camel_cards_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut winnings = 0;
    let sorted_game_hands = sort_game_hands_joker(&lines);
    for (i, hand) in sorted_game_hands.iter().enumerate() {
        let hand_split = hand.split(" ").collect::<Vec<&str>>();
        let bid = hand_split[1].parse::<u32>().unwrap();
        winnings += bid * (i as u32 + 1);
    }

    println!("Camel cards part two: {}", winnings);
}

fn get_distinct_cards(hand: &str) -> HashMap<char, u8> {
    let mut cards: HashMap<char, u8> = HashMap::new();
    for c in hand.chars() {
        if cards.contains_key(&c) {
            cards.insert(c, cards.get(&c).unwrap() + 1);
        } else {
            cards.insert(c, 1);
        }
    }
    cards.into_iter().collect()
}

fn sort_game_hands(game_hands: &Vec<String>) -> Vec<String> {
    let mut sorted_hands = game_hands.clone();
    sorted_hands.sort_by(|a, b| hand_cmp(a, b));
    sorted_hands
}

fn sort_game_hands_joker(game_hands: &Vec<String>) -> Vec<String> {
    let mut sorted_hands = game_hands.clone();
    sorted_hands.sort_by(|a, b| joker_hand_cmp(a, b));
    sorted_hands
}

fn hand_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let a_rank = hand_rank(&a.split(" ").collect::<Vec<&str>>()[0]);
    let b_rank = hand_rank(&b.split(" ").collect::<Vec<&str>>()[0]);
    if a_rank == b_rank {
        for (a_card, b_card) in a.chars().zip(b.chars()) {
            let a_value = card_value(a_card);
            let b_value = card_value(b_card);
            if a_value != b_value {
                return a_value.cmp(&b_value);
            }
        }
        return std::cmp::Ordering::Equal;
    } else {
        return a_rank.cmp(&b_rank);
    }
}

fn joker_hand_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let a_rank = joker_hand_rank(&a.split(" ").collect::<Vec<&str>>()[0]);
    let b_rank = joker_hand_rank(&b.split(" ").collect::<Vec<&str>>()[0]);
    if a_rank == b_rank {
        for (a_card, b_card) in a.chars().zip(b.chars()) {
            let a_value = joker_card_value(a_card);
            let b_value = joker_card_value(b_card);
            if a_value != b_value {
                return a_value.cmp(&b_value);
            }
        }
        return std::cmp::Ordering::Equal;
    } else {
        return a_rank.cmp(&b_rank);
    }
}

fn card_value(c: char) -> i8 {
    match c {
        'A' => 14,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        _ => c.to_digit(10).unwrap() as i8,
    }
}

fn joker_card_value(c: char) -> i8 {
    match c {
        'A' => 13,
        'T' => 10,
        'J' => 1,
        'Q' => 11,
        'K' => 12,
        _ => c.to_digit(10).unwrap() as i8,
    }

}

fn hand_type_value(hand_type: HandType) -> u8 {
    match hand_type {
        HandType::FiveOfAKind => 7,
        HandType::FourOfAKind => 6,
        HandType::FullHouse => 5,
        HandType::ThreeOfAKind => 4,
        HandType::TwoPair => 3,
        HandType::OnePair => 2,
        HandType::HighCard => 1,
    }
}

fn hand_rank(hand: &str) -> u8 {
    let distinct_cards = get_distinct_cards(hand);
    match distinct_cards.len() {
        1 => hand_type_value(HandType::FiveOfAKind),
        2 => {
            let mut values = distinct_cards.values().collect::<Vec<&u8>>();
            values.sort();
            if *values[0] == 1 {
                hand_type_value(HandType::FourOfAKind)
            } else {
                hand_type_value(HandType::FullHouse)
            }
        },
        3 => {
            let mut values = distinct_cards.values().collect::<Vec<&u8>>();
            values.sort();
            if *values[2] == 3 {
                hand_type_value(HandType::ThreeOfAKind)
            } else {
                hand_type_value(HandType::TwoPair)
            }
        },
        4 => hand_type_value(HandType::OnePair),
        _ => hand_type_value(HandType::HighCard)
    }

}

fn joker_hand_rank(hand: &str) -> u8 {
    let distinct_cards = get_distinct_cards(hand);
    match distinct_cards.len() {
        1 => hand_type_value(HandType::FiveOfAKind),
        2 => {
            if distinct_cards.contains_key(&'J') {
                hand_type_value(HandType::FiveOfAKind)
            } else {
                let mut values = distinct_cards.values().collect::<Vec<&u8>>();
                values.sort();
                if *values[0] == 1 {
                    hand_type_value(HandType::FourOfAKind)
                } else {
                    hand_type_value(HandType::FullHouse)
                }
            }
        },
        3 => {
            if distinct_cards.contains_key(&'J') {
                let joker_value = *distinct_cards.get(&'J').unwrap();
                if joker_value == 1 {
                    let mut values = distinct_cards.values().collect::<Vec<&u8>>();
                    values.sort();
                    if *values[2] == 3 {
                        hand_type_value(HandType::FourOfAKind)
                    } else {
                        hand_type_value(HandType::FullHouse)
                    }
                } else {
                    hand_type_value(HandType::FourOfAKind)
                }
            } else {
                let mut values = distinct_cards.values().collect::<Vec<&u8>>();
                values.sort();
                if *values[2] == 3 {
                    hand_type_value(HandType::ThreeOfAKind)
                } else {
                    hand_type_value(HandType::TwoPair)
                }
            }
        },
        4 => {
            if distinct_cards.contains_key(&'J') {
                hand_type_value(HandType::ThreeOfAKind)
            } else {
                hand_type_value(HandType::OnePair)
            }
        },
        _ => {
            if distinct_cards.contains_key(&'J') {
                hand_type_value(HandType::OnePair)
            } else {
                hand_type_value(HandType::HighCard)
            }
        }
    }
}