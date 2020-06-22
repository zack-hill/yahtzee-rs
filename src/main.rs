mod dice;
mod score_card;

use dice::*;
use score_card::*;

use std::io;

fn main() {
    let mut card = ScoreCard::new();
    card.print();

    while !card.is_complete() {
        let mut dice = dice::roll_dice();
        print_dice(&dice, "Dice: ");
        reroll(&mut dice);

        let mut yahtzee_bonus_applied = false;
        loop {
            let category = get_category();
            let score = ScoreCard::score_roll(&dice, category);

            if card.get_score(&category).is_none() {
                card.set_score(&category, score);
                break;
            } else {
                if category == ScoringCategory::Yahtzee && score != 0 && !yahtzee_bonus_applied {
                    card.set_score(&category, score);
                    println!("Yahtzee bonus applied. Pick another category.");
                    yahtzee_bonus_applied = true;
                    continue;
                }
                println!("Category already filled. Pick another one.");
            }
        }
        card.calculate_score();
        card.print();
    }
    println!("Game Complete!")
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

fn get_category() -> ScoringCategory {
    loop {
        println!("Enter the id of the category to score:");
        let input = read_line();
        match input.trim().parse::<u8>() {
            Ok(x) => {
                match x {
                    1 => return ScoringCategory::Ones,
                    2 => return ScoringCategory::Twos,
                    3 => return ScoringCategory::Threes,
                    4 => return ScoringCategory::Fours,
                    5 => return ScoringCategory::Fives,
                    6 => return ScoringCategory::Sixes,
                    7 => return ScoringCategory::ThreeOfAKind,
                    8 => return ScoringCategory::FourOfAKind,
                    9 => return ScoringCategory::FullHouse,
                    10 => return ScoringCategory::SmallStraight,
                    11 => return ScoringCategory::LargeStraight,
                    12 => return ScoringCategory::Yahtzee,
                    13 => return ScoringCategory::Chance,
                    _ => println!("Error: Invalid category"),
                };
            }
            Err(_) => println!("Error: Invalid category"),
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
