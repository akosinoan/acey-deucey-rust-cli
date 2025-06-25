mod card;
mod deck;
use crate::user::User as User;
use crate::game::deck::Deck;


use std::io;

pub struct Game {
    players : Vec<User>,
    current_player_index : i32,
    money_in_pot: f32,
    deck: Deck,
}

impl Game {
    pub fn init ()-> Self {
        let mut players: Vec<User> = Vec::new();
        println!("initializing game....") ;
           
        loop{
            println!("Add Player: \nPlease Enter your name:");
            
            let mut name =  String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");
            players.push( User::register(name) );

            println!("Do you want to add more? : (1. yes, 2. no)");
            
            let mut option =  String::new();
            io::stdin()
                .read_line(&mut option)
                .expect("Failed to read line");
            let option: u32 = match option.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            
            match option {
                1=> {
                    continue;
                },
                _ => break,
            }
        } 
        for player in &players{
            println!("{}: ${}",player.get_name(),player.get_money());
        }

        let deck = Deck::init_deck();
        
        Self{

            players,current_player_index:0,deck,money_in_pot:0.0
        }
    }

    pub fn start_game(&mut self){
        self.deck.shuffle();


    }

         
}