use crate::cardutils::*;
use crate::basicstrategy::*;
#[derive(Debug, Clone)]
pub struct GameRules {
    pub decks_in_shoe: u8,
    pub deck_penetration: usize,
    pub hit_soft_17: bool,
    pub double_after_split: bool,
    pub surrender: Option<Surrender>,
    pub resplit_aces: bool,
    pub hit_split_aces: bool,
    pub double_split_aces: bool,
}

impl GameRules {
    pub fn new(
        decks_in_shoe: u8,
        deck_penetration: usize,
        hit_soft_17: bool,
        double_after_split: bool,
        surrender: Option<Surrender>,
        resplit_aces: bool,
        hit_split_aces: bool,
        double_split_aces: bool,
    ) -> Self {
        GameRules {
            decks_in_shoe,
            deck_penetration,
            hit_soft_17,
            double_after_split,
            surrender,
            resplit_aces,
            hit_split_aces,
            double_split_aces,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Count {
    pub deck_estimation_factor: f64,
    pub running_count: usize,
    pub cards_seen: usize,
    pub std_bet: usize,
}

impl Count {
    pub fn new(std_bet: usize, deck_estimation_factor: f64) -> Self {
        Count {
            deck_estimation_factor,
            running_count: 0,
            cards_seen: 0,
            std_bet,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub rounds_left: usize,
    pub rules: GameRules,
    pub count: Count,
    pub wins: usize,
    pub losses: usize,
    pub pushes: usize,
    pub hits: usize,
    pub doubles: usize,
    pub splits: usize,
    pub stands: usize,
    pub surrenders: usize,
    pub blackjacks: usize,
    pub amount_bet: usize,
    pub amount_won: usize,
    pub amount_lost: usize,
    pub current_round: Round,
    pub bankroll: usize,
}

impl Game {
    pub fn new(rules: GameRules, count: Count, bankroll: usize, rounds_to_play: usize) -> Self {
        let mut shoe = Shoe::new(rules.decks_in_shoe);
        Game {
            rounds_left: rounds_to_play,
            rules: rules.clone(),
            wins: 0,
            blackjacks: 0,
            current_round: Round {
                rules: rules.clone(),
                hands: vec![Hand::new(count.std_bet, &mut shoe)],
                dealer: HandCards::from(&mut shoe),
                has_split_aces: false,
                shoe: Shoe::new(rules.decks_in_shoe),
            },
            count,
            losses: 0,
            pushes: 0,
            surrenders: 0,
            amount_bet: 0,
            amount_lost: 0,
            amount_won: 0,
            doubles: 0,
            hits: 0,
            stands: 0,
            splits: 0,
            bankroll,
        }
    }

    pub fn play(mut self) -> Self {
        while self.rounds_left > 0 {
            self.play_round();
            if self.bankroll <= (self.count.std_bet << 3) {
                return self;
            }
            self.rounds_left -= 1;
        }
        self
    }

    fn play_round(&mut self) {
        self.new_round();
        let mut hand_index = 0;
        let mut play;
        while hand_index < self.current_round.hands.len() {
            play = BasicStrategyLUT::make_move(self, hand_index);
            match play {
                Decision::Hit => {
                    self.hit(hand_index);
                },
                Decision::Stand => {
                    self.stand();
                    hand_index += 1;
                    continue;
                },
                Decision::Double => {
                    self.double(hand_index);
                    hand_index += 1;
                    continue;
                },
                Decision::Split => {
                    self.split(hand_index);
                },
                Decision::Surrender => {
                    self.surrender();
                    hand_index += 1;
                    continue;
                },
                _ => unreachable!(),
            }
        }
        self.current_round.dealer.dealer_play(&mut self.current_round.shoe, self.rules.hit_soft_17);
        for hand_index in 0..self.current_round.hands.len() {
            if self.is_bust(hand_index) {
                self.loss(hand_index);
            } else if self.is_push(hand_index) {
                self.push(hand_index);
            } else if self.is_winner(hand_index) {
                self.award_winnings(hand_index);
            } else {
                self.loss(hand_index);
            }
        }
        if self.should_reshuffle() {
            self.reshuffle();
        }
    }

    pub fn split(&mut self, hand_index: usize) {
        self.splits += 1;
        self.current_round.split(hand_index);
        self.bankroll -= self.current_round.hands[hand_index].bet;
    }

    pub fn double(&mut self, hand_index: usize) {
        self.doubles += 1;
        self.current_round.double(hand_index);
        self.bankroll -= self.current_round.hands[hand_index].bet >> 1;
    }

    pub fn hit(&mut self, hand_index: usize) {
        self.hits += 1;
        self.current_round.hit(hand_index);
    }

    pub fn surrender(&mut self) {
        self.surrenders += 1;
        self.bankroll += self.current_round.hands[0].bet >> 1;
    }

    pub fn stand(&mut self) {
        self.stands += 1;
    }

    pub fn should_reshuffle(&mut self) -> bool {
        let max_penetration = self.rules.deck_penetration;
        let cards_left = self.current_round.shoe.cards_left();


        cards_left <= max_penetration
    }

    pub fn reshuffle(&mut self) {
        self.current_round.shoe = Shoe::new(self.rules.decks_in_shoe);
        self.count.cards_seen = 0;
        self.count.running_count = 0;
    }

    pub fn is_bust(&mut self, hand_index: usize) -> bool {
        self.current_round.hands[hand_index].cards.is_bust()
    }

    pub fn is_winner(&mut self, hand_index: usize) -> bool {
        let dealer_value = self.current_round.dealer.num_value();
        let player_value = self.current_round.hands[hand_index].cards.num_value();
        if dealer_value > 21 {
            return true;
        }
        player_value > dealer_value
    }

    pub fn is_push(&mut self, hand_index: usize) -> bool {
        let dealer_value = self.current_round.dealer.num_value();
        let player_value = self.current_round.hands[hand_index].cards.num_value();
        if dealer_value > 21 {
            return false;
        }
        dealer_value == player_value
    }

    pub fn is_blackjack(&mut self, hand_index: usize) -> bool {
        self.current_round.hands[hand_index].is_blackjack()
    }

    pub fn award_winnings(&mut self, hand_index: usize) {
        if self.is_blackjack(hand_index) {
            self.award_blackjack(hand_index);
            return;
        }
        if self.is_winner(hand_index) {
            self.award_normal_win(hand_index);
        }
    }
    pub fn loss(&mut self, hand_index: usize) {
        self.losses += 1;
        self.amount_lost += self.current_round.hands[hand_index].bet;
    }


    pub fn push(&mut self, hand_index: usize) {
        let bet = self.current_round.hands[hand_index].bet;
        self.pushes += 1;
        self.bankroll += bet;
    }

    fn award_blackjack(&mut self, hand_index: usize) {
        let bet = self.current_round.hands[hand_index].bet;
        self.blackjacks += 1;
        self.wins += 1;
        self.amount_won += bet + (bet >> 1);
        self.bankroll += (bet << 1) + (bet >> 1);
    }

    fn award_normal_win(&mut self, hand_index: usize) {
        let bet = self.current_round.hands[hand_index].bet;
        self.wins += 1;
        self.amount_won += bet;
        self.bankroll += bet << 1;
    }

    fn new_round(&mut self) {
        self.bankroll -= self.count.std_bet;
        self.current_round.hands = vec![Hand::new(self.count.std_bet, &mut self.current_round.shoe)];
        self.current_round.dealer = HandCards::from(&mut self.current_round.shoe);
        self.current_round.has_split_aces = false;
    }

}

#[derive(Debug, Clone)]
pub struct Round {
    pub rules: GameRules,
    pub shoe: Shoe,
    pub has_split_aces: bool,
    pub hands: Vec<Hand>,
    pub dealer: HandCards,
}

impl Round {
    pub fn can_split(&self, hand_to_split: usize) -> bool {
        let splits = &self.hands.len() - 1;
        let hand = &self.hands[hand_to_split];
        if !hand.cards.is_pair() {
            return false;
        }

        let pair_of = hand.cards.first_card().get_rank();

        if !self.rules.resplit_aces && self.has_split_aces && pair_of == Rank::Ace {
            return false;
        }
        
        if splits >= 3 {
            return false;
        }

        true
    }
    pub fn can_double(&self, hand_to_double: usize) -> bool {
        let hand = &self.hands[hand_to_double];
        let splits = &self.hands.len() - 1;
        let double_after_split = self.rules.double_after_split;
        let double_split_aces = self.rules.double_split_aces;

        if hand.cards.len() != 2 {
            return false;
        }
        if !double_split_aces && hand.split_from == Some(Rank::Ace) {
            return false;
        }
        if !double_after_split && (splits > 0) {
            return false;
        }
        true
    }
    pub fn can_surrender(&self) -> bool {
        let splits = &self.hands.len() - 1;
        let hand_len = self.hands[0].cards.len();
        let surrender = self.rules.surrender;
        if splits != 0 || hand_len > 2 || Option::is_none(&surrender) {
            return false;
        }
        true
    }
    pub fn can_hit(&self, hand_to_hit: usize) -> bool {
        let hand = &self.hands[hand_to_hit];
        let splits = &self.hands.len() - 1;
        let hit_split_aces = &self.rules.hit_split_aces;
        let split_from = hand.split_from;
        if splits != 0 && !hit_split_aces && split_from == Some(Rank::Ace) {
            return false;
        }
        true
    }

    pub fn split(&mut self, hand_index: usize) {
        let card = self.hands[hand_index].cards.first_card();
        let new_hand = self.hands[hand_index].split(&mut self.shoe);
        if card.get_rank() == Rank::Ace {
            self.has_split_aces = true;
        }
        self.hands.push(new_hand);
    }

    pub fn double(&mut self, hand_index: usize) {
        let hand = &mut self.hands[hand_index];
        hand.double(&mut self.shoe);
    }
    pub fn hit(&mut self, hand_index: usize) {
        let hand = &mut self.hands[hand_index];
        hand.hit(&mut self.shoe);
    }
}
#[derive(Debug, Clone)]
pub struct Hand {
    pub split_from: Option<Rank>,
    pub bet: usize,
    pub cards: HandCards,
}

impl Hand {
    pub fn new(bet: usize, shoe: &mut Shoe) -> Self {
        Hand {
            bet,
            split_from: None,
            cards: HandCards::from(shoe),
        }
    }
    
    pub fn split(&mut self, shoe: &mut Shoe) -> Hand {
        let card = self.cards.pop();
        let card_rank = card.get_rank();
        let mut hand = HandCards::new();
        hand.push(card);
        hand.push(shoe.deal());

        self.cards.push(shoe.deal());
        self.split_from = Some(card_rank);

        Hand {
            bet: self.bet,
            split_from: Some(card_rank),
            cards: hand,
        }
    }

    pub fn hit(&mut self, shoe: &mut Shoe) {
        self.cards.hit(shoe);
    }

    pub fn double(&mut self, shoe: &mut Shoe) {
        self.bet <<= 1;
        self.cards.hit(shoe);
    }

    pub fn is_bust(&self) -> bool {
        self.cards.is_bust()
    }

    pub fn is_blackjack(&self) -> bool {
        self.split_from.is_none() && self.cards.len() == 2 && self.cards.num_value() == 21
    }
}
