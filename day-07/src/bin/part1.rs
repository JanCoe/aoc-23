use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(card: char) -> Card {
        match card {
            '1' => Card::One,
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

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else {
            for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                if *card1 == *card2 {
                    continue;
                } else if *card1 < *card2 {
                    return Ordering::Less;
                } else if *card1 > *card2 {
                    return Ordering::Greater;
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Hand {
    fn new(line: &str) -> Hand {
        let mut line = line.split(' ');
        let cards = line.next().expect("cards exist");
        let bid: usize = line.next().expect("bid exists").parse().unwrap();

        let cards: Vec<Card> = cards.chars().map(|c| Card::new(c)).collect();
        let temp_hand: Hand = Hand {
            cards: cards.clone(),
            hand_type: HandType::HighCard,
            bid,
        };
        let hand_type = temp_hand.get_hand_type();
        Hand {
            cards,
            hand_type,
            bid,
        }
    }

    fn get_hand_type(&self) -> HandType {
        let num_by_card = self.cards.iter().fold(HashMap::new(), |mut acc, card| {
            let count = acc.entry(card).or_insert(0);
            *count += 1;
            acc
        });

        let card_stats: Vec<_> = num_by_card.values().map(|&x| x).collect();

        if card_stats.contains(&5) {
            HandType::FiveOfAKind
        } else if card_stats.contains(&4) {
            HandType::FourOfAKind
        } else if card_stats.contains(&3) && card_stats.contains(&2) {
            HandType::FullHouse
        } else if card_stats.contains(&3) {
            HandType::ThreeOfAKind
        } else if card_stats.contains(&2) && card_stats.len() == 3 {
            HandType::TwoPair
        } else if card_stats.contains(&2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let data = include_str!("../../../input/day-07.txt");
    let mut hands: Vec<Hand> = data.lines().map(|line| Hand::new(line)).collect();

    hands.sort_unstable();
    let winnings: usize = hands
        .iter()
        .enumerate()
        .map(|(ctr, hand)| (ctr + 1) * hand.bid)
        .sum();
    println!("winnings: {}", winnings);
}

#[cfg(test)]
mod tests {
    use super::{Hand, HandType};

    const DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_is_stronger() {
        let mut data = DATA.lines();

        let _hand1 = Hand::new(data.next().unwrap());
        let _hand2 = Hand::new(data.next().unwrap());
        let hand3 = Hand::new(data.next().unwrap());
        let hand4 = Hand::new(data.next().unwrap());
        assert!(hand3 > hand4);
    }

    #[test]
    fn test_hand_type() {
        let mut data = DATA.lines();

        assert_eq!(Hand::new(data.next().unwrap()).hand_type, HandType::OnePair);
        assert_eq!(
            Hand::new(data.next().unwrap()).hand_type,
            HandType::ThreeOfAKind
        );
        assert_eq!(Hand::new(data.next().unwrap()).hand_type, HandType::TwoPair);
        assert_eq!(Hand::new(data.next().unwrap()).hand_type, HandType::TwoPair);
        assert_eq!(
            Hand::new(data.next().unwrap()).hand_type,
            HandType::ThreeOfAKind
        );
    }
}
