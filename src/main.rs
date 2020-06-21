fn main() {
    println!("Hello, world!");
}

enum ScoringCategory {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfAKind,
    FourOfAKind,
    SmallStraight,
    LargeStraight,
    Chance,
    Yahtzee,
    YahtzeeBonus,
}

struct ScoreCard {
    ones: Option<u32>,
    twos: Option<u32>,
    threes: Option<u32>,
    fours: Option<u32>,
    fives: Option<u32>,
    sixes: Option<u32>,
    three_of_a_kind: Option<u32>,
    four_of_a_kind: Option<u32>,
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
    fn calculate_score(&mut self) {
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
            + self.small_straight.unwrap_or_default()
            + self.large_straight.unwrap_or_default()
            + self.yahtzee.unwrap_or_default()
            + self.chance.unwrap_or_default()
            + self.yahtzee_bonus_count * 100;
        self.grand_total = self.upper_section_total + self.lower_section_total;
    }
}
