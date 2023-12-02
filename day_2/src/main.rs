use std::ops::{Index, IndexMut};

const INPUT: &str = include_str!("input.txt");

#[derive(Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Index<&str> for Set {
    type Output = u32;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "red" => &self.red,
            "green" => &self.green,
            "blue" => &self.blue,
            _ => panic!("Index {} not valid for colour set.", index),
        }
    }
}

impl IndexMut<&str> for Set {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => panic!("Index {} not valid for colour set.", index),
        }
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let allowed = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum_of_ids = 0;

    for l in INPUT.lines() {
        let game_id: u32 = l
            .split(':')
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        if l.split(':')
            .nth(1)
            .unwrap()
            .split(';')
            .map(|set| {
                set.split(',').fold(Set::default(), |mut cum_set, group| {
                    let mut iter = group.trim().split(' ');
                    let num: u32 = iter.next().unwrap().parse().unwrap();
                    let col = iter.next().unwrap();

                    cum_set[col] += num;
                    cum_set
                })
            })
            .all(|set| {
                set["red"] <= allowed["red"]
                    && set["green"] <= allowed["green"]
                    && set["blue"] <= allowed["blue"]
            })
        {
            sum_of_ids += game_id;
        }
    }

    println!("Sum of game ids: {}", sum_of_ids);
}

fn part_2() {
    let mut sum_of_powers: u32 = 0;

    for l in INPUT.lines() {
        let mut min_req = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        l.split(':')
            .nth(1)
            .unwrap()
            .split(';')
            .map(|set| {
                set.split(',').fold(Set::default(), |mut cum_set, group| {
                    let mut iter = group.trim().split(' ');
                    let num: u32 = iter.next().unwrap().parse().unwrap();
                    let col = iter.next().unwrap();

                    cum_set[col] += num;
                    cum_set
                })
            })
            .for_each(|set| {
                min_req["red"] = min_req["red"].max(set["red"]);
                min_req["green"] = min_req["green"].max(set["green"]);
                min_req["blue"] = min_req["blue"].max(set["blue"]);
            });

        sum_of_powers += min_req["red"] * min_req["green"] * min_req["blue"];
    }

    println!("Sum of minimum powers: {}", sum_of_powers);
}
