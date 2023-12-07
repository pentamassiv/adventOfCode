use std::{cmp::Ordering, path::Path};

pub fn run<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path> + std::fmt::Display,
{
    // Annoying helper to be able to run the tests in regular and debug mode (see https://github.com/rust-lang/rust-analyzer/issues/13208)
    let input = if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        std::fs::read_to_string(format!("aoc23/{path}")).unwrap()
    };
    // Part 1
    let mut hands: Vec<_> = input.lines().map(|l| Hand::from((l, false))).collect();
    hands.sort_unstable_by(|a, b| a.cmp(b, false));
    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bet())
        .sum();

    // Part 2
    hands = input.lines().map(|l| Hand::from((l, true))).collect();
    hands.sort_unstable_by(|a, b| a.cmp(b, true));
    let part2 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bet())
        .sum();

    (part1, part2)
}

#[derive(Debug)]
enum Hand<'a> {
    HighCard(&'a str, usize),
    Pair(&'a str, usize),
    TwoPair(&'a str, usize),
    Three(&'a str, usize),
    FullHouse(&'a str, usize),
    Four(&'a str, usize),
    Five(&'a str, usize),
}

impl<'a> Hand<'a> {
    fn bet(&self) -> usize {
        match self {
            Self::HighCard(_, v)
            | Self::Pair(_, v)
            | Self::TwoPair(_, v)
            | Self::Three(_, v)
            | Self::FullHouse(_, v)
            | Self::Four(_, v)
            | Self::Five(_, v) => *v,
        }
    }

    fn cmp(&self, other: &Self, joker_rules: bool) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Five(a, _), Self::Five(b, _))
            | (Self::Four(a, _), Self::Four(b, _))
            | (Self::FullHouse(a, _), Self::FullHouse(b, _))
            | (Self::Three(a, _), Self::Three(b, _))
            | (Self::TwoPair(a, _), Self::TwoPair(b, _))
            | (Self::Pair(a, _), Self::Pair(b, _))
            | (Self::HighCard(a, _), Self::HighCard(b, _)) => cmp_cards(a, b, joker_rules),

            (Self::Five(_, _), _) => Ordering::Greater,
            (_, Self::Five(_, _)) => Ordering::Less,

            (Self::Four(_, _), _) => Ordering::Greater,
            (_, Self::Four(_, _)) => Ordering::Less,

            (Self::FullHouse(_, _), _) => Ordering::Greater,
            (_, Self::FullHouse(_, _)) => Ordering::Less,

            (Self::Three(_, _), _) => Ordering::Greater,
            (_, Self::Three(_, _)) => Ordering::Less,

            (Self::TwoPair(_, _), _) => Ordering::Greater,
            (_, Self::TwoPair(_, _)) => Ordering::Less,

            (Self::Pair(_, _), _) => Ordering::Greater,
            (_, Self::Pair(_, _)) => Ordering::Less,
        }
    }
}

impl<'a> From<(&'a str, bool)> for Hand<'a> {
    fn from(line_and_rules: (&'a str, bool)) -> Self {
        let (line, joker_rules) = line_and_rules;
        let hand = &line[..5];
        let bet = line[6..].parse().unwrap();

        let mut n = [0; 13];
        for c in hand.chars() {
            match c {
                'A' => {
                    n[12] += 1;
                }
                'K' => {
                    n[11] += 1;
                }
                'Q' => {
                    n[10] += 1;
                }
                'J' => {
                    n[9] += 1;
                }
                'T' => {
                    n[8] += 1;
                }
                '9' => {
                    n[7] += 1;
                }
                '8' => {
                    n[6] += 1;
                }
                '7' => {
                    n[5] += 1;
                }
                '6' => {
                    n[4] += 1;
                }
                '5' => {
                    n[3] += 1;
                }
                '4' => {
                    n[2] += 1;
                }
                '3' => {
                    n[1] += 1;
                }
                '2' => {
                    n[0] += 1;
                }
                _ => panic!("not a valid card"),
            }
        }
        let no_jokers = if joker_rules {
            let temp_no_jokers = n[9];
            n[9] = 0;
            temp_no_jokers
        } else {
            0
        };
        n.sort_unstable();
        let no_same_cards = n[12];
        match no_same_cards + no_jokers {
            5 => Self::Five(hand, bet),
            4 => Self::Four(hand, bet),
            3 => {
                if n[11] == 2 {
                    Self::FullHouse(hand, bet)
                } else {
                    Self::Three(hand, bet)
                }
            }
            2 => {
                if n[11] == 2 {
                    Self::TwoPair(hand, bet)
                } else {
                    Self::Pair(hand, bet)
                }
            }
            1 => Self::HighCard(hand, bet),
            _ => unreachable!(),
        }
    }
}

fn cmp_cards(a: &str, b: &str, joker_rules: bool) -> Ordering {
    for (a, b) in a.chars().zip(b.chars()) {
        match (a, b) {
            ('A', 'A') | ('K', 'K') | ('Q', 'Q') | ('J', 'J') | ('T', 'T') => (),
            ('A', _) => return Ordering::Greater,
            (_, 'A') => return Ordering::Less,

            ('K', _) => return Ordering::Greater,
            (_, 'K') => return Ordering::Less,

            ('Q', _) => return Ordering::Greater,
            (_, 'Q') => return Ordering::Less,

            ('J', _) => {
                if joker_rules {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            }
            (_, 'J') => {
                if joker_rules {
                    return Ordering::Greater;
                }
                return Ordering::Less;
            }

            ('T', _) => return Ordering::Greater,
            (_, 'T') => return Ordering::Less,

            (a, b) => match a.cmp(&b) {
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            },
        }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let hand1 = Hand::Pair("32T3K", 765);
        let hand2 = Hand::TwoPair("KTJJT", 220);
        let hand3 = Hand::TwoPair("KK677", 28);
        let hand4 = Hand::Three("T55J5", 684);
        let hand5 = Hand::Three("QQQJA", 483);
        // Hand 1
        assert_eq!(Ordering::Less, hand1.cmp(&hand2, false));
        assert_eq!(Ordering::Less, hand1.cmp(&hand3, false));
        assert_eq!(Ordering::Less, hand1.cmp(&hand4, false));
        assert_eq!(Ordering::Less, hand1.cmp(&hand5, false));

        // Hand 2
        assert_eq!(Ordering::Less, hand2.cmp(&hand3, false));
        assert_eq!(Ordering::Less, hand2.cmp(&hand4, false));
        assert_eq!(Ordering::Less, hand2.cmp(&hand5, false));

        // Hand 3
        assert_eq!(Ordering::Less, hand3.cmp(&hand4, false));
        assert_eq!(Ordering::Less, hand3.cmp(&hand5, false));

        // Hand 4
        assert_eq!(Ordering::Less, hand4.cmp(&hand5, false));
    }

    #[test]
    fn test_example() {
        let (part1, part2) = run("input/examples/7/1.txt");
        assert_eq!(part1, 6_440);
        assert_eq!(part2, 5_905);
    }

    #[test]
    fn test_input() {
        let (part1, part2) = run("input/7.txt");
        assert_eq!(part1, 246_912_307);
        assert_eq!(part2, 246_894_760);
    }
}
