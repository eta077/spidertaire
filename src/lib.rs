#![deny(missing_docs)]
#![deny(clippy::all)]
//! Defines structures representing cards and collections of cards.

use rand::prelude::*;

/// The value of a card.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardValue {
    K,
    Q,
    J,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    A,
}

impl CardValue {
    /// Creates an array of all card values in descending order.
    pub fn all() -> [CardValue; 13] {
        [
            CardValue::K,
            CardValue::Q,
            CardValue::J,
            CardValue::Ten,
            CardValue::Nine,
            CardValue::Eight,
            CardValue::Seven,
            CardValue::Six,
            CardValue::Five,
            CardValue::Four,
            CardValue::Three,
            CardValue::Two,
            CardValue::A,
        ]
    }

    /// Returns the string representation of the card value.
    pub fn as_str(&self) -> &str {
        match self {
            CardValue::K => "K",
            CardValue::Q => "Q",
            CardValue::J => "J",
            CardValue::Ten => "10",
            CardValue::Nine => "9",
            CardValue::Eight => "8",
            CardValue::Seven => "7",
            CardValue::Six => "6",
            CardValue::Five => "5",
            CardValue::Four => "4",
            CardValue::Three => "3",
            CardValue::Two => "2",
            CardValue::A => "A",
        }
    }

    /// Gets the value one higher than the current value.
    ///
    /// ```
    /// use spidertaire::*;
    ///
    /// assert_eq!(CardValue::K.previous(), None);
    /// assert_eq!(CardValue::Seven.previous(), Some(CardValue::Eight));
    /// ```
    pub fn previous(&self) -> Option<CardValue> {
        match self {
            CardValue::K => None,
            CardValue::Q => Some(CardValue::K),
            CardValue::J => Some(CardValue::Q),
            CardValue::Ten => Some(CardValue::J),
            CardValue::Nine => Some(CardValue::Ten),
            CardValue::Eight => Some(CardValue::Nine),
            CardValue::Seven => Some(CardValue::Eight),
            CardValue::Six => Some(CardValue::Seven),
            CardValue::Five => Some(CardValue::Six),
            CardValue::Four => Some(CardValue::Five),
            CardValue::Three => Some(CardValue::Four),
            CardValue::Two => Some(CardValue::Three),
            CardValue::A => Some(CardValue::Two),
        }
    }

    /// Gets the value one lower than the current value.
    ///
    /// ```
    /// use spidertaire::*;
    ///
    /// assert_eq!(CardValue::A.next(), None);
    /// assert_eq!(CardValue::Seven.next(), Some(CardValue::Six));
    /// ```
    pub fn next(&self) -> Option<CardValue> {
        match self {
            CardValue::K => Some(CardValue::Q),
            CardValue::Q => Some(CardValue::J),
            CardValue::J => Some(CardValue::Ten),
            CardValue::Ten => Some(CardValue::Nine),
            CardValue::Nine => Some(CardValue::Eight),
            CardValue::Eight => Some(CardValue::Seven),
            CardValue::Seven => Some(CardValue::Six),
            CardValue::Six => Some(CardValue::Five),
            CardValue::Five => Some(CardValue::Four),
            CardValue::Four => Some(CardValue::Three),
            CardValue::Three => Some(CardValue::Two),
            CardValue::Two => Some(CardValue::A),
            CardValue::A => None,
        }
    }
}

/// ```
/// use spidertaire::*;
///
/// let value: u8 = CardValue::K.into();
/// assert_eq!(value, 13);
///
/// let value: u8 = CardValue::A.into();
/// assert_eq!(value, 1);
/// ```
impl From<CardValue> for u8 {
    fn from(value: CardValue) -> Self {
        match value {
            CardValue::K => 13,
            CardValue::Q => 12,
            CardValue::J => 11,
            CardValue::Ten => 10,
            CardValue::Nine => 9,
            CardValue::Eight => 8,
            CardValue::Seven => 7,
            CardValue::Six => 6,
            CardValue::Five => 5,
            CardValue::Four => 4,
            CardValue::Three => 3,
            CardValue::Two => 2,
            CardValue::A => 1,
        }
    }
}

/// ```
/// use spidertaire::*;
/// 
/// let value = CardValue::try_from(13);
/// assert_eq!(value, Ok(CardValue::K));
/// 
/// let value = CardValue::try_from(1);
/// assert_eq!(value, Ok(CardValue::A));
/// 
/// let value = CardValue::try_from(100);
/// assert!(value.is_err());
/// ```
impl TryFrom<u8> for CardValue {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            13 => Ok(CardValue::K),
            12 => Ok(CardValue::Q),
            11 => Ok(CardValue::J),
            10 => Ok(CardValue::Ten),
            9 => Ok(CardValue::Nine),
            8 => Ok(CardValue::Eight),
            7 => Ok(CardValue::Seven),
            6 => Ok(CardValue::Six),
            5 => Ok(CardValue::Five),
            4 => Ok(CardValue::Four),
            3 => Ok(CardValue::Three),
            2 => Ok(CardValue::Two),
            1 => Ok(CardValue::A),
            _ => Err(["Unexpected card value: ", &value.to_string()].concat()),
        }
    }
}

/// The suit of a card.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl CardSuit {
    /// Creates an array of all card suits.
    pub fn all() -> [CardSuit; 4] {
        [
            CardSuit::Hearts,
            CardSuit::Diamonds,
            CardSuit::Clubs,
            CardSuit::Spades,
        ]
    }

    /// Returns the string representation of the card suit.
    pub fn as_str(&self) -> &str {
        match self {
            CardSuit::Hearts => "\u{2665}",
            CardSuit::Diamonds => "\u{2666}",
            CardSuit::Clubs => "\u{2663}",
            CardSuit::Spades => "\u{2660}",
        }
    }
}

/// A playing card.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    /// The value of the card.
    pub value: CardValue,
    /// The suit of the card.
    pub suit: CardSuit,
}

/// A 52-card deck.
#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    /// The cards in the deck.
    pub cards: Vec<Card>,
}

impl Deck {
    /// Constructs a new Deck with all suits.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a new Deck with 52 cards of the same suit.
    ///
    /// ```
    /// use spidertaire::*;
    ///
    /// let deck = Deck::from_suit(CardSuit::Spades);
    /// assert!(deck.cards
    ///     .iter()
    ///     .find(|card| card.suit == CardSuit::Hearts
    ///         || card.suit == CardSuit::Diamonds
    ///         || card.suit == CardSuit::Clubs)
    ///     .is_none());
    /// ```
    pub fn from_suit(suit: CardSuit) -> Deck {
        let mut cards = Vec::with_capacity(52);
        for _ in 0..4 {
            for value in CardValue::all() {
                cards.push(Card { value, suit });
            }
        }
        Deck { cards }
    }

    /// Constructs a new Deck with two suits of 26 cards each.
    ///
    /// ```
    /// use spidertaire::*;
    ///
    /// let deck = Deck::from_suits(CardSuit::Spades, CardSuit::Hearts);
    /// assert!(deck.cards
    ///     .iter()
    ///     .find(|card| card.suit == CardSuit::Diamonds || card.suit == CardSuit::Clubs)
    ///     .is_none());
    /// ```
    pub fn from_suits(suit1: CardSuit, suit2: CardSuit) -> Deck {
        let mut cards = Vec::with_capacity(52);
        for _ in 0..2 {
            for value in CardValue::all() {
                cards.push(Card { value, suit: suit1 });
            }
            for value in CardValue::all() {
                cards.push(Card { value, suit: suit2 });
            }
        }
        Deck { cards }
    }

    /// Randomly orders the cards in the deck.
    ///
    /// ```
    /// use spidertaire::*;
    ///
    /// let mut deck = Deck::new();
    /// deck.shuffle();
    ///
    /// assert!(deck != Deck::new());
    ///
    /// let shuffle1 = deck.clone();
    /// deck.shuffle();
    ///
    /// assert!(deck != shuffle1);
    /// ```
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Merges the cards from deck2 into this deck.
    /// 
    /// ```
    /// use spidertaire::*;
    /// 
    /// let mut deck = Deck::new();
    /// deck.combine(Deck::new());
    /// assert_eq!(deck.cards.len(), 104);
    /// ```
    pub fn combine(&mut self, mut deck2: Deck) {
        self.cards.append(&mut deck2.cards);
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in CardSuit::all() {
            for value in CardValue::all() {
                cards.push(Card { value, suit });
            }
        }
        Self { cards }
    }
}
