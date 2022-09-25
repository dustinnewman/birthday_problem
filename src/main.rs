use std::collections::HashSet;
use std::env::args;
use rand::Rng;

const DEFAULT_N: u32 = 23;
const TRIAL_COUNT: u32 = 1000;
const SET_SIZE: u32 = 365;

fn is_random_collision(n: u32) -> bool {
    let mut set = HashSet::new();
    let mut rng = rand::thread_rng();
    let mut dup = false;
    for _ in 0..n {
        let day = rng.gen_range(0..SET_SIZE);
        if set.contains(&day) {
            dup = true;
            break;
        }
        set.insert(day);
    }
    dup
}

fn birthday_problem(n: u32) -> f32 {
    let mut count = 0;
    for _ in 0..TRIAL_COUNT {
        if is_random_collision(n) {
            count += 1;
        }
    }
    count as f32 / TRIAL_COUNT as f32
}

fn main() {
    let n = args().nth(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(DEFAULT_N);
    let p = birthday_problem(n);
    println!("Probability for n = {:?} = {:?}%", n, p * 100.0);
}
