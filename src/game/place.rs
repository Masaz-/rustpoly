use crate::game::player::Player;

pub struct Place {
    id: u8,
    name: String,
    prop_type: String,
    group: String,
    price: i16,
    houses: u8,
    owner: Option<Player>
}

impl Place {
    pub fn new(id: u8, name: &str, prop_type: &str, group: &str, price: i16) -> Place {
        Place {
            id,
            name: name.to_string(),
            prop_type: prop_type.to_string(),
            group: group.to_string(),
            price,
            houses: 0,
            owner: None
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_prop_type(&self) -> &str {
        &self.prop_type
    }

    pub fn get_group(&self) -> &str {
        &self.group
    }

    pub fn get_price(&self) -> i16 {
        self.price
    }

    pub fn get_owner(self) -> Option<Player> {
        self.owner
    }

    pub fn set_owner(&mut self, owner: Option<Player>) {
        self.owner = owner;
    }

    pub fn add_house(&mut self) {
        if self.houses < 5 {
            self.houses += 1;
        }
    }

    pub fn get_house_price(&self) -> u16 {
        if self.prop_type == "st" && self.houses < 5 {
            match self.group.as_str() {
                "brown"|"skyblue" => 50,
                "purple"|"orange" => 100, 
                "red"|"yellow" => 150,
                "green"|"blue" => 200,
                _ => 0
            }
        }
        else {
            0
        }
    }

    pub fn has_hotel(&self) -> bool {
        self.houses == 5
    }
}