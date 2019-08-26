extern crate rand;
use rand::prelude::*;

pub fn test() {
    println!("this is a test");
}

pub trait Agent {    
    fn act(&mut self, game: &mut Game);
}

#[derive(Debug, Clone)]
pub enum TileType {
    Empty,
    Player,
    Pit,
    Cheese,    
}

#[derive(Debug)]
pub struct Game {
    score: i32,
    runs: u32,
    state: State,
    player_starting_pos: usize,
    player_pos: usize,
    quit: bool,
}

impl Game {
    pub fn new(mut state: State) -> Self {
        let player_pos = 5;
        state.world[player_pos] = TileType::Player;
        Game {
            runs: 0,
            score: 0,
            state,
            player_starting_pos: player_pos,
            player_pos,
            quit: false,
        }
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Invalid => println!("Invalid action"),
            Action::Left => {
                if self.player_pos == 0 {
                    println!("Player leaving board");
                    self.score = -1;
                } else {
                    self.state.world[self.player_pos] = TileType::Empty;
                    self.player_pos -= 1;
                    self.evaluate_new_pos();
                }
            },
            Action::Right => {
                if self.player_pos == self.state.world.len() - 1 {
                    println!("Player leaving board");
                    self.score = -1;
                } else {
                    self.state.world[self.player_pos] = TileType::Empty;
                    self.player_pos += 1;
                    self.evaluate_new_pos();
                }                
            },            
            Action::Quit => self.quit = true,
            _ => panic!("Unhandled action {:?}", action),
        }
    }

    fn evaluate_new_pos(&mut self) {
        match self.state.world[self.player_pos] {
            TileType::Cheese => {                                
                self.win();
            },
            TileType::Pit => {
                self.lose();
            },
            TileType::Empty => {
                self.state.world[self.player_pos] = TileType::Player;                
            },
            _ => {
                panic!("Unhandled tile type {:?}", self.state.world[self.player_pos]);
            },
        }                    
    }

    pub fn game_over(&self) -> bool {
        self.quit || self.score <= -5 || self.score >= 5
    }

    fn win(&mut self) {
        self.score += 1;
        self.runs += 1;
        self.reset();
        println!("You won!");
    }

    fn lose(&mut self) {     
        self.score += -1;
        self.runs += 1;
        self.reset();
        println!("You lost.");
    }

    fn reset(&mut self) {
        self.state.world[self.player_starting_pos] = TileType::Player;    
        self.player_pos = self.player_starting_pos;
    }
}

#[derive(Debug)]
pub struct State {
    pub world: Vec<TileType>,    
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Left,
    Right,
    Quit,
    Invalid,    
}

pub struct Human {

}

impl Human {    
    fn print_state(game: &Game) {
        print!("#");
        for tile in &game.state.world {
            match tile {
                TileType::Empty => print!("="),
                TileType::Player => print!("P"),
                TileType::Cheese => print!("C"),
                TileType::Pit => print!("O"),
            }
        }
        println!("# | Score: {} | Run: {}", game.score, game.runs);        
    }

    fn get_action() -> Action {
        use std::io;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.to_lowercase();
                if input.starts_with("a") {
                    return Action::Left;
                } else if input.starts_with("d") {
                    return Action::Right;
                } else if input.starts_with("q") {
                    return Action::Quit;
                } else {
                    return Action::Invalid;
                }
            },
            Err(error) => {
                panic!("Input error {}", error);
            },
        }
    }
}

impl Agent for Human {
    fn act(&mut self, game: &mut Game) {
        Self::print_state(game);
        let mut action = Action::Invalid;
        while action == Action::Invalid {
            println!("Type 'A' to move left, 'D' to move right, 'Q' else to quit, and then press 'Enter'.");
            action = Self::get_action();
        }
        game.update(action);
    }
}

pub struct QLearningBot {
    q_table: Vec<Vec<f32>>,
    discount_factor: f32,
    learning_rate: f32,
    exploration_param: f32,
    rng: ThreadRng,
}

impl QLearningBot {
    const ACTION_LIST: [Action; 2] = [Action::Left, Action::Right];

    fn initialize_q_table(&mut self) {
        for val in self.q_table.iter_mut() {
            for i in 0..val.len() {
                val[i] = self.rng.gen();
            }
        }
    }

    fn get_max_q_table_action(&self, player_pos: usize) -> (Action, f32) {
        let (mut max_action_id, mut max_action_val) = (0, 0.0);        
        for action_id in 0..self.q_table[player_pos].len() {
            if self.q_table[player_pos][action_id] > max_action_val {
                max_action_id = action_id;
                max_action_val = self.q_table[player_pos][action_id];
            }
        }
        (Self::ACTION_LIST[max_action_id].clone(), max_action_val)
    }

    fn update_q_table(&mut self, old_player_pos: usize, new_player_pos: usize, action: Action, score_delta: f32) {
        let action_id = Self::get_action_id(action);
        let old_state_quality = self.q_table[old_player_pos][action_id];
        let (_best_action, best_quality_in_future) = self.get_max_q_table_action(new_player_pos);        
        let reward = score_delta;
        let new_state_quality = (1.0 - self.learning_rate) * old_state_quality + self.learning_rate * (reward + self.discount_factor * best_quality_in_future);
        self.q_table[old_player_pos][action_id] = new_state_quality;
    }

    fn get_action_id(action: Action) -> usize {
        Self::ACTION_LIST.iter().position(|x| *x == action).expect("Must be a real action")
    }
}

impl Agent for QLearningBot {
    fn act(&mut self, game: &mut Game) {
        let roll: f32 = self.rng.gen();
        let action = if roll < self.exploration_param {
            let r_action = self.rng.gen_range(0, Self::ACTION_LIST.len());
            Self::ACTION_LIST[r_action].clone()
        } else {
            let (best_action, _) = self.get_max_q_table_action(game.player_pos);
            best_action
        };
        let old_score = game.score;
        let old_position = game.player_pos;
        game.update(action.clone());
        let score_delta = game.score - old_score;
        self.update_q_table(old_position, game.player_pos, action, score_delta as f32);
    }
}

pub mod AgentFactory {
    use super::*;

    pub fn human() -> Human {
        Human {}
    }

    pub fn bot(learning_rate: f32, discount_factor: f32, exploration_param: f32, game: &Game) -> QLearningBot {
        let q_table = vec![vec![0f32; QLearningBot::ACTION_LIST.len()]; game.state.world.len()];
        let mut bot = QLearningBot {
            discount_factor,
            learning_rate,
            exploration_param,
            q_table,
            rng: rand::thread_rng(),
        };
        bot.initialize_q_table();
        bot
    }
}