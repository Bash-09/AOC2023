const INPUT: &str = include_str!("input.txt");

fn main() {
    part_1();
    part_2();
}

struct Symbol {
    x: usize,
    y: usize,
    value: char,
}

struct Num {
    x: usize,
    y: usize,
    len: usize,
    value: u32,
}

fn parse_input(input: &str) -> (Vec<Symbol>, Vec<Num>) {
    let mut numbers: Vec<Num> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    input.lines().enumerate().for_each(|(y, l)| {
        let mut current_num: Option<Num> = None;
        l.chars().enumerate().for_each(|(x, c)| {
            if let Some(d) = c.to_digit(10) {
                if let Some(num) = &mut current_num {
                    num.value = num.value * 10 + d;
                    num.len += 1;
                } else {
                    current_num = Some(Num {
                        x,
                        y,
                        len: 1,
                        value: d,
                    });
                }
            } else {
                if current_num.is_some() {
                    numbers.push(current_num.take().unwrap());
                }

                if c != '.' {
                    symbols.push(Symbol { x, y, value: c });
                }
            }
        });

        if let Some(num) = current_num {
            numbers.push(num);
        }
    });

    (symbols, numbers)
}

fn adjacent(s: &Symbol, n: &Num) -> bool {
    s.x >= n.x.saturating_sub(1)
        && s.x <= n.x + n.len
        && s.y >= n.y.saturating_sub(1)
        && s.y <= n.y + 1
}

fn part_1() {
    let (symbols, numbers) = parse_input(INPUT);

    let sum: u32 = numbers
        .into_iter()
        .filter(|n| symbols.iter().any(|s| adjacent(s, n)))
        .map(|thing| thing.value)
        .sum();

    println!("Sum of part numbers: {}", sum);
}

fn part_2() {
    let (symbols, numbers) = parse_input(INPUT);

    let sum: u32 = symbols
        .iter()
        .filter(|&s| s.value == '*')
        .map(|s| {
            numbers
                .iter()
                .filter(|&n| adjacent(s, n))
                .collect::<Vec<_>>()
        })
        .filter(|nums| nums.len() == 2)
        .map(|pair| pair[0].value * pair[1].value)
        .sum();

    println!("Sum of gear ratios: {}", sum);
}
