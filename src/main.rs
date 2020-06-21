mod dice;
mod score_card;

use dice::*;
use score_card::*;

use std::io;

fn main() {
    let mut card = ScoreCard::new();
    card.calculate_score();
    card.print();

    while !card.is_complete() {
        let dice = roll();
        println!("Choose a category:");
    }
}

fn roll() -> DieSet {
    let mut dice = dice::roll_dice();
    print_dice(&dice, "Dice: ");
    reroll(&mut dice);
    dice
}

fn reroll(dice: &mut DieSet) {
    for reroll_num in 0..2 {
        loop {
            println!("Enter the dice to re-roll:");
            let input = read_line();
            let values: Vec<&str> = input.split(' ').collect();
            let mut numbers: Vec<u8> = Vec::new();
            for value in values {
                let value = value.trim();
                if value != "" {
                    match value.parse() {
                        Ok(x) => numbers.push(x),
                        Err(_) => {
                            println!("Error: {} is not a valid number.", value);
                            break;
                        }
                    }
                }
            }

            if numbers.len() == 0 {
                return;
            }

            let mut rerolled = dice.clone();
            let mut used_indices: Vec<usize> = Vec::new();

            let mut success = true;
            for number in numbers {
                let mut value_found = false;
                for (i, &die) in dice.iter().enumerate() {
                    if die == number && !used_indices.contains(&i) {
                        rerolled[i] = roll_die();
                        used_indices.push(i);
                        value_found = true;
                        break;
                    }
                }
                if !value_found {
                    println!("Error: Unable to find value {} in rolled dice.", number);
                    success = false;
                    break;
                }
            }
            if !success {
                continue;
            }
            for i in 0..5 {
                dice[i] = rerolled[i];
            }
            print_dice(&dice, &format!("Reroll #{}: ", reroll_num + 1));
            break;
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}
