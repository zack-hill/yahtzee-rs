use rand::Rng;
use std::cmp;
use std::collections::HashMap;

pub type DieSet = [u8; 5];

pub fn roll_die() -> u8 {
    rand::thread_rng().gen_range(1, 7)
}

pub fn roll_dice() -> DieSet {
    [roll_die(), roll_die(), roll_die(), roll_die(), roll_die()]
}

pub fn count_dice(dice: &DieSet, num: u8) -> u32 {
    let mut count = 0;
    for &die in dice {
        if die == num {
            count += 1;
        }
    }
    count
}

pub fn sum_dice(dice: &DieSet) -> u32 {
    dice.iter().sum::<u8>() as u32
}

pub fn calc_mode(dice: &DieSet) -> (u8, u8) {
    let occurrences = count_occurrences(dice);
    let mode = *(&occurrences)
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers");
    (mode, occurrences[&mode])
}

pub fn count_occurrences(dice: &DieSet) -> HashMap<u8, u8> {
    let mut occurrences: HashMap<u8, u8> = HashMap::new();
    for &value in dice {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    occurrences
}

pub fn calc_straight_length(dice: &DieSet) -> u8 {
    let mut dice = dice.to_vec();
    dice.sort();
    let mut last = 99; // Initialize last to a value we know won't increment length
    let mut length = 1;
    let mut max_length = 1;
    for die in dice {
        println!("{}", die);
        if die == last {
            continue;
        }
        if die == last + 1 {
            length += 1;
            max_length = cmp::max(max_length, length);
        } else {
            length = 1;
        }
        last = die;
    }
    max_length
}

pub fn print_dice(dice: &DieSet, prefix: &str) {
    println!(
        "{0}{1} {2} {3} {4} {5}",
        prefix, dice[0], dice[1], dice[2], dice[3], dice[4],
    );
}
