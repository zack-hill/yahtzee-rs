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
