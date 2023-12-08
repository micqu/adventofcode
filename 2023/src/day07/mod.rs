use itertools::Itertools;

use crate::utils::solution::{Solution, IntoSolution};

const INPUT: &'static str = include_str!("input.txt");
const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARDS2: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

pub fn part1() -> Option<Solution> {
    let mut hands = INPUT.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();

        let mut cards = [0; 5];
        for (i, ch) in hand.chars().enumerate() {
            cards[i] = CARDS.iter().position(|c| *c == ch).unwrap() as u32;
        }

        Hand { cards, bid, hand_type: calculate_type(cards) }
    }).collect_vec();
    
    hands.sort_by(|a, b| a.hand_type.cmp(&b.hand_type).then_with(|| a.cards.cmp(&b.cards)));
    hands.iter().rev().enumerate().map(|(i, h)| h.bid * (i + 1) as u32).sum::<u32>().solution()
}

pub fn part2() -> Option<Solution> {
    let mut hands = INPUT.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();

        let mut cards = [0; 5];
        for (i, ch) in hand.chars().enumerate() {
            cards[i] = CARDS2.iter().position(|c| *c == ch).unwrap() as u32;
        }

        Hand { cards, bid, hand_type: calculate_type_joker(cards) }
    }).collect_vec();

    hands.sort_by(|a, b| a.hand_type.cmp(&b.hand_type).then_with(|| a.cards.cmp(&b.cards)));
    hands.iter().rev().enumerate().map(|(i, x)| x.bid * (i + 1) as u32).sum::<u32>().solution()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u32; 5],
    bid: u32,
    hand_type: u32,
}

fn calculate_type(mut cards: [u32; 5]) -> u32 {
    cards.sort_unstable();
    let mut s = [0; 4];
    let mut l = cards[0];
    let mut i = 0;
    for c in cards[1..].iter() {
        if *c == l {
            s[i] += 1;
        } else {
            l = *c;
            i += 1;
        }
    }

    s.sort_unstable_by(|a, b| b.cmp(a));
    map_count_to_type(s[0], s[1])
}

fn calculate_type_joker(mut cards: [u32; 5]) -> u32 {
    cards.sort_unstable();
    let mut j = 0;
    let mut s = [0; 4];
    let mut l = cards[0];
    let mut i = 0;
    for c in cards[1..].iter() {
        if *c == 12 {
            j += 1;
        } else {
            if *c == l {
                s[i] += 1;
            } else {
                l = *c;
                i += 1;
            }
        }
    }

    s.sort_unstable_by(|a, b| b.cmp(a));
    
    if j > 0 {
        let d = j.min(4 - s[0]);
        s[0] += d;
        s[1] += j - d;
    }
    
    map_count_to_type(s[0], s[1])
}

fn map_count_to_type(a: u32, b: u32) -> u32 {
    match (a, b) {
        (4, 0) => 1,
        (3, 0) => 2,
        (2, 1) => 3,
        (2, 0) => 4,
        (1, 1) => 5,
        (1, 0) => 6,
        (0, 0) => 7,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 251121738),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 251421071),
            _ => panic!(),
        }
    }
}