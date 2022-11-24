mod player;
mod place;
mod card;

// External creates
use rand::Rng;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Local imports
use player::Player;
use place::Place;
use card::Card;
use crate::tools::pl;

pub struct Game {
    id: u8,
    players: Vec<Player>,
    places: Vec<Place>,
    chances: Vec<Card>,
    communities: Vec<Card>,
    turn: Option<Player>
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();

        crate::tools::clear();
        pl("+--------------------------------------------------+");
        pl("|                                                  |");
        pl("|                     RUSTPOLY                     |");
        pl("|                                                  |");
        pl("|  Text Based Version of Monopoly Written in rust  |");
        pl("|        Version 1.0 (C)2022 Martti Natunen        |");
        pl("|                                                  |");
        pl("+--------------------------------------------------+");
        pl("");
        pl("");
        pl("                Press ENTER to start               ");

        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();

        Game {
            id: rng.gen(),
            players: vec![],
            places: vec![],
            chances: vec![],
            communities: vec![],
            turn: None
        }        
    }

    pub fn init(&mut self) {
        self.load_streets();
        self.load_chance_cards();
        self.load_community_cards();
        self.add_players();

        self.turn = &mut Option::Some(self.players[0]);

        while (true) {
            self.print_board();
            self.print_options();
        }
    }

    fn roll_dice(&self) {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..8);
    }

    fn buy(&self) {

    }
    
    fn end_round(&self) {

    }

    fn add_players(&mut self) {
        self.players = vec![];
        
        self.players.push(Player::new(0, "Player 1", 1500, true));
        self.players.push(Player::new(1, "Player 2", 1500, false));
        self.players.push(Player::new(2, "Player 3", 1500, false));
        self.players.push(Player::new(3, "Player 4", 1500, false));
    }

    fn load_streets(&mut self) {
        if let Ok(lines) = Game::read_lines("./data/streets.list") {
            for line in lines {
                if let Ok(row) = line {
                    let vec: Vec<&str> = row.trim().split("|").collect();
                    let place = Place::new(
                        vec[0].parse().unwrap(),
                        vec[1],
                        vec[2],
                        vec[3],
                        vec[4].parse().unwrap()
                    );

                    self.places.push(place);
                }
            }
        }
    }

    fn load_chance_cards(&mut self) {
        if let Ok(lines) = Game::read_lines("./data/chance_cards.list") {
            for line in lines {
                if let Ok(row) = line {
                    let vec: Vec<&str> = row.trim().split("|").collect();
                    let chance = Card::new(
                        vec[0].parse().unwrap(),
                        vec[1],
                        vec[2],
                        vec[3].parse().unwrap(),
                        vec[4] == "1"
                    );

                    self.chances.push(chance);
                }
            }
        }
    }

    fn load_community_cards(&mut self) {
        if let Ok(lines) = Game::read_lines("./data/community_cards.list") {
            for line in lines {
                if let Ok(row) = line {
                    let vec: Vec<&str> = row.trim().split("|").collect();
                    let community = Card::new(
                        vec[0].parse().unwrap(),
                        vec[1],
                        vec[2],
                        vec[3].parse().unwrap(),
                        vec[4] == "1"
                    );

                    self.communities.push(community);
                }
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn print_board(&self) {
        let mut input_string = String::new();
        let mut first_row: bool = true;
        let mut i = 0;

        crate::tools::clear();

        while i < 40 {
            if first_row {
                if i == 0 || i == 10 || i == 20 || i == 30 || i == 40 {
                    pl("+----+----+----+----+----+----+----+----+----+----+");
                }
    
                print!("| {} ", self.places[i].get_prop_type());
    
                if i == 9 || i == 19 || i == 29 || i == 39 {
                    pl("|");
                    first_row = false;
                    i += 1;
                    i -= 10;
                }
                else {
                    i += 1;
                }
            }
            else {
                print!("|");

                for j in 0..self.players.len() {
                    if i == self.players[j].get_position().into() { 
                        print!("{}", j + 1);
                    } else {
                        print!(" ");
                    }
                }

                if i == 9 || i == 19 || i == 29 || i == 39 {
                    pl("|");
                    first_row = true;
                }

                if i == 39 {
                    pl("+----+----+----+----+----+----+----+----+----+----+");
                }

                i += 1;
            }
        }

        io::stdin().read_line(&mut input_string).unwrap();
    }

    fn print_options(&self) {
        pl("");

        pl("1: Roll Dice");
        pl("2: Buy ");
        pl("4: End round");
        pl(&(self.turn.as_ref().expect("Current turn of player should not be empty").get_name().to_string() + ">"));

        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();

        self.do_option(&input_string);
    }

    fn do_option(&self, option: &str) {
        match option {
            "1" => self.roll_dice(),
            "2" => self.buy(),
            "3" => self.end_round(),
            _ => ()
        }
    }
}