use super::agent::Agent;
use super::game_definition::*;

extern crate rand;
use rand::prelude::*;
use std::{thread, time};

pub struct QLearningBot {
    q_table: Vec<Vec<f32>>,
    discount_factor: f32,
    learning_rate: f32,
    exploration_param: f32,
    rng: ThreadRng,
}

impl QLearningBot {
    pub const ACTION_LIST: [Action; 2] = [Action::Left, Action::Right];

    pub fn new(learning_rate: f32, discount_factor: f32, game: &Game) -> Self {
        let q_table = vec![vec![0f32; QLearningBot::ACTION_LIST.len()]; game.state.world.len()];
         QLearningBot {
            discount_factor,
            learning_rate,
            exploration_param: 1.0,
            q_table,
            rng: rand::thread_rng(),
         }
    }

    // we initalize with completely random values which means that initially there will be a lot of back and forth
    // but as the algorithm continues it should slowly fix the q_table to be increasing which will cause the player to move to the cheese
    pub fn initialize_q_table(&mut self) {
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

    // the idea is that we want to update the table cell with a linear mix between the current quality
    // and the guess for the quality based on what we currently got as a reward and what we expect as the best quality in the new state we move to
    // this linear mixing uses the learning_rate parameter and is needed for stochastic worlds where the reward may not always be the same
    // we also discount the future values the idea behind that being that we can tweak our bot to by myopic (short-sighted) where it cares more about current reward rather than future estimate    
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
        game.print_state();
        let timeout = time::Duration::from_millis(200);
        thread::sleep(timeout);

        self.exploration_param = 1.0 / game.runs as f32;
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
        let move_result = game.update(action.clone());
        let score_delta = game.score - old_score;
        self.update_q_table(old_position, game.player_pos, action, score_delta as f32);


        if (move_result == MoveResult::Win || move_result == MoveResult::Loss) && game.runs % 5 == 0 {
            for i in 0..self.q_table.len() {
                print!("({}-{}),", self.q_table[i][0], self.q_table[i][0]);
            }
            println!();
        }
    }
}