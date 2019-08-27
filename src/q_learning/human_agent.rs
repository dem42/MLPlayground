use super::game_definition::*;
use super::agent::Agent;

pub struct Human {}

impl Human {
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
        game.print_state();
        let mut action = Action::Invalid;
        while action == Action::Invalid {
            println!("Type 'A' to move left, 'D' to move right, 'Q' else to quit, and then press 'Enter'.");
            action = Self::get_action();
        }
        game.update(action);
    }
}
