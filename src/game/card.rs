pub struct Card {
    id: u8,
    text: String,
    rule: String,
    value: i16,
    collect: bool
}

impl Card {
    pub fn new(id: u8, text: &str, rule: &str, value: i16, collect: bool) -> Card {
        Card {
            id,
            text: text.to_string(),
            rule: rule.to_string(),
            value,
            collect
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}