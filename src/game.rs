mod card;
mod deck;
mod player;

use super::utils as Utils;

use crate::game::player::Player as Player;
use crate::game::deck::Deck;

use std::{io, str::FromStr};

const ANTE_AMOUNT:f64 = 200.0;
pub struct Game {
    players : Vec<Player>,
    dealer_index : usize,
    money_in_pot: f64,
    deck: Deck,
    discarded_cards: Deck,
}

impl Game {
    pub fn init ()-> Self {
        let mut players: Vec<Player> = Vec::new();
        println!("initializing game....") ;
        
        println!("How many players wants to play? :");
       
        let number_of_players:u32 = Utils::get_user_input();


        for _i in 0..number_of_players{
            println!("Add Player: \nPlease Enter your name:" );

            let name:String = Utils::get_user_input();

            players.push(Player::register(name));

        }

    
        for player in &players{
            println!("{}: ${}",player.name(),player.money());
        }

        let deck = Deck::init_deck();
        let discarded_cards = Deck::new(); 
        Self{

            players,dealer_index:0,deck,money_in_pot:0.0, discarded_cards,
        }
    }

    pub fn start_game(&mut self){
        self.deck.shuffle();

        //todo! loop {
            self.get_ante();

            while self.money_in_pot > 0.0{

                
                self.deal_cards_to_players();
                self.start_round();
                
                
            }
            

                  // todo! loop}
            
    }

    fn discard_player_hand(&mut self,player_index: usize){
        let player = &mut self.players[player_index];
        while let Some(card) = player.discard_card_top() {
             self.discarded_cards.add_to_pile(card);
        }

       // shows that player's hand is empty
       // player.show_cards_on_hand();
        println!("{}'s hand was added to discard Pile ..",player.name());

        //shows
        //self.discarded_cards.show_deck();
    }

    fn start_round(&mut self){

        
        let mut current_player_index: usize = self.dealer_index;
        loop{

            self.player_play_hand(current_player_index);    

            current_player_index+=1;

            if current_player_index == self.players.len(){
                current_player_index=0;
            }

            if(self.money_in_pot < 0.0 || current_player_index == self.dealer_index){
                break;
            }
        }
        
    }

    

    
    fn player_play_hand(&mut self,player_index:usize){

        let player = &mut self.players[player_index];

        println!("{}'s turn. ",player.name());
        
        player.show_cards_on_hand();
        println!("option: \n1. Play.\n2. Fold.");

        loop {

            let option:u32 = Utils::get_user_input();
                  match option{
                1 => {

                },
                2 => {println!("{} folds. ",player.name())},
                _=> {println!("Invalid input. try again"); continue;}
            }

            //Discard player hand               
            self.discard_player_hand(player_index);
            

            break;
        }
        
    
       

    }

        
     



    fn deal_card_to_player(&mut self, player_index: usize){

        let player = &mut self.players[player_index];
        
        match self.deck.draw_card() {
            Some(card) => {
                player.take_card(card);
               // println!("Dealt card to {}: {:?}", player.name(), card);
            },
            None => {
                println!("No more cards in the deck, reshuffling discarded cards.");
                self.discarded_cards.shuffle();
                
                while let Some(card) = self.discarded_cards.draw_card() {
                    self.deck.add_to_pile(card);
                }
                
                if let Some(card) = self.deck.draw_card() {
                    player.take_card(card);
                    //println!("Dealt card to {}: {:?}", player.name(), card);
                } else {
                    println!("No cards left to deal.");
                }
            },
        }
    }

    fn deal_cards_to_players(&mut self){
          let mut current_player_index = self.dealer_index;

            loop{
                
                self.deal_card_to_player(current_player_index);
                self.deal_card_to_player(current_player_index);
                
                current_player_index+=1;
                if current_player_index == self.players.len(){
                    current_player_index = 0;

                }
                
                if current_player_index == self.dealer_index{
                    break;
                }

            }

/*      //show dealt hands 
        for player in &self.players{
                player.show_cards_on_hand();
        }
 */
        println!("\n*****Dealt cards to players.***\n");
 
    }
    
    
    pub fn get_ante(&mut self){

        for player in self.players.iter_mut(){
            player.pay_ante(ANTE_AMOUNT);
            
            self.money_in_pot+= ANTE_AMOUNT;
        }
       
        for player in &self.players{
            println!("{} has {}" , player.name(),player.money());
        }
        
    }

         
}