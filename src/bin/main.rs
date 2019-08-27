extern crate ml_playground;

use ml_playground::q_learning::*;

fn main() {
    let mut world = vec![TileType::Empty; 16];
    world[0] = TileType::Pit;
    world[14] = TileType::Cheese;
    let state = State { world };

    let mut game = Game::new(state);
    let mut agent = agent_factory::bot(0.2, 0.9, &game);

    loop {
        let action = agent.act(&mut game);        
        if game.game_over() {
            break;
        }
    }
}
