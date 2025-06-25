pub struct User {
    name: String,
    money:f32,
}

const INIT_MONEY:f32 = 10_000.0;

impl User {
    pub fn register (name:String)-> Self {
        Self{
            name, money:INIT_MONEY,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_money(&self) -> f32 {
        self.money
    }

    
}