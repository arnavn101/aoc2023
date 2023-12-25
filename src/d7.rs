use std::cmp::Ordering;
use std::collections::HashMap;

fn parse_lines(lines: &Vec<String>, consider_joker: bool) -> Vec<(String, usize)> {
    let mut hands: Vec<(String, usize)> = Vec::new();

    lines.iter().for_each(|line| {
        let mut split = line.split_whitespace();
        let mut hand = split.next().unwrap().to_string();

        if consider_joker {
            hand = hand
                .chars()
                .map(|x| if x == 'J' { 'X' } else { x })
                .collect();
        }

        let bid = split.next().unwrap().parse::<usize>().unwrap();
        hands.push((hand, bid));
    });

    hands
}

fn sort_hands_key(hand1: &String, hand2: &String) -> Ordering {
    fn all_pos_cards() -> Vec<char> {
        vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            'X', // X for Joker
        ]
    }

    fn get_card_strength(card: char) -> usize {
        all_pos_cards()
            .iter()
            .rev()
            .position(|&c| c == card)
            .unwrap()
    }

    fn get_hand_type(hand: &String) -> usize {
        let mut card_ht: HashMap<char, usize> = hand.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let joker_count = card_ht.get(&'X').unwrap_or(&0).clone();
        card_ht.remove(&'X');
        let mx_count_before_joker = card_ht.values().max().unwrap_or(&0).clone();

        if mx_count_before_joker == 0 {
            return 6; // all jokers
        }

        if joker_count > 0 {
            for (_, count) in card_ht.iter_mut() {
                if *count == mx_count_before_joker {
                    *count += joker_count;
                    break;
                }
            }
        }

        let card_counts: Vec<usize> = card_ht.values().cloned().collect();
        let mx_count = card_counts.iter().max().unwrap().clone();

        if mx_count >= 4 {
            return mx_count + 1; // 5 for 4 of a kind, 6 for 5 of a kind
        } else if card_counts.contains(&3) && card_counts.contains(&2) {
            return 4; // 4 for full house
        } else if mx_count == 3 {
            return mx_count; // 3 for 3 of a kind
        } else if card_counts.iter().filter(|&c| *c == 2).count() == 2 {
            return 2; // 2 for two pair
        } else {
            return mx_count - 1; // 0 for high card, 1 for pair
        }
    }

    let hand_ordering = get_hand_type(hand1).cmp(&get_hand_type(hand2));
    if !hand_ordering.is_eq() {
        return hand_ordering;
    }

    for (card1, card2) in hand1.chars().zip(hand2.chars()) {
        let card_ordering = get_card_strength(card1).cmp(&get_card_strength(card2));
        if !card_ordering.is_eq() {
            return card_ordering;
        }
    }

    Ordering::Equal
}

pub fn sort_hands(hands: &Vec<(String, usize)>) -> Vec<(String, usize)> {
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_unstable_by(|hand1, hand2| sort_hands_key(&hand1.0, &hand2.0));
    sorted_hands
}

pub fn get_total_winnings(sorted_hands: &Vec<(String, usize)>) -> usize {
    let mut idx = 0;
    sorted_hands.iter().fold(0, |acc, hand| {
        idx += 1;
        acc + (hand.1 * idx)
    })
}

pub fn p1(lines: &Vec<String>) -> usize {
    let hands = parse_lines(lines, false);
    let sorted_hands = sort_hands(&hands);
    get_total_winnings(&sorted_hands)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let hands = parse_lines(lines, true);
    let sorted_hands = sort_hands(&hands);
    get_total_winnings(&sorted_hands)
}
