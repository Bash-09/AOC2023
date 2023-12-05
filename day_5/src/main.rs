use std::ops::Range;

const INPUT: &str = include_str!("input.txt");
const INPUT_SAMPLE: &str = include_str!("input_sample.txt");

fn main() {
    part_1();
    part_2();
}

#[derive(Debug)]
struct Data {
    pub seeds: Vec<i64>,
    pub maps: Vec<Vec<AlmanacMap>>,
}

#[derive(Debug)]
struct AlmanacMap {
    pub range: Range<i64>,
    pub offset: i64,
}

fn parse_map(line: &str) -> Option<AlmanacMap> {
    let mut vals = line.split(" ");
    let dst_start: i64 = vals.next()?.parse().ok()?;
    let src_start: i64 = vals.next()?.parse().ok()?;
    let range: i64 = vals.next()?.parse().ok()?;

    Some(AlmanacMap {
        range: src_start..src_start + range,
        offset: dst_start - src_start,
    })
}

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .map(|(_, seed_vals)| seed_vals.split_whitespace().map(|v| v.parse().unwrap()))
        .unwrap()
        .collect();

    let mut maps: Vec<Vec<AlmanacMap>> = Vec::new();
    let mut cur_map: Vec<AlmanacMap> = Vec::new();

    lines.filter(|l| !l.trim().is_empty()).for_each(|l| {
        if l.contains(':') {
            if !cur_map.is_empty() {
                let mut new_map = Vec::new();
                std::mem::swap(&mut new_map, &mut cur_map);
                maps.push(new_map);
            }
        } else {
            cur_map.push(parse_map(l).unwrap());
        }
    });

    if !cur_map.is_empty() {
        maps.push(cur_map);
    }

    Data { seeds, maps }
}

fn find_min(data: &Data) -> i64 {
    use rayon::prelude::*;

    data.seeds
        .par_iter()
        .map(|s| {
            data.maps.iter().fold(*s, |s, maps| {
                let offset = maps
                    .iter()
                    .find_map(|m| {
                        if m.range.contains(&s) {
                            Some(m.offset)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);
                s + offset
            })
        })
        .min()
        .unwrap()
}

fn part_1() {
    let data = parse_input(INPUT);

    let min = find_min(&data);

    println!("Min location: {}", min);
}

fn part_2() {
    let mut data = parse_input(INPUT);

    let mut new_seeds = Vec::new();
    for i in (0..data.seeds.len()).step_by(2) {
        new_seeds.extend(data.seeds[i]..data.seeds[i] + data.seeds[i + 1]);
    }
    data.seeds = new_seeds;

    let min = find_min(&data);

    println!("Min location of ranges: {}", min);
}
