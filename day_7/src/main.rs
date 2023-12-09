use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");
const INPUT_SAMPLE: &str = include_str!("input_sample.txt");

#[derive(PartialEq, Eq, Copy, Clone)]
enum Task {
    Part1,
    Part2,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
enum Card {
    Joker,
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

const ALL_CARDS: &[Card; 14] = {
    use Card::*;
    &[
        Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
    ]
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn char_to_card(c: char, task: Task) -> Card {
    use Card::*;
    match c {
        '2' => Two,
        '3' => Three,
        '4' => Four,
        '5' => Five,
        '6' => Six,
        '7' => Seven,
        '8' => Eight,
        '9' => Nine,
        'T' => Ten,
        'J' if task == Task::Part1 => Jack,
        'J' if task == Task::Part2 => Joker,
        'Q' => Queen,
        'K' => King,
        'A' => Ace,
        _ => {
            panic!("Invalid card: {}", c);
        }
    }
}

fn check_hand(cards: &[Card; 5]) -> Hand {
    fn check_hand_inner(cards: &[Card; 5]) -> Hand {
        let mut counts = [0; 14];
        cards.iter().for_each(|&c| {
            counts[c as usize] += 1;
        });

        if counts.iter().any(|&c| c >= 5) {
            return Hand::FiveOfAKind;
        }

        if counts.iter().any(|&c| c == 4) {
            return Hand::FourOfAKind;
        }

        if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
            return Hand::FullHouse;
        }

        if counts.iter().any(|&c| c == 3) {
            return Hand::ThreeOfAKind;
        }

        if counts.iter().filter(|&&c| c == 2).count() == 2 {
            return Hand::TwoPair;
        }

        if counts.iter().any(|&c| c == 2) {
            return Hand::OnePair;
        }

        Hand::HighCard
    }

    let non_joker_cards: Vec<_> = cards.iter().filter(|c| **c != Card::Joker).collect();

    if non_joker_cards.len() == 5 {
        return check_hand_inner(cards);
    }

    let num_jokers = 5 - non_joker_cards.len();

    let mut cards = [Card::Ace; 5];
    non_joker_cards
        .iter()
        .enumerate()
        .for_each(|(i, c)| cards[i] = **c);

    ALL_CARDS
        .iter()
        .map(|c| {
            for i in 0..num_jokers {
                cards[non_joker_cards.len() + i] = *c;
            }

            check_hand_inner(&cards)
        })
        .max()
        .unwrap()
}

fn parse_input(input: &str, task: Task) -> Vec<([Card; 5], Hand, u64)> {
    let mut out = Vec::new();

    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .for_each(|l| {
            let (cards, bet) = l.split_once(" ").unwrap();
            let cards: [Card; 5] = cards
                .chars()
                .map(|c| char_to_card(c, task))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let bet: u64 = bet.parse().unwrap();
            let hand = check_hand(&cards);
            out.push((cards, hand, bet));
        });

    out
}

fn sort_cards(cards: &mut Vec<([Card; 5], Hand, u64)>) {
    cards.sort_by(|(cards1, hand1, _), (cards2, hand2, _)| {
        let hand_ord = hand1.cmp(hand2);
        if hand_ord != Ordering::Equal {
            return hand_ord;
        }

        cards1
            .iter()
            .zip(cards2)
            .find_map(|(c1, c2)| {
                let ord = c1.cmp(c2);
                if ord == Ordering::Equal {
                    return None;
                }
                Some(ord)
            })
            .unwrap_or(Ordering::Equal)
    });
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut data = parse_input(INPUT, Task::Part1);

    sort_cards(&mut data);

    let sum: u64 = data
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) as u64 * bid)
        .sum();

    println!("Winnings: {:?}", sum);
}

fn part_2() {
    let mut data = parse_input(INPUT, Task::Part2);

    sort_cards(&mut data);

    let sum: u64 = data
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) as u64 * bid)
        .sum();

    println!("Winnings with Joker: {:?}", sum);
}
