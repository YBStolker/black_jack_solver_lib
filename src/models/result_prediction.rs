use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ResultPrediction {
    pub win_chance: f32,
    pub loss_chance: f32,
    pub tie_chance: f32,
}

impl ResultPrediction {
    pub fn win() -> Self {
        ResultPrediction {
            win_chance: 1_f32,
            loss_chance: 0_f32,
            tie_chance: 0_f32,
        }
    }
    pub fn loss() -> Self {
        ResultPrediction {
            win_chance: 0_f32,
            loss_chance: 1_f32,
            tie_chance: 0_f32,
        }
    }
    pub fn tie() -> Self {
        ResultPrediction {
            win_chance: 0_f32,
            loss_chance: 0_f32,
            tie_chance: 1_f32,
        }
    }

    pub fn add(&mut self, other: &ResultPrediction) {
        self.win_chance += other.win_chance;
        self.loss_chance += other.loss_chance;
        self.tie_chance += other.tie_chance;
    }

    pub fn normalize(&mut self) {
        let total = self.win_chance + self.loss_chance + self.tie_chance;
        if total > 0.0 {
            self.win_chance /= total;
            self.loss_chance /= total;
            self.tie_chance /= total;
        }
    }
}
