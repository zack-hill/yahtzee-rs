use crate::dice::*;

enum ScoringCategory {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfAKind,
    FourOfAKind,
    FullHouse,
    SmallStraight,
    LargeStraight,
    Chance,
    Yahtzee,
}

pub struct ScoreCard {
    ones: Option<u32>,
    twos: Option<u32>,
    threes: Option<u32>,
    fours: Option<u32>,
    fives: Option<u32>,
    sixes: Option<u32>,
    three_of_a_kind: Option<u32>,
    four_of_a_kind: Option<u32>,
    full_house: Option<u32>,
    small_straight: Option<u32>,
    large_straight: Option<u32>,
    chance: Option<u32>,
    yahtzee: Option<u32>,
    yahtzee_bonus_count: u32,

    // Scoring
    upper_section_sub_total: u32,
    upper_section_bonus: u32,
    upper_section_total: u32,
    lower_section_total: u32,
    grand_total: u32,
}

impl ScoreCard {
    pub fn new() -> ScoreCard {
        ScoreCard {
            ones: None,
            twos: None,
            threes: None,
            fours: None,
            fives: None,
            sixes: None,
            three_of_a_kind: None,
            four_of_a_kind: None,
            full_house: None,
            small_straight: None,
            large_straight: None,
            chance: None,
            yahtzee: None,
            yahtzee_bonus_count: 0,
            upper_section_sub_total: 0,
            upper_section_bonus: 0,
            upper_section_total: 0,
            lower_section_total: 0,
            grand_total: 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.ones.is_some()
            && self.twos.is_some()
            && self.threes.is_some()
            && self.fours.is_some()
            && self.fives.is_some()
            && self.sixes.is_some()
            && self.three_of_a_kind.is_some()
            && self.four_of_a_kind.is_some()
            && self.full_house.is_some()
            && self.small_straight.is_some()
            && self.large_straight.is_some()
            && self.chance.is_some()
            && self.yahtzee.is_some()
    }

    pub fn score_roll(&self, dice: &DieSet, category: ScoringCategory) -> Option<u32> {
        match category {
            ScoringCategory::Ones => {
                if self.ones.is_some() {
                    return None;
                }
                return Some(count_dice(dice, 1));
            }
            ScoringCategory::Twos => {
                if self.twos.is_some() {
                    return None;
                }
                return Some(count_dice(dice, 2) * 2);
            }
            ScoringCategory::Threes => {
                if self.threes.is_some() {
                    return None;
                }
                return Some(count_dice(dice, 3) * 3);
            }
            ScoringCategory::Fours => {
                if self.fours.is_some() {
                    return None;
                }
                return Some(count_dice(dice, 4) * 4);
            }
            ScoringCategory::Fives => {
                if self.fives.is_some() {
                    return None;
                }
                return Some(count_dice(dice, 5) * 5);
            }
            ScoringCategory::Sixes => {
                if self.sixes.is_some() {
                    return None;
                }
                return Some(count_dice(dice, 6) * 6);
            }
            ScoringCategory::ThreeOfAKind => Some(0),
            ScoringCategory::FourOfAKind => Some(0),
            ScoringCategory::FullHouse => Some(0),
            ScoringCategory::SmallStraight => Some(0),
            ScoringCategory::LargeStraight => Some(0),
            ScoringCategory::Chance => Some(0),
            ScoringCategory::Yahtzee => Some(0),
        }
    }

    pub fn get_score(&self, category: ScoringCategory) -> Option<u32> {
        match category {
            ScoringCategory::Ones => self.ones,
            ScoringCategory::Twos => self.twos,
            ScoringCategory::Threes => self.threes,
            ScoringCategory::Fours => self.fours,
            ScoringCategory::Fives => self.fives,
            ScoringCategory::Sixes => self.sixes,
            ScoringCategory::ThreeOfAKind => self.three_of_a_kind,
            ScoringCategory::FourOfAKind => self.four_of_a_kind,
            ScoringCategory::FullHouse => self.full_house,
            ScoringCategory::SmallStraight => self.small_straight,
            ScoringCategory::LargeStraight => self.large_straight,
            ScoringCategory::Chance => self.chance,
            ScoringCategory::Yahtzee => self.yahtzee,
        }
    }

    pub fn set_score(&mut self, category: ScoringCategory, value: u32) {
        let value = Some(value);
        match category {
            ScoringCategory::Ones => self.ones = value,
            ScoringCategory::Twos => self.twos = value,
            ScoringCategory::Threes => self.threes = value,
            ScoringCategory::Fours => self.fours = value,
            ScoringCategory::Fives => self.fives = value,
            ScoringCategory::Sixes => self.sixes = value,
            ScoringCategory::ThreeOfAKind => self.three_of_a_kind = value,
            ScoringCategory::FourOfAKind => self.four_of_a_kind = value,
            ScoringCategory::FullHouse => self.full_house = value,
            ScoringCategory::SmallStraight => self.small_straight = value,
            ScoringCategory::LargeStraight => self.large_straight = value,
            ScoringCategory::Chance => self.chance = value,
            ScoringCategory::Yahtzee => self.yahtzee = value,
        }
    }

    pub fn calculate_score(&mut self) {
        self.upper_section_sub_total = self.ones.unwrap_or_default()
            + self.twos.unwrap_or_default()
            + self.threes.unwrap_or_default()
            + self.fours.unwrap_or_default()
            + self.fives.unwrap_or_default()
            + self.sixes.unwrap_or_default();
        self.upper_section_bonus = if self.upper_section_sub_total >= 63 {
            35
        } else {
            0
        };
        self.upper_section_total = self.upper_section_sub_total + self.upper_section_bonus;
        self.lower_section_total = self.three_of_a_kind.unwrap_or_default()
            + self.four_of_a_kind.unwrap_or_default()
            + self.full_house.unwrap_or_default()
            + self.small_straight.unwrap_or_default()
            + self.large_straight.unwrap_or_default()
            + self.yahtzee.unwrap_or_default()
            + self.chance.unwrap_or_default()
            + self.yahtzee_bonus_count * 100;
        self.grand_total = self.upper_section_total + self.lower_section_total;
    }

    pub fn print(&self) {
        println!("=============================================================");
        println!("{0: <29} | {1: <29}", "Upper Section", "Lower Section");
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {0: <2} | {1: <15} | {2: <6}",
            "Id", "Category", "Points",
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "1",
            "Ones",
            format_category(self.ones),
            "7",
            "3 of a kind",
            format_category(self.three_of_a_kind)
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "2",
            "Twos",
            format_category(self.twos),
            "8",
            "4 of a kind",
            format_category(self.four_of_a_kind)
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "3",
            "Threes",
            format_category(self.threes),
            "9",
            "Full House",
            format_category(self.full_house)
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "4",
            "Fours",
            format_category(self.fours),
            "10",
            "Sm. Straight",
            format_category(self.small_straight)
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "5",
            "Fives",
            format_category(self.fives),
            "11",
            "Lg. Straight",
            format_category(self.large_straight)
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "6",
            "Sixes",
            format_category(self.sixes),
            "12",
            "YAHTZEE",
            format_category(self.yahtzee)
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "",
            "Upper Sub Total",
            self.upper_section_sub_total,
            "13",
            "Chance",
            format_category(self.chance)
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "",
            "Upper Bonus",
            self.upper_section_bonus,
            "",
            "YAHTZEE Bonus",
            self.yahtzee_bonus_count * 100
        );
        println!(
            "{0: <2} | {1: <15} | {2: <6} | {3: <2} | {4: <15} | {5: <6}",
            "",
            "Upper Total",
            self.upper_section_total,
            "",
            "Lower Total",
            self.lower_section_total
        );
        println!("-------------------------------------------------------------");
        println!(
            "{0: <34} | {1: <15} | {2: <6}",
            "", "Grand Total", self.grand_total
        );
        println!("=============================================================");
    }
}

fn format_category(num: Option<u32>) -> String {
    match num {
        Some(x) => x.to_string(),
        None => String::new(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_roll() {
        // assert_eq!(true, is_player_trapped(&board_state, 1));
    }

    #[test]
    fn calculate_score() {
        // let card = ScoreCard {
        //     ones: Some(3),
        //     twos: Some(6),
        //     threes: Some(6),
        //     fours: Some(20),
        //     fives: Some(20),
        //     sixes: Some(12),
        // }
    }
}
