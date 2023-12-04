fn main() {
    part_1();
    part_2();
}

const INPUT: &str = include_str!("input.txt");

fn wins_in_card(input: &str) -> usize {
    let (_, nums) = input.split_once(":").unwrap();
    let (winning, yours) = nums.split_once("|").unwrap();
    let winning: Vec<_> = winning
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    yours
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .filter(|n| winning.contains(n))
        .count()
}

fn part_1() {
    let total: u64 = INPUT
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| wins_in_card(l))
        .map(|wins| wins.checked_sub(1).map(|p| 2u64.pow(p as _)).unwrap_or(0))
        .sum();

    println!("Total points: {total}");
}

fn part_2() {
    let cards: Vec<usize> = INPUT
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(wins_in_card)
        .collect();

    let mut count: Vec<u64> = vec![1; cards.len()];

    cards.iter().enumerate().for_each(|(i, wins)| {
        let num_cards = count[i];
        for j in i + 1..i + wins + 1 {
            count.get_mut(j).map(|n| *n += num_cards);
        }
    });

    println!("Total cards: {}", count.iter().sum::<u64>());
}
