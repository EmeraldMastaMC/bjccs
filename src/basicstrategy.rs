use crate::cardutils::*;
use crate::gamelogic::*;
use Decision::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Decision {
    Hit,
    Stand,
    Double,
    Surrender,
    Split,
    DoubleOrStand,
    SplitIfDASOrHit,
    SurrenderOrHit,
    SurrenderOrStand,
    SurrenderOrSplit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LUTTyupe {
    Soft,
    Hard,
    Pair,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Surrender {
    Late,
    Early,
}
#[derive(Debug, Clone)]
pub struct BasicStrategyLUT {
    hard: [[Decision; 10]; 15],
    soft: [[Decision; 10]; 9],
    pair: [[Decision; 10]; 10],
}

impl BasicStrategyLUT {
    // TODO deviations
    #[inline(always)]
    pub fn make_move(game: &Game, hand_index: usize) -> Decision {
        let raw_move = Self::raw_move(game, hand_index);
        let can_double = game.current_round.can_double(hand_index);
        let can_hit = game.current_round.can_hit(hand_index);
        let can_surrender = game.current_round.can_surrender();
        let double_after_split = game.rules.double_after_split;
        match raw_move {
            Hit => {
                if can_hit {
                    Hit
                } else {
                    Stand
                }
            }
            Stand => Stand,
            Double => {
                if can_double {
                    Double
                } else {
                    Hit
                }
            }
            DoubleOrStand => {
                if can_double {
                    Double
                } else {
                    Stand
                }
            }
            Split => Split,
            SplitIfDASOrHit => {
                if double_after_split {
                    Split
                } else {
                    Hit
                }
            }
            SurrenderOrHit => {
                if can_surrender {
                    Surrender
                } else {
                    Hit
                }
            }
            SurrenderOrStand => {
                if can_surrender {
                    Surrender
                } else {
                    Stand
                }
            }
            SurrenderOrSplit => {
                if can_surrender {
                    Surrender
                } else {
                    Split
                }
            }
            Surrender => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn calculate_cache(game: &Game) {
        Self::cache_lut(Self::which_lut(game));
    }

    #[inline(always)]
    pub fn cache_lut(lut: &'static BasicStrategyLUT) {
        unsafe {
            CACHED_LUT = lut;
        }
    }

    #[inline(always)]
    pub fn raw_move(game: &Game, hand_index: usize) -> Decision {
        let (lut, lut_type) = Self::get_lut(game, hand_index);
        let (player_index, dealer_index) = Self::get_indices(game, lut_type, hand_index);

        match lut_type {
            LUTTyupe::Hard => lut.hard[player_index][dealer_index],
            LUTTyupe::Soft => lut.soft[player_index][dealer_index],
            LUTTyupe::Pair => lut.pair[player_index][dealer_index],
        }
    }

    #[inline(always)]
    fn get_indices(game: &Game, lut_type: LUTTyupe, hand_index: usize) -> (usize, usize) {
        match lut_type {
            LUTTyupe::Hard => Self::get_hard_indices(game, hand_index),
            LUTTyupe::Soft => Self::get_soft_indices(game, hand_index),
            LUTTyupe::Pair => Self::get_pair_indices(game, hand_index),
        }
    }

    #[inline(always)]
    fn get_pair_indices(game: &Game, hand_index: usize) -> (usize, usize) {
        let pair_of = game.current_round.hands[hand_index]
            .cards
            .first_card()
            .get_rank();
        let dealer_index = Self::dealer_index(game);
        let player_index = match pair_of {
            Rank::Number(n) => (n - 2) as usize,
            Rank::Jack => 8,
            Rank::Queen => 8,
            Rank::King => 8,
            Rank::Ace => 9,
        };
        (player_index, dealer_index)
    }

    #[inline(always)]
    fn get_hard_indices(game: &Game, hand_index: usize) -> (usize, usize) {
        let value = game.current_round.hands[hand_index].cards.num_value();
        let dealer_index = Self::dealer_index(game);
        let player_index = match value {
            ..=18 => (value - 4) as usize,
            19.. => 14,
        };
        (player_index, dealer_index)
    }

    #[inline(always)]
    fn get_soft_indices(game: &Game, hand_index: usize) -> (usize, usize) {
        let value = game.current_round.hands[hand_index].cards.num_value();
        let dealer_index = Self::dealer_index(game);
        let player_index = match value {
            ..=20 => (value - 12) as usize,
            21.. => 8,
        };
        (player_index, dealer_index)
    }

    #[inline(always)]
    fn dealer_index(game: &Game) -> usize {
        let dealer_card = game.current_round.dealer.first_card();
        (dealer_card.value() - 2) as usize
    }

    #[inline(always)]
    fn get_lut(game: &Game, hand_index: usize) -> (&'static BasicStrategyLUT, LUTTyupe) {
        (Self::get_cached_lut(), Self::type_of_lut(game, hand_index))
    }

    #[inline(always)]
    fn get_cached_lut() -> &'static BasicStrategyLUT {
        unsafe { CACHED_LUT }
    }

    #[inline(always)]
    fn type_of_lut(game: &Game, hand_index: usize) -> LUTTyupe {
        let can_split = game.current_round.can_split(hand_index);
        let value_type = game.current_round.hands[hand_index].cards.value_type();
        if can_split {
            return LUTTyupe::Pair;
        }
        match value_type {
            ValueType::Hard => LUTTyupe::Hard,
            ValueType::Soft => LUTTyupe::Soft,
        }
    }

    #[inline(always)]
    fn which_lut(game: &Game) -> &'static BasicStrategyLUT {
        let hit_soft_17 = game.rules.hit_soft_17;
        let decks_in_shoe = game.rules.decks_in_shoe;
        match (hit_soft_17, decks_in_shoe) {
            (false, 1) => unimplemented!(),
            (false, 2) => unimplemented!(),
            (false, 4..=8) => &BS_FOUR_EIGHT_DECK_S17,
            (true, 1) => unimplemented!(),
            (true, 2) => unimplemented!(),
            (true, 4..=8) => unimplemented!(),
            _ => panic!("Invalid rules"),
        }
    }
}

// TODO more basic strategy LUTs for different rulesets
static BS_FOUR_EIGHT_DECK_S17: BasicStrategyLUT = BasicStrategyLUT {
    hard: [
        // 2    3    4    5    6    7    8    9   10    A
        [Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit], // Four
        [Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit], // Five
        [Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit], // Six
        [Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit], // Seven
        [Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit, Hit], // Eight
        [Hit, Double, Double, Double, Double, Hit, Hit, Hit, Hit, Hit], // Nine
        [
            Double, Double, Double, Double, Double, Double, Double, Double, Hit, Hit,
        ], // Ten
        [
            Double, Double, Double, Double, Double, Double, Double, Double, Double, Double,
        ], // Eleven
        [Hit, Hit, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit], // Twelve
        [Stand, Stand, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit], // Thirteen
        [Stand, Stand, Stand, Stand, Stand, Hit, Hit, Hit, Hit, Hit], // Fourteen
        [
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
            Hit,
            Hit,
            Hit,
            SurrenderOrHit,
            SurrenderOrHit,
        ], // Fifteen
        [
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
            Hit,
            Hit,
            SurrenderOrHit,
            SurrenderOrHit,
            SurrenderOrHit,
        ], // Sixteen
        [
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
            SurrenderOrStand,
        ], // Seventeen
        [
            Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand,
        ], // Eighteen+
    ],
    soft: [
        // 2    3    4    5       6    7    8    9   10    A
        [Hit, Hit, Hit, Hit, Double, Hit, Hit, Hit, Hit, Hit], // A,A , or twelve, used when you cant split aces
        [Hit, Hit, Hit, Double, Double, Hit, Hit, Hit, Hit, Hit], // Thirteen
        [Hit, Hit, Hit, Double, Double, Hit, Hit, Hit, Hit, Hit], // Fourteen
        [Hit, Hit, Double, Double, Double, Hit, Hit, Hit, Hit, Hit], // Fifteen
        [Hit, Hit, Double, Double, Double, Hit, Hit, Hit, Hit, Hit], // Sixteen
        [Hit, Double, Double, Double, Double, Hit, Hit, Hit, Hit, Hit], // Seventeen
        [
            DoubleOrStand,
            DoubleOrStand,
            DoubleOrStand,
            DoubleOrStand,
            DoubleOrStand,
            Stand,
            Stand,
            Hit,
            Hit,
            Hit,
        ], // Eighteen
        [
            Stand,
            Stand,
            Stand,
            Stand,
            DoubleOrStand,
            Stand,
            Stand,
            Stand,
            Stand,
            Stand,
        ], // Nineteen
        [
            Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand,
        ], // Twenty+
    ],
    pair: [
        //             2                3      4      5      6      7    8    9   10    A
        [
            SplitIfDASOrHit,
            SplitIfDASOrHit,
            Split,
            Split,
            Split,
            Split,
            Hit,
            Hit,
            Hit,
            Hit,
        ], // 2,2
        [
            SplitIfDASOrHit,
            SplitIfDASOrHit,
            Split,
            Split,
            Split,
            Split,
            Hit,
            Hit,
            Hit,
            Hit,
        ], // 3,3
        [
            Hit,
            Hit,
            Hit,
            SplitIfDASOrHit,
            SplitIfDASOrHit,
            Hit,
            Hit,
            Hit,
            Hit,
            Hit,
        ], // 4,4
        [
            SplitIfDASOrHit,
            Split,
            Split,
            Split,
            Split,
            Hit,
            Hit,
            Hit,
            Hit,
            Hit,
        ], // 6,6
        [
            Double, Double, Double, Double, Double, Double, Double, Double, Hit, Hit,
        ], // 5,5
        [Split, Split, Split, Split, Split, Split, Hit, Hit, Hit, Hit], // 7,7
        [
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            SurrenderOrSplit,
        ], // 8,8
        [
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            Split,
            SurrenderOrSplit,
        ], // 9,9
        [
            Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand, Stand,
        ], // 10,10
        [
            Split, Split, Split, Split, Split, Split, Split, Split, Split, Split,
        ], // A,A
    ],
};

static mut CACHED_LUT: &BasicStrategyLUT = &BS_FOUR_EIGHT_DECK_S17;
