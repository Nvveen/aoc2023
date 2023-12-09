use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn new(cards: &str) -> Self {
        let (hand, bid) = cards.split_once(" ").unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let hand = hand.chars().map(Card::from_char).collect::<Vec<_>>();
        Hand {
            cards: hand.try_into().unwrap(),
            bid,
        }
    }

    fn hand_type(&self) -> HandType {
        let mut set = HashSet::from(self.cards.clone());
        set.remove(&Card::Jack);
        let mut counts = set
            .iter()
            .map(|v| self.cards.iter().filter(|&c| c == v).count())
            .collect::<Vec<_>>();
        counts.sort();

        let missing = 5 - counts.iter().sum::<usize>();
        match counts.last_mut() {
            None => counts.push(5),
            Some(v) => *v += missing,
        }
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

pub fn day7_2(input: &str) -> u32 {
    let mut hands = input.lines().map(Hand::new).collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_day7_2() {
        assert_eq!(day7_2(INPUT), 5905);
    }
}
