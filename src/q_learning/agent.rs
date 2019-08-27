use super::game_definition::*;

pub trait Agent {    
    fn act(&mut self, game: &mut Game);
}

pub mod agent_factory {
    use super::*;
    use super::super::human_agent::Human;
    use super::super::q_learning_agent::QLearningBot;

    pub fn human() -> Human {
        Human {}
    }

    pub fn bot(learning_rate: f32, discount_factor: f32, game: &Game) -> QLearningBot {        
        let mut bot = QLearningBot::new(learning_rate, discount_factor, game);
        bot.initialize_q_table();
        bot
    }
}