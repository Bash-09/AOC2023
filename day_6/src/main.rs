struct Race {
    t: u64,
    d: u64,
}

const P1: &[Race] = &[
    Race { t: 35, d: 213 },
    Race { t: 69, d: 1168 },
    Race { t: 68, d: 1086 },
    Race { t: 87, d: 1248 },
];

const P2: Race = Race {
    t: 35696887,
    d: 213116810861248,
};

fn main() {
    part_1();
    part_2();
}

fn solve_race(race: &Race) -> u64 {
    // The equation for the distance in a race:
    //  x = time holding button
    //  t = race duration
    //  d = distance
    //  d = x(t - x)
    //  d = x*t - x*x
    //  -x^2 + x*t - d = 0

    // If we set d to the distance we need to beat, we can use the
    // quadratic formula to solve for the roots (or where we match the
    // distance to beat), then the range of times we can win with
    // is just the set of integers within those bounds.

    let t: f64 = race.t as f64;
    let d: f64 = race.d as f64;

    // Quadratic formula with t and d substituted in
    let root_disc = (t * t - 4.0 * d).sqrt();
    let min = (-t + root_disc) / -2.0;
    let max = (-t - root_disc) / -2.0;

    max as u64 - min as u64
}

fn part_1() {
    let result: u64 = P1.iter().map(solve_race).product();
    println!("Product of race times: {result}");
}

fn part_2() {
    let result = solve_race(&P2);
    println!("Big race time: {result}");
}
