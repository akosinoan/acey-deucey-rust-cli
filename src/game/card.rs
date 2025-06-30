
use strum_macros::EnumIter;
use std::fmt;
#[derive(Debug)]
pub struct Card{
    number_value:u32,
    face_value:FaceValue,
    suite:Suite,

}

impl Card {
    pub fn new (number_value:u32,face_value:FaceValue,suite:Suite) -> Self {
        Self { number_value, face_value, suite }
    }

    pub fn number_value (&self)-> u32{
        self.number_value
    }

}

impl fmt::Display for Card{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         
        let face = match self.face_value {
            FaceValue::One =>   "One",
            FaceValue::Two =>   "Two",
            FaceValue::Three =>  "Three",
            FaceValue::Four =>   "Four",
            FaceValue::Five =>   "Five",
            FaceValue::Six =>   "Six",
            FaceValue::Seven =>   "Seven",
            FaceValue::Eight =>   "Eight",
            FaceValue::Nine =>   "Nine",
            FaceValue::Ten =>   "Ten",
            FaceValue::Jack =>   "Jack",
            FaceValue::Queen =>   "Queen",
            FaceValue::King =>   "King",
         };

        let suite =   match self.suite {
            Suite::Hearts =>   "♥ (Hearts)",
            Suite::Diamonds =>   "♦ (Diamonds)",
            Suite::Spades =>   "♠ (Spades)",
            Suite::Clubs =>   "♣ (Clubs)",
        };


        write!(f, "{} of {}", face, suite)

    }
}

#[derive(EnumIter, Debug, PartialEq ,Clone, Copy)]
pub enum Suite{
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}
#[derive(EnumIter, Debug, PartialEq,Clone, Copy)]
pub enum FaceValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}