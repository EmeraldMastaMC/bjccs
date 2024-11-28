#[path = "./cardutils.rs"]
pub mod cardutils;
pub mod gamelogic;
pub mod basicstrategy;

#[cfg(test)]
mod card_utils_tests {
    use crate::cardutils::*;
    use crate::basicstrategy;
    use crate::gamelogic;
    use rand::Rng;
    #[test]
    fn card_value() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(card.value(), 11);

        let card = Card::new(Rank::Number(2), Suit::Spades);
        assert_eq!(card.value(), 2);

        let card = Card::new(Rank::Number(3), Suit::Spades);
        assert_eq!(card.value(), 3);

        let card = Card::new(Rank::Number(4), Suit::Spades);
        assert_eq!(card.value(), 4);

        let card = Card::new(Rank::Number(5), Suit::Spades);
        assert_eq!(card.value(), 5);

        let card = Card::new(Rank::Number(6), Suit::Spades);
        assert_eq!(card.value(), 6);

        let card = Card::new(Rank::Number(7), Suit::Spades);
        assert_eq!(card.value(), 7);

        let card = Card::new(Rank::Number(8), Suit::Spades);
        assert_eq!(card.value(), 8);

        let card = Card::new(Rank::Number(9), Suit::Spades);
        assert_eq!(card.value(), 9);

        let card = Card::new(Rank::Number(10), Suit::Spades);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Jack, Suit::Spades);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Queen, Suit::Spades);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::King, Suit::Spades);
        assert_eq!(card.value(), 10);



        let card = Card::new(Rank::Ace, Suit::Clubs);
        assert_eq!(card.value(), 11);

        let card = Card::new(Rank::Number(2), Suit::Clubs);
        assert_eq!(card.value(), 2);

        let card = Card::new(Rank::Number(3), Suit::Clubs);
        assert_eq!(card.value(), 3);

        let card = Card::new(Rank::Number(4), Suit::Clubs);
        assert_eq!(card.value(), 4);

        let card = Card::new(Rank::Number(5), Suit::Clubs);
        assert_eq!(card.value(), 5);

        let card = Card::new(Rank::Number(6), Suit::Clubs);
        assert_eq!(card.value(), 6);

        let card = Card::new(Rank::Number(7), Suit::Clubs);
        assert_eq!(card.value(), 7);

        let card = Card::new(Rank::Number(8), Suit::Clubs);
        assert_eq!(card.value(), 8);

        let card = Card::new(Rank::Number(9), Suit::Clubs);
        assert_eq!(card.value(), 9);

        let card = Card::new(Rank::Number(10), Suit::Clubs);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Jack, Suit::Clubs);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Queen, Suit::Clubs);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::King, Suit::Clubs);
        assert_eq!(card.value(), 10);



        let card = Card::new(Rank::Ace, Suit::Hearts);
        assert_eq!(card.value(), 11);

        let card = Card::new(Rank::Number(2), Suit::Hearts);
        assert_eq!(card.value(), 2);

        let card = Card::new(Rank::Number(3), Suit::Hearts);
        assert_eq!(card.value(), 3);

        let card = Card::new(Rank::Number(4), Suit::Hearts);
        assert_eq!(card.value(), 4);

        let card = Card::new(Rank::Number(5), Suit::Hearts);
        assert_eq!(card.value(), 5);

        let card = Card::new(Rank::Number(6), Suit::Hearts);
        assert_eq!(card.value(), 6);

        let card = Card::new(Rank::Number(7), Suit::Hearts);
        assert_eq!(card.value(), 7);

        let card = Card::new(Rank::Number(8), Suit::Hearts);
        assert_eq!(card.value(), 8);

        let card = Card::new(Rank::Number(9), Suit::Hearts);
        assert_eq!(card.value(), 9);

        let card = Card::new(Rank::Number(10), Suit::Hearts);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Jack, Suit::Hearts);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Queen, Suit::Hearts);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::King, Suit::Hearts);
        assert_eq!(card.value(), 10);



        let card = Card::new(Rank::Ace, Suit::Diamonds);
        assert_eq!(card.value(), 11);

        let card = Card::new(Rank::Number(2), Suit::Diamonds);
        assert_eq!(card.value(), 2);

        let card = Card::new(Rank::Number(3), Suit::Diamonds);
        assert_eq!(card.value(), 3);

        let card = Card::new(Rank::Number(4), Suit::Diamonds);
        assert_eq!(card.value(), 4);

        let card = Card::new(Rank::Number(5), Suit::Diamonds);
        assert_eq!(card.value(), 5);

        let card = Card::new(Rank::Number(6), Suit::Diamonds);
        assert_eq!(card.value(), 6);

        let card = Card::new(Rank::Number(7), Suit::Diamonds);
        assert_eq!(card.value(), 7);

        let card = Card::new(Rank::Number(8), Suit::Diamonds);
        assert_eq!(card.value(), 8);

        let card = Card::new(Rank::Number(9), Suit::Diamonds);
        assert_eq!(card.value(), 9);

        let card = Card::new(Rank::Number(10), Suit::Diamonds);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Jack, Suit::Diamonds);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::Queen, Suit::Diamonds);
        assert_eq!(card.value(), 10);

        let card = Card::new(Rank::King, Suit::Diamonds);
        assert_eq!(card.value(), 10);
    }

    #[test]
    fn shoe_creation() {
        const DECK_SIZE: usize = 52;
        let mut rng = rand::thread_rng();

        for _ in 0..5 {
            let num = rng.gen_range(1..=10) as usize;
            let shoe = Shoe::new(num as u8);
            assert_eq!(shoe.len(), DECK_SIZE * num);
        }
    }

    #[test]
    fn basic_strategy() {
        let rules = gamelogic::GameRules {
                double_after_split: true,
                resplit_aces: false,
                double_split_aces: false,
                surrender: None,
                hit_soft_17: false,
                decks_in_shoe: 6,
                hit_split_aces: false,
                blackjack_payout: 2.5,
        };
        let game = gamelogic::Game {
            total_rounds: 0,
            rules,
            wins: 0,
            blackjacks: 0,
            losses: 0,
            pushes: 0,
            surrenders: 0,
            amount_bet: 0.0,
            amount_lost: 0.0,
            amount_won: 0.0,
            average_bet: 0.0,
            current_round: gamelogic::Round {
                rules,
                bet: 0.0,
                hands: vec![
                    gamelogic::Hand {
                        bet: 0.0,
                        split_from: None,
                        cards: HandCards {
                            cards: vec![
                                Card::new(Rank::Number(9), Suit::Hearts),
                                Card::new(Rank::Number(7), Suit::Diamonds),
                            ],
                        },
                    },
                    gamelogic::Hand {
                        bet: 0.0,
                        split_from: None,
                        cards: HandCards {
                            cards: vec![
                                Card::new(Rank::Number(9), Suit::Hearts),
                                Card::new(Rank::Number(7), Suit::Diamonds),
                            ],
                        },
                    },
                    //gamelogic::Hand {
                    //    bet: 0.0,
                    //    split_from: None,
                    //    cards: HandCards {
                    //        cards: vec![
                    //            Card::new(Rank::Number(9), Suit::Hearts),
                    //            Card::new(Rank::Number(7), Suit::Diamonds),
                    //        ],
                    //    },
                    //},
                    gamelogic::Hand {
                        bet: 0.0,
                        split_from: None,
                        cards: HandCards {
                            cards: vec![
                                Card::new(Rank::Number(8), Suit::Hearts),
                                Card::new(Rank::Number(8), Suit::Diamonds),
                            ],
                        },
                    },
                ],
                dealer: HandCards {
                    cards: vec![
                        Card::new(Rank::Number(10), Suit::Hearts),
                        Card::new(Rank::Number(8), Suit::Diamonds),
                    ],
                },
                has_split_aces: false,
                shoe: Shoe::new(6),
            },
            bankroll: 100.0,
        };
        assert!(basicstrategy::BasicStrategyLUT::raw_move(&game, 2) == basicstrategy::Decision::Split);
    }
}
