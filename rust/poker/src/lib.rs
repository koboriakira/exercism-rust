use core::panic;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord)]
enum Pat {
    HighCard(Rank, Rank, Rank, Rank, Rank),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeCard(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank, Rank),
    FourCard(Rank, Rank),
    StraightFlush(Rank),
    RoyalStraightFlush,
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Ord)]
enum Rank {
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 14,
}

impl Rank {
    fn new(c: char) -> Self {
        match c {
            'A' => Rank::ACE,
            '2' => Rank::TWO,
            '3' => Rank::THREE,
            '4' => Rank::FOUR,
            '5' => Rank::FIVE,
            '6' => Rank::SIX,
            '7' => Rank::SEVEN,
            '8' => Rank::EIGHT,
            '9' => Rank::NINE,
            '1' => Rank::TEN,
            'J' => Rank::JACK,
            'Q' => Rank::QUEEN,
            'K' => Rank::KING,
            _ => panic!("Illegal char for rank"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

impl Suit {
    fn new(c: char) -> Self {
        match c {
            'H' => Suit::Heart,
            'S' => Suit::Spade,
            'D' => Suit::Diamond,
            'C' => Suit::Club,
            _ => panic!(),
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut pat_as_hand = hands.iter().fold(HashMap::new(), |mut map, hand| {
        let pat = analyze(hand);
        map.entry(pat).or_insert_with(|| vec![]).push(*hand);
        map
    });

    pat_as_hand
        .keys()
        .copied()
        .max()
        .and_then(|max| pat_as_hand.remove(&max))
}

fn analyze(hand: &str) -> Pat {
    let cards = hand.split(" ").collect::<Vec<&str>>();

    let suited = suits(&cards).len() == 1;
    let rank_as_count = count_rank(&ranks(&cards));
    let two_card = rank_as_count.get(&2);
    let three_card = rank_as_count.get(&3);
    let four_card = rank_as_count.get(&4);
    let kickers = match rank_as_count.get(&1) {
        Some(kickers) => kickers.clone(),
        None => vec![],
    };
    let straight = straight(&kickers);

    match (four_card, three_card, two_card, straight, suited) {
        (Some(four_card), _, _, _, _) => Pat::FourCard(four_card[0], kickers[0]),
        (None, Some(three_card), Some(two_card), _, _) => {
            Pat::FullHouse(three_card[0], two_card[0])
        }
        (None, Some(three_card), None, _, _) => {
            Pat::ThreeCard(three_card[0], kickers[0], kickers[1])
        }
        (None, None, Some(two_card), _, _) if two_card.len() == 2 => {
            Pat::TwoPair(two_card[0], two_card[1], kickers[0])
        }
        (None, None, Some(two_card), _, _) if two_card.len() == 1 => {
            Pat::OnePair(two_card[0], kickers[0], kickers[1], kickers[2])
        }
        (_, _, _, Some(Pat::Straight(n)), true) if n == Rank::ACE => Pat::RoyalStraightFlush,
        (_, _, _, Some(Pat::Straight(n)), true) => Pat::StraightFlush(n),
        (_, _, _, Some(straight), false) => straight,
        (_, _, _, None, true) => Pat::Flush(kickers[0]),
        _ => Pat::HighCard(kickers[0], kickers[1], kickers[2], kickers[3], kickers[4]),
    }
}

fn straight(ranks: &Vec<Rank>) -> Option<Pat> {
    if ranks.len() != 5 {
        return None;
    }

    let mut lowest = false;
    let result = !ranks.windows(2).any(|x| match x[0] as i32 - x[1] as i32 {
        // In the case 'A' and '5'
        9 => {
            lowest = true;
            false
        }
        1 => false,
        _ => true,
    });
    match (result, lowest) {
        (true, true) => Some(Pat::Straight(Rank::FIVE)),
        (true, false) => Some(Pat::Straight(ranks[0])),
        _ => None,
    }
}

fn count_rank(ranks: &Vec<Rank>) -> HashMap<u32, Vec<Rank>> {
    let init: HashMap<Rank, u32> = HashMap::new();
    let result_init: HashMap<u32, Vec<Rank>> = HashMap::new();

    ranks
        .into_iter()
        .fold(init, |mut acc, rank| {
            *acc.entry(*rank).or_insert_with(|| 0_u32) += 1_u32;
            acc
        })
        .iter()
        .fold(result_init, |mut acc, (rank, count)| {
            acc.entry(*count).or_insert_with(|| vec![]).push(*rank);
            acc
        })
        .iter()
        .map(|(k, v)| {
            (
                *k,
                v.into_iter()
                    .sorted()
                    .rev()
                    .map(|r| *r)
                    .collect::<Vec<Rank>>(),
            )
        })
        .collect()
}

fn ranks(cards: &Vec<&str>) -> Vec<Rank> {
    fn rank(card: &&str) -> Option<Rank> {
        match card.chars().nth(0) {
            Some(value) => Some(Rank::new(value)),
            _ => None,
        }
    }
    cards.into_iter().filter_map(rank).sorted().rev().collect()
}

fn suits(cards: &Vec<&str>) -> HashSet<Suit> {
    fn suit(card: &&str) -> Option<Suit> {
        match card.chars().nth(card.len() - 1) {
            Some(value) => Some(Suit::new(value)),
            _ => None,
        }
    }
    cards
        .into_iter()
        .filter_map(suit)
        .collect::<HashSet<Suit>>()
}
