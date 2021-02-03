use core::panic;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let pat_as_hand = hands
        .iter()
        .map(|&hand| {
            let cards = hand.split(" ").collect::<Vec<&str>>();
            (analyze(cards), hand)
        })
        .fold(HashMap::new(), |mut map, (pat, hand)| {
            map.entry(pat).or_insert(vec![]).push(hand);
            map
        });
    println!("{:?}", pat_as_hand);
    let pats = pat_as_hand.keys().map(|x| *x).collect::<Vec<Pat>>();
    let strongest_pat = strongest(pats);
    println!("{:?}", strongest_pat);
    let result = pat_as_hand.get(&strongest_pat).unwrap().clone();

    Some(result)
}

fn strongest(pats: Vec<Pat>) -> Pat {
    pats.into_iter()
        .fold(Pat::INIT, |a, b| match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Greater) => a,
            Some(std::cmp::Ordering::Equal) => a,
            Some(std::cmp::Ordering::Less) => b,
            _ => panic!(""),
        })
}

fn analyze(cards: Vec<&str>) -> Pat {
    let rank_as_count = count_rank(&ranks(&cards));
    let two_card = rank_as_count.get(&2);
    let three_card = rank_as_count.get(&3);
    let four_card = rank_as_count.get(&4);
    let kickers = match rank_as_count.get(&1) {
        Some(kickers) => kickers.clone(),
        None => vec![],
    };
    let straight = straight(&kickers);
    let suited = suits(&cards).len() == 1;

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
        (_, _, _, Some(Pat::Straight(n)), true) => Pat::StraightFlush(n),
        (_, _, _, Some(Pat::LowestStraight), true) => Pat::LowestStraighFlush,
        (_, _, _, Some(straight), false) => straight,
        (_, _, _, None, true) => Pat::Flush(kickers[0]),
        _ => Pat::HighCard(kickers[0], kickers[1], kickers[2], kickers[3], kickers[4]),
    }
}

fn straight(ranks: &Vec<Rank>) -> Option<Pat> {
    match ranks.len() != 5 {
        true => None,
        _ => {
            let mut lowest = false;
            let result =
                !ranks
                    .windows(2)
                    .map(|x| (x[0] as i32, x[1] as i32))
                    .any(|(a, b)| match a - b {
                        // In the case 'A' and '5'
                        9 => {
                            lowest = true;
                            false
                        }
                        1 => false,
                        _ => true,
                    });
            match (result, lowest) {
                (false, _) => None,
                (_, true) => Some(Pat::LowestStraight),
                (_, false) => Some(Pat::Straight(ranks[0])),
            }
        }
    }
}

fn count_rank(ranks: &Vec<Rank>) -> HashMap<u32, Vec<Rank>> {
    let init: HashMap<Rank, u32> = HashMap::new();
    let result_init: HashMap<u32, Vec<Rank>> = HashMap::new();

    ranks
        .into_iter()
        .fold(init, |mut acc, rank| {
            *acc.entry(*rank).or_insert(0_u32) += 1_u32;
            acc
        })
        .iter()
        .fold(result_init, |mut acc, (rank, count)| {
            acc.entry(*count).or_insert(vec![]).push(*rank);
            acc
        })
        .iter()
        .map(|(k, v)| (*k, v.iter().sorted().rev().cloned().collect::<Vec<Rank>>()))
        .collect()
}

fn ranks(cards: &Vec<&str>) -> Vec<Rank> {
    fn rank(card: &&str) -> Rank {
        Rank::new(card.chars().nth(0).unwrap())
    }
    cards.into_iter().map(rank).sorted().rev().collect()
}

fn suits(cards: &Vec<&str>) -> HashSet<char> {
    fn suit(card: &&str) -> char {
        card.chars().nth(card.len() - 1).unwrap()
    }
    cards.into_iter().map(suit).collect::<HashSet<char>>()
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
enum Pat {
    INIT,
    HighCard(Rank, Rank, Rank, Rank, Rank),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeCard(Rank, Rank, Rank),
    LowestStraight,
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank, Rank),
    FourCard(Rank, Rank),
    LowestStraighFlush,
    StraightFlush(Rank),
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
