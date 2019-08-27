#[derive(Debug, Clone)]
pub enum TileType {
    Empty,
    Player,
    Pit,
    Cheese,    
}

#[derive(PartialEq)]
pub enum MoveResult {
    NextRound,
    Win,
    Loss,
    Quit,
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

#[derive(Debug)]
pub struct Game {
    pub score: i32,
    pub runs: u32,
    pub state: State,
    pub player_pos: usize,
    player_starting_pos: usize,
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

    pub fn update(&mut self, action: Action) -> MoveResult {
        match action {
            Action::Invalid => {
                println!("Invalid action");
                MoveResult::Loss
            },
            Action::Left => {
                if self.player_pos == 0 {
                    println!("Player leaving board");
                    self.score = -1;
                    MoveResult::Loss
                } else {
                    self.state.world[self.player_pos] = TileType::Empty;
                    self.player_pos -= 1;
                    self.evaluate_new_pos()
                }
            },
            Action::Right => {
                if self.player_pos == self.state.world.len() - 1 {
                    println!("Player leaving board");
                    self.score = -1;
                    MoveResult::Loss
                } else {
                    self.state.world[self.player_pos] = TileType::Empty;
                    self.player_pos += 1;
                    self.evaluate_new_pos()
                }                
            },            
            Action::Quit => {
                self.quit = true;
                MoveResult::Quit
            },
        }
    }

    fn evaluate_new_pos(&mut self) -> MoveResult {
        match self.state.world[self.player_pos] {
            TileType::Cheese => {                                
                self.win();
                MoveResult::Win
            },
            TileType::Pit => {
                self.lose();
                MoveResult::Loss
            },
            TileType::Empty => {
                self.state.world[self.player_pos] = TileType::Player;
                MoveResult::NextRound
            },
            _ => {
                panic!("Unhandled tile type {:?}", self.state.world[self.player_pos]);
            },
        }
    }

    pub fn print_state(&self) {
        print!("#");
        for tile in &self.state.world {
            match tile {
                TileType::Empty => print!("="),
                TileType::Player => print!("P"),
                TileType::Cheese => print!("C"),
                TileType::Pit => print!("O"),
            }
        }
        println!("# | Score: {} | Run: {}", self.score, self.runs);
    }

    pub fn game_over(&self) -> bool {
        self.quit || self.runs > 20
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