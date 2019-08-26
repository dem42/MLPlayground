extern crate ml_playground;

use ml_playground::q_learning::simple_q_learning::*;

fn main() {
    let mut human = AgentFactory::human();
    
    let mut world = vec![TileType::Empty; 16];
    world[0] = TileType::Pit;
    world[14] = TileType::Cheese;
    let state = State { world };

    let mut game = Game::new(state);

    loop {
        let action = human.act(&mut game);        
        if game.game_over() {
            break;
        }
    }
}
