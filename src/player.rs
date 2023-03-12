#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    X, 
    O, 
}

impl Player {
    pub fn rotate(&mut self) {
        match self {
            Player::X => *self = Player::O, 
            Player::O => *self = Player::X,
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            Player::X => String::from("X"),
            Player::O => String::from("O"),
        }
    }
}


