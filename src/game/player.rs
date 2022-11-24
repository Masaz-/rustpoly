pub struct Player {
    id: u8,
    name: String,
    alive: bool,
    money: i16,
    human: bool,
    position: u8,
    moving: bool
}

impl Player {
    pub fn new(id: u8, name: &str, money: i16, human: bool) -> Player {
        Player {
            id,
            name: name.to_string(),
            alive: true,
            money,
            human,
            position: 0,
            moving: false
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn change_balance(&mut self, amount: i16) -> i16 {
        self.money += amount;
        self.money
    }

    pub fn get_position(&self) -> u8 {
        self.position
    }

    pub fn set_position(&mut self, position: u8) {
        self.position = position;
    }

    pub fn is_human(&self) -> bool {
        self.human
    }

    pub fn set_moving(&mut self, moving: bool) {
        self.moving = moving
    }

    pub fn is_moving(&self) -> bool {
        self.moving
    }
}