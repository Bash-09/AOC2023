use std::{ops::Range, time::Instant};

const INPUT: &str = include_str!("input.txt");
const INPUT_SAMPLE: &str = include_str!("input_sample.txt");

fn main() {
    let start = Instant::now();
    part_1();
    let fin = Instant::now().duration_since(start);
    println!("Part 1: {}ms", fin.as_secs_f64() * 1000.0);

    println!("");

    let start = Instant::now();
    part_2();
    let fin = Instant::now().duration_since(start);
    println!("Part 2: {}ms", fin.as_secs_f64() * 1000.0);
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

fn part_1() {
    let data = parse_input(INPUT);

    let min = data
        .seeds
        .iter()
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
        .unwrap();

    println!("Min location: {}", min);
}

fn part_2() {
    let data = parse_input(INPUT);

    let mut source_ranges: Vec<Range<i64>> = Vec::new();
    let mut dest_ranges: Vec<Range<i64>> = Vec::new();

    for i in (0..data.seeds.len()).step_by(2) {
        source_ranges.push(data.seeds[i]..data.seeds[i] + data.seeds[i + 1]);
    }

    // Need to progress through each layer of maps
    data.maps.iter().for_each(|maps| {
        // Pop source ranges, process into dest ranges. If any source ranges get split up and only get partially
        // pushed into a dest range, push them back onto the source ranges so another iteration can take care of them.
        while let Some(source) = source_ranges.pop() {
            let got_mapped = maps
                .iter()
                .find(|&map| {
                    // Range has already been emptied - finish
                    if source.is_empty() {
                        return true;
                    }

                    // Source range entirely outside mapping range - ignore
                    if source.start >= map.range.end || source.end <= map.range.start {
                        return false;
                    }

                    // Source range entirely inside mapping range - map whole range
                    if source.start >= map.range.start && source.end <= map.range.end {
                        dest_ranges.push(source.start + map.offset..source.end + map.offset);
                        return true;
                    }

                    // Source range contains entire mapping range -
                    //   push starting range back to source ranges
                    //   push ending range back to source ranges
                    //   map middle section
                    if source.start <= map.range.start && source.end >= map.range.end {
                        source_ranges.push(source.start..map.range.start);
                        source_ranges.push(map.range.end..source.end);
                        dest_ranges.push(map.range.start + map.offset..map.range.end + map.offset);
                        return true;
                    }

                    // Source overlaps mapping start only -
                    //   push starting range back to source ranges
                    //   map remaining section
                    if source.start <= map.range.start && source.end <= map.range.end {
                        source_ranges.push(source.start..map.range.start);
                        dest_ranges.push(map.range.start + map.offset..source.end + map.offset);
                        return true;
                    }

                    // Source overlaps mapping end only -
                    //   map starting range
                    //   push remaining section back to source_ranges
                    if source.start >= map.range.start && source.end >= map.range.end {
                        dest_ranges.push(source.start + map.offset..map.range.end + map.offset);
                        source_ranges.push(map.range.end..source.end);
                        return true;
                    }

                    panic!("Missed a case!");
                })
                .is_some();

            if !got_mapped {
                dest_ranges.push(source);
            }
        }

        assert!(source_ranges.is_empty());
        std::mem::swap(&mut source_ranges, &mut dest_ranges);
    });

    let min: i64 = source_ranges
        .iter()
        .min_by_key(|r| {
            assert!(!r.is_empty());
            r.start
        })
        .unwrap()
        .start;

    println!("Min location of ranges: {}", min);
}

/*
/// Original part 2 solution which just parallelised and brute-forced all the possible seeds.
/// Took about 16.6gb of ram and all 24 of my cores att 100% to get it in about 15 seconds.
fn part_2() {
    use rayon::prelude::*;

    let mut data = parse_input(INPUT);

    let mut new_seeds = Vec::new();
    for i in (0..data.seeds.len()).step_by(2) {
        new_seeds.extend(data.seeds[i]..data.seeds[i] + data.seeds[i + 1]);
    }
    data.seeds = new_seeds;

    let min = data
        .seeds
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
        .unwrap();

    println!("Min location of ranges: {}", min);
}
*/
