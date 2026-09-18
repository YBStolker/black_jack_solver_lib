use crate::models::game_state::GameState;
use crate::models::result_prediction::ResultPrediction;

pub struct GameStateSolver {
    game_state: GameState,
}

impl GameStateSolver {
    pub fn new(game_state: GameState) -> Self {
        GameStateSolver { game_state }
    }

    pub fn solve_game_state(&self) -> ResultPrediction {
        
        ResultPrediction::win()
    }
}
