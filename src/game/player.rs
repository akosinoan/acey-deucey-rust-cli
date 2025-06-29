
use crate::game::{card::{Card, FaceValue, Suite}, player};
pub struct Player {
    name: String,
    money:f64,
    is_ante_paid:bool,
    cards_on_hand:Vec<Card>,
}

const INIT_MONEY:f64 = 10_000.0;

impl Player {
    pub fn register (name:String)-> Self {
        Self{
            name, money:INIT_MONEY,is_ante_paid:false,cards_on_hand: Vec::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn money(&self) -> f64 {
        self.money
    }

    pub fn subtract_money (&mut self,amount:f64)-> bool {

        if(self.money >= amount){
            self.money-=amount;
            true
        }else{
            println!("{} has insufficient money.",self.money);
            false
        }

    }

    pub fn take_card(&mut self,card:Card){
        self.cards_on_hand.push(card);
    }

    pub fn discard_card_top(&mut self)-> Option<Card>{
        self.cards_on_hand.pop()
    }

    pub fn show_cards_on_hand(&self){
        print!("{} has [ ",self.name);
        
        for (i,card) in self.cards_on_hand.iter().enumerate(){
            print!("{} ",format!("{}",card));
            
            if i+1 < self.cards_on_hand.len(){
                print!(", ");
            }
        }
        println!(" ]")
    }

    pub fn is_ante_paid (&self) -> bool{
        self.is_ante_paid
    }

    pub fn pay_ante(&mut self,amount:f64){

        if(self.subtract_money(amount)){
            println!("{} paid {} for ante.",self.name , amount);
            self.is_ante_paid = true;

        }else {
            self.is_ante_paid = false;
        }
    }
    
}