use super::card::{Card, FaceValue, Suite};
use strum::IntoEnumIterator;
use rand::prelude::*;
pub struct Deck{
    cards:Vec<Card>,
    
}

impl Deck{
    pub fn shuffle(&mut self){
        
        println!("*****shuffling deck!*****");

        for i in 0..self.cards.len(){
            let mut rng = rand::rng();
            let swap_to = rng.random_range(i.. self.cards.len());
            
            self.cards.swap(i, swap_to);
            
        }

        // display cards in deck 
        // for card in &self.cards{
        //     println!("{:?}",card);
        // }
    }
  
    pub fn draw_card(&mut self)->  Option<Card>{
        self.cards.pop()
        
    }

    pub fn add_to_pile(&mut self, card: Card){
        self.cards.push(card);
    }

    pub fn _cards_remaining(&self) -> u32 {
        
        match u32::try_from(self.cards.len()){
                        Ok(i)=>i+1,
                        Err(_) => 0
                    }
        }

    pub fn new() ->Self{
        Self { cards:Vec::new() }
    }

    pub fn init_deck() -> Self{
        let mut cards:Vec<Card> = Vec::new();
        for suite in Suite::iter(){
            for (i ,face_value) in FaceValue::iter().enumerate(){
                cards.push(
                    Card::new(
       match u32::try_from(i){
                        Ok(i)=>i+1,
                        Err(_) => 0
                    }, 
                    face_value,
                    suite) );
            }
        }
        // display cards in deck
        // for card in &cards{
        //     println!("{:?}",card);
        // }    
        
        Self{
            cards,
        }
    }

    pub fn _show_deck(&mut self){
        println!("**** this deck has [ ");
        
        for (i,card) in self.cards.iter().enumerate(){
            print!("{} ",format!("{}",card));
            
            if i+1 < self.cards.len(){
                println!(", ");
            }else {
                println!(" ");
            }
        }
        println!(" ]")
    }
}