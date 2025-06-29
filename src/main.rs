mod game;
mod utils;
use game::Game as Game;

use crate::utils as Utils;
fn main() {

    loop{
        println!("Main Menu: \n1. Start new game.\n2. Exit");
        
        let option: u32 = Utils::get_user_input();
       
       match option {
        
        1=> {
            let mut game: Game = Game::init();
            
            game.start_game();
        },

        2=>break,

        _ => {println!("Invalid input. Try again. "); continue;},
       }
       
 

    }
    
}


