use rand::Rng;

pub type DieSet = [u8; 5];

pub fn roll_die() -> u8 {
    rand::thread_rng().gen_range(1, 7)
}

pub fn roll_dice() -> DieSet {
    [roll_die(), roll_die(), roll_die(), roll_die(), roll_die()]
}

pub fn count_dice(dice: &DieSet, num: u8) -> u32 {
    let mut count = 0;
    for die in dice.iter() {
        if *die == num {
            count += 1;
        }
    }
    count
}

pub fn print_dice(dice: &DieSet, prefix: &str) {
    println!(
        "{0}{1} {2} {3} {4} {5}",
        prefix, dice[0], dice[1], dice[2], dice[3], dice[4],
    );
}
