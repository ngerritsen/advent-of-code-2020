use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Deck = VecDeque<usize>;

#[derive(PartialEq, Eq)]
enum Player {
    One,
    Two,
}

fn main() {
    let (p1, p2) = parse_decks(include_str!("input.txt"));

    println!("{}", play_game(p1.clone(), p2.clone(), false).1);
    println!("{}", play_game(p1, p2, true).1);
}

fn play_game(mut p1: Deck, mut p2: Deck, recursive: bool) -> (Player, usize) {
    let mut history: HashSet<(Deck, Deck)> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if history.contains(&(p1.clone(), p2.clone())) {
            return (Player::One, 0);
        }

        history.insert((p1.clone(), p2.clone()));

        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();

        let mut won = Player::One;

        if recursive && p1.len() >= p1_card && p2.len() >= p2_card {
            let p1_next = p1.iter().take(p1_card).cloned().collect();
            let p2_next = p2.iter().take(p2_card).cloned().collect();

            won = play_game(p1_next, p2_next, true).0;
        } else if p2_card > p1_card {
            won = Player::Two;
        }

        if won == Player::One {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    }

    if !p1.is_empty() {
        (Player::One, calculate_score(p1))
    } else {
        (Player::Two, calculate_score(p2))
    }
}

fn calculate_score(cards: Deck) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

fn parse_decks(input: &str) -> (Deck, Deck) {
    input
        .split("\n\n")
        .map(|p| p.lines().skip(1).map(|l| l.parse().unwrap()).collect())
        .collect_tuple::<(Deck, Deck)>()
        .unwrap()
}
