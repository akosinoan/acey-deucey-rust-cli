use std::io;
mod user;
mod game;
use game::Game as Game;
use user::User as User;
fn main() {
    let mut players:Vec<User> = Vec::new();

    loop{
        println!("Main Menu: \n1. Start new game.\n2. Exit");
        let mut option =  String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
       
       match option {
        
        1=> {
            let mut game: Game = Game::init();
            
            game.start_game();
        },

        2=>break,

        _ => (),
       }
       
 

    }
    
}


