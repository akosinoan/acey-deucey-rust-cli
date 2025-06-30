mod card;
mod deck;
mod player;


use super::utils as Utils;

use crate::game::player::Player as Player;
use crate::game::deck::Deck;
use crate::utils::get_user_input;
use crate::game::card::Card as Card;
use std::cmp;

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

    

        let deck = Deck::init_deck();
        let discarded_cards = Deck::new(); 
        Self{

            players,dealer_index:0,deck,money_in_pot:0.0, discarded_cards,
        }
    }

    pub fn start_game(&mut self){
        self.deck.shuffle();

        Utils::pause();

        'start_game_loop:loop {
    
            for player in &self.players{
               println!("{} have ${}",player.name(),player.money());
            }
            self.get_ante();

            while self.money_in_pot > 0.0{

                
                
                self.deal_cards_to_players();
                Utils::pause();
                self.start_round();
                
                
            }
        println!("\n\n*****Pot Money is Empty.");

        for player in &mut self.players  {
            println!("{} have ${}",player.name(),player.money());

            while let Some(card) =  player.discard_card_top(){
               self.discarded_cards.add_to_pile(card);
            }
            
        }

        loop{
            println!("Do you want to play another round?\n1. Yes\n2. No");
            let option:u32 = get_user_input();
                match option {
                    1 => continue 'start_game_loop,
                    2 => break 'start_game_loop ,
                    _ => {println!("Invalid input"); continue;}
                    
                }
            }
        }
            
    }

    fn start_round(&mut self){

        
        let mut current_player_index: usize = self.dealer_index;
        loop{
            
            if self.players[current_player_index].is_ante_paid() {
                self.player_play_hand(current_player_index);    
            }
            current_player_index+=1;

            if current_player_index == self.players.len(){
                current_player_index=0;
            }

            if self.money_in_pot <= 0.0 || current_player_index == self.dealer_index{
                break;
            }
        }
        self.dealer_index+=1;
        if self.dealer_index >= self.players.len(){
            self.dealer_index=0;
        }
        
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


    
    fn player_play_hand(&mut self,player_index:usize){

        let player = &self.players[player_index];

        println!("\n\n{}'s turn. ",player.name());
        println!("{} have ${}",player.name(),player.money(), );
        println!("Money in the pot is ${}",self.money_in_pot);
        player.show_cards_on_hand();
        println!("option: \n1. Play.\n2. Fold.");

        loop {

            let option:u32 = Utils::get_user_input();
            
            match option{
                1 => {
                    let mut bet_amount:f64 = 0.0;
                    
                    let dealer_card:Card =match self.deck.draw_card(){
                        Some(card) => card,
                        None=> {
                            self.refill_deck_from_discarded_cards(); 
                            
                            if let Some(card) = self.deck.draw_card() {
                               card
                                //println!("Dealt card to {}: {:?}", player.name(), card);
                            } else {
                                panic!("No cards to deal");
                            }
                        },
                    };
                    let player = &mut self.players[player_index];
                   
                    while bet_amount <= 0.0 
                        || bet_amount > self.money_in_pot  
                        || bet_amount > player.money(){
                            println!("How much do you want to bet?\n [ 1 - {}]:" , 
                            {
                                if self.money_in_pot < player.money(){
                                self.money_in_pot 
                                }else{
                                    player.money()
                                }

                            });

                                bet_amount = get_user_input();
                            
                            if bet_amount <= 0.0{
                                println!("Invalid amount.");
                            }
                            if bet_amount > self.money_in_pot{
                                println!("Invalid amount: cannot be larger than money in pot [${}]",self.money_in_pot );
                            }
                            if bet_amount > player.money(){
                                println!("Invalid amount: Insufficient money. you only have [${}]",player.money());
                            }
                    }

                    println!("\n\n");
                    player.show_cards_on_hand(); 
                    println!("Dealer drew [{}] ",format!("{}",dealer_card));
                    let is_player_win:bool;
                    //check if pairs
                    if player.peek_card(0).number_value() == player.peek_card(1).number_value() {
                        
                        loop{
                        println!("You have a pair! Choose if dealer card is higher or lower than your cards.\n1. Higher.\n2. Lower:");

                        let option:u32 = get_user_input();
                            match option {
                                1 | 2 =>{
                                    if dealer_card.number_value() > player.peek_card(0).number_value() && option == 1  {
                                        is_player_win = true;
                                    }else if dealer_card.number_value() < player.peek_card(0).number_value() && option == 2  {
                                        is_player_win = true
                                    }else {
                                        is_player_win = false;
                                    }
                                    break;
                                },
                                
                                _=>continue,
                            }
                        }


                    }else{
                        let player_card_max_value = cmp::max(player.peek_card(0).number_value(),player.peek_card(1).number_value());
                        let player_card_min_value = cmp::min(player.peek_card(0).number_value(), player.peek_card(1).number_value());

                        if dealer_card.number_value() < player_card_max_value && dealer_card.number_value() > player_card_min_value {
                            is_player_win= true;
                        }else {
                            is_player_win= false;
                        }
                    }

                    if is_player_win {

                        
                        println!("{} won !",player.name());
                        self.money_in_pot-= bet_amount;
                        player.add_money(bet_amount);

                    }else{
                        println!("{} lost! ",player.name());
                        player.subtract_money(bet_amount);
                        println!("adding {} to pot money! ",bet_amount);
                        self.money_in_pot+= bet_amount;
                    }

                }, //option 1 play
                2 => {println!("{} folds. ",self.players[player_index].name())},
                _=> {println!("Invalid input. try again"); continue;}
            }

            //Discard player hand               
            self.discard_player_hand(player_index);
            
            Utils::pause();
            break;
        }
        
    
       

    }

        
     
    fn refill_deck_from_discarded_cards(&mut self){
        println!("No more cards in the deck, reshuffling discarded cards.");
        self.discarded_cards.shuffle();
        
        while let Some(card) = self.discarded_cards.draw_card() {
            self.deck.add_to_pile(card);
        }
    }


    fn deal_card_to_player(&mut self, player_index: usize){

        
        match self.deck.draw_card() {
            Some(card) => {
                let player = &mut self.players[player_index];
                player.take_card(card);
               // println!("Dealt card to {}: {:?}", player.name(), card);
            },
            None => {
                self.refill_deck_from_discarded_cards();
                
                if let Some(card) = self.deck.draw_card() {
                    let player = &mut self.players[player_index];
                    player.take_card(card);
                    //println!("Dealt card to {}: {:?}", player.name(), card);
                } else {
                    panic!("No cards to deal");
                }
            },
        }
    }

    fn deal_cards_to_players(&mut self){
          let mut current_player_index = self.dealer_index;


          
            loop{
                if self.players[current_player_index].is_ante_paid(){
                    self.deal_card_to_player(current_player_index);
                    self.deal_card_to_player(current_player_index);
                
                }
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
            println!("{} have ${}" , player.name(),player.money());
        }
        
    }

         
}