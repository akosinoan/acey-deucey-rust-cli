use std::{io::{self, Read}, str::FromStr};


pub fn get_user_input<T: FromStr>() -> T
    where 
        <T as FromStr>::Err: std::fmt::Debug,
    {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed readline");

            match input.trim().parse::<T>(){
                Ok(value) => return value,
                Err(_) => {println!("Invalid input. Try again.");continue;},
            }
        }
}

pub fn pause(){
    //println!("............");
    let _ = io::stdin().read (&mut [0u8]).unwrap();
}