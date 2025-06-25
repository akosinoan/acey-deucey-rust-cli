
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
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
}

#[derive(EnumIter, Debug, PartialEq ,Clone, Copy)]
pub enum Suite{
    Cloves,
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