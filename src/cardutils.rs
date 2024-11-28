use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }
    pub fn value(&self) -> u8 {
        self.rank.value()
    }
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let rank_num = rng.gen_range(0..13);
        let rank = match rank_num {
            0 => Rank::Ace,
            1..=9 => Rank::Number(rank_num),
            10 => Rank::Jack,
            11 => Rank::Queen,
            12 => Rank::King,
            _ => unreachable!(),
        };

        let suit_num = rng.gen_range(0..4);
        let suit = match suit_num {
            0 => Suit::Spades,
            1 => Suit::Clubs,
            2 => Suit::Hearts,
            3 => Suit::Diamonds,
            _ => unreachable!(),
        };
        Card::new(rank, suit)
    }
    pub fn get_rank(&self) -> Rank {
        self.rank
    }
    pub fn hilo_value(&self) -> f64 {
        match self.rank {
            Rank::Ace | Rank::Number(2) | Rank::Number(3) | Rank::Number(4) | Rank::Number(5) | Rank::Number(6) => 1.0,
            Rank::Number(7) | Rank::Number(8) | Rank::Number(9) => 0.0,
            Rank::Number(10) | Rank::Jack | Rank::Queen | Rank::King => -1.0,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}
impl Rank {
    fn value(&self) -> u8 {
        match self {
            Rank::Ace => 11,
            Rank::Number(n) => *n,
            Rank::Jack | Rank::Queen | Rank:: King => 10,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, Clone)]
pub struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new(num_decks: u8) -> Shoe {
        let mut cards = vec![];
        let mut rng = rand::thread_rng();

        // Populate the shoe with cards
        for _ in 0..(num_decks) {
            Self::push_fresh_deck(&mut cards);
        }
        cards.shuffle(&mut rng);

        Shoe {
            cards
        }
    }
    pub fn deal(&mut self) -> Card {
        self.cards.pop().expect("No cards left in shoe")
    }
    pub fn cards_left(&self) -> usize {
        self.cards.len()
    }
    fn push_fresh_deck(cards: &mut Vec<Card>) {
        cards.push(Card::new(Rank::Ace, Suit::Spades));
        cards.push(Card::new(Rank::Number(2), Suit::Spades));
        cards.push(Card::new(Rank::Number(3), Suit::Spades));
        cards.push(Card::new(Rank::Number(4), Suit::Spades));
        cards.push(Card::new(Rank::Number(5), Suit::Spades));
        cards.push(Card::new(Rank::Number(6), Suit::Spades));
        cards.push(Card::new(Rank::Number(7), Suit::Spades));
        cards.push(Card::new(Rank::Number(8), Suit::Spades));
        cards.push(Card::new(Rank::Number(9), Suit::Spades));
        cards.push(Card::new(Rank::Number(10), Suit::Spades));
        cards.push(Card::new(Rank::Jack, Suit::Spades));
        cards.push(Card::new(Rank::Queen, Suit::Spades));
        cards.push(Card::new(Rank::King, Suit::Spades));

        cards.push(Card::new(Rank::Ace, Suit::Clubs));
        cards.push(Card::new(Rank::Number(2), Suit::Clubs));
        cards.push(Card::new(Rank::Number(3), Suit::Clubs));
        cards.push(Card::new(Rank::Number(4), Suit::Clubs));
        cards.push(Card::new(Rank::Number(5), Suit::Clubs));
        cards.push(Card::new(Rank::Number(6), Suit::Clubs));
        cards.push(Card::new(Rank::Number(7), Suit::Clubs));
        cards.push(Card::new(Rank::Number(8), Suit::Clubs));
        cards.push(Card::new(Rank::Number(9), Suit::Clubs));
        cards.push(Card::new(Rank::Number(10), Suit::Clubs));
        cards.push(Card::new(Rank::Jack, Suit::Clubs));
        cards.push(Card::new(Rank::Queen, Suit::Clubs));
        cards.push(Card::new(Rank::King, Suit::Clubs));

        cards.push(Card::new(Rank::Ace, Suit::Hearts));
        cards.push(Card::new(Rank::Number(2), Suit::Hearts));
        cards.push(Card::new(Rank::Number(3), Suit::Hearts));
        cards.push(Card::new(Rank::Number(4), Suit::Hearts));
        cards.push(Card::new(Rank::Number(5), Suit::Hearts));
        cards.push(Card::new(Rank::Number(6), Suit::Hearts));
        cards.push(Card::new(Rank::Number(7), Suit::Hearts));
        cards.push(Card::new(Rank::Number(8), Suit::Hearts));
        cards.push(Card::new(Rank::Number(9), Suit::Hearts));
        cards.push(Card::new(Rank::Number(10), Suit::Hearts));
        cards.push(Card::new(Rank::Jack, Suit::Hearts));
        cards.push(Card::new(Rank::Queen, Suit::Hearts));
        cards.push(Card::new(Rank::King, Suit::Hearts));

        cards.push(Card::new(Rank::Ace, Suit::Diamonds));
        cards.push(Card::new(Rank::Number(2), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(3), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(4), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(5), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(6), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(7), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(8), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(9), Suit::Diamonds));
        cards.push(Card::new(Rank::Number(10), Suit::Diamonds));
        cards.push(Card::new(Rank::Jack, Suit::Diamonds));
        cards.push(Card::new(Rank::Queen, Suit::Diamonds));
        cards.push(Card::new(Rank::King, Suit::Diamonds));
    }
}

#[derive(Debug, Clone)]
pub struct HandCards {
    pub cards: Vec<Card>,
}


impl HandCards {
    pub fn new() -> Self {
        HandCards { cards: vec![] }
    }
    pub fn from(shoe: &mut Shoe) -> Self {
        let cards = vec![shoe.deal(), shoe.deal()];
        HandCards {
            cards: cards.clone(),
        }
    }
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    pub fn hit(&mut self, shoe: &mut Shoe) {
        self.cards.push(shoe.deal());
    }
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
    // Returns (Number value, soft or hard)
    pub fn value(&self) -> (u8, ValueType) {
        (self.num_value(), self.value_type())
    }

    pub fn first_card(&self) -> Card {
        self.cards[0]
    }

    pub fn value_type(&self) -> ValueType {
        match !self.has_ace() || self.raw_value() > 21 {
            true => ValueType::Hard,
            false => ValueType::Soft,
        }
    }

    pub fn is_pair(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].rank == self.cards[1].rank
    }

    pub fn is_bust(&self) -> bool {
        self.num_value() > 21
    }

    pub fn has_ace(&self) -> bool {
        for card in self.cards.iter() {
            if card.rank == Rank::Ace { return true; }
        };
        false
    }
    fn raw_value(&self) -> u8 {
        let mut value = 0;
        for card in self.cards.iter() {
            value += card.value();
        }
        value
    }
    pub fn num_value(&self) -> u8 {
        let mut value = self.raw_value();
        if self.has_ace() && value > 21 {
            value -= 10;
        }
        value
    }

    pub fn pop(&mut self) -> Card {
        self.cards.pop().expect("No cards left in hand")
    }

    pub fn dealer_play(&mut self, shoe: &mut Shoe, hit_soft_17: bool) {
        match hit_soft_17 {
            true => self.dealer_play_h17(shoe),
            false => self.dealer_play_s17(shoe),
        }
            
    }

    fn dealer_play_s17(&mut self, shoe: &mut Shoe) {
        while self.num_value() < 17 {
            self.hit(shoe);
        }
    }

    fn dealer_play_h17(&mut self, shoe: &mut Shoe) {
        loop {
            let value = self.num_value();
            match value {
                0..=16 => {
                    self.hit(shoe);
                }
                17 => {
                    if self.has_ace() {
                        self.hit(shoe);
                    } else {
                        break;
                    }
                },
                _ => break,
            }
        }
    }
}

impl Default for HandCards {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Hard,
    Soft,
}
