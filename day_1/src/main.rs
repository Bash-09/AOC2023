const INPUT: &str = include_str!("input.txt");

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let sum: u32 = INPUT.lines().fold(0, |sum, line| {
        sum + {
            let mut digs = line.chars().filter_map(|c| c.to_digit(10));
            let first = digs.next().unwrap_or(0);
            let last = digs.last().unwrap_or(first);
            first * 10 + last
        }
    });

    println!("Part 1: {}", sum);
}

fn part_2() {
    let sum: u32 = INPUT.lines().fold(0, |sum, line| {
        sum + {
            let mut digs = NumIter { src: line };
            let first = digs.next().unwrap_or(0);
            let last = digs.last().unwrap_or(first);
            first * 10 + last
        }
    });

    println!("Part 2: {}", sum);
}

struct NumIter<'a> {
    src: &'a str,
}

impl<'a> Iterator for NumIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.src.is_empty() {
            if let Some(n) = self.src.chars().next().and_then(|c| c.to_digit(10)) {
                self.src = &self.src[1..];
                return Some(n);
            }

            const NUMS: [&str; 10] = [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            for (n, &str) in NUMS.iter().enumerate() {
                if self.src.starts_with(str) {
                    self.src = &self.src[1..];
                    return Some(n as u32);
                }
            }

            self.src = &self.src[1..];
        }

        None
    }
}

#[test]
fn num_iter() {
    fn nums(str: &str) -> Vec<u32> {
        NumIter { src: str }.collect()
    }

    assert_eq!(nums("two1nine"), vec![2, 1, 9]);
    assert_eq!(nums("eightwothree"), vec![8, 2, 3]);
    assert_eq!(nums("abcone2threexyz"), vec![1, 2, 3]);
    assert_eq!(nums("xtwone3four"), vec![2, 1, 3, 4]);
    assert_eq!(nums("4nineeightseven2"), vec![4, 9, 8, 7, 2]);
    assert_eq!(nums("zoneight234"), vec![1, 8, 2, 3, 4]);
    assert_eq!(nums("7pqrstsixteen"), vec![7, 6]);
}
