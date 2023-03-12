use super::cell::*;
use super::player::*;
use super::settings::*;

use rusty_engine::prelude::*;

use std::path::PathBuf;

pub const BOARD_SIZE : f32 = CELL_SIZE * (CELL_PER_ROW as f32) + CELL_GAP * 2.0;
pub const BOARD_ORIGIN_X : f32 = -BOARD_SIZE / 2.0 + CELL_OFFSET;
pub const BOARD_ORIGIN_Y : f32 = BOARD_SIZE / 2.0 - CELL_OFFSET;

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Draw, 
    Winner(Player), 
}

#[derive(Debug, Clone, Copy)]
pub enum GameStatus {
    InProgress, 
    End(GameResult), 
}

#[derive(Debug)]
pub struct Board {
    pub data: Vec<Vec<Cell>>, 
    pub size: usize, 
    pub game_status: GameStatus, 
}


impl Board {
    pub fn data(&self) -> &Vec<Vec<Cell>> {
        return &self.data;
    }

    pub fn new(size: usize) -> Board {
        let mut board = Board {
            size, 
            data: Vec::new(), 
            game_status: GameStatus::InProgress, 
        };
        
        let mut x = BOARD_ORIGIN_X;
        let mut y = BOARD_ORIGIN_Y;
        for i in 0..size {
            board.data.push(Vec::new());
            for j in 0..size {
                let mut cell = Cell::new(format!("cell_{}{}", i, j), PathBuf::from(CELL_BORDER_SPRITE_PATH), Vec2::new(x, y));
                cell.value_sprite_label = format!("cell_value_{}{}", i, j);
                board.data.get_mut(i).unwrap().push(cell);
                x += CELL_SIZE + CELL_GAP;
            }
            y -= CELL_SIZE + CELL_GAP;
            x  = BOARD_ORIGIN_X;
        }

        board
    }   

    pub fn handle_click(&mut self, coordinate: Vec2, engine: &mut Engine, current_player: Player) -> bool {
        let mut flag = false;

        for inner in &mut self.data {
            for cell in inner {
                if cell.clicked_on(coordinate.clone()) {
                    if cell.occupy(current_player) {
                        let mut symbol = engine.add_sprite(cell.value_sprite_label.clone(), cell.value_sprite.clone());
                        symbol.layer = SYMBOL_LAYER;
                        symbol.translation = cell.coordinate.clone();
                        
                        flag = true;
                        break;
                    }
                } 
            }
        }

        if flag {
            self.update_game_status();
        }

        flag
    }

    pub fn is_occupied(&self) -> bool {
        for inner in self.data() {
            for cell in inner {
                if !cell.is_occupied() {
                    return false;
                }
            }
        }

        true
    }

    pub fn check_winner(&self) -> Option<Player> {
        let data = self.data();

        for row in data {
            if row[0].value() != CellValue::Empty {
                if row[0].value() == row[1].value() && row[1].value() == row[2].value() {
                    return Some(row[0].value().unwrap());
                }
            }
        }

        for col in 0..3 {
            if data[0][col].value() != CellValue::Empty {
                if data[0][col].value() == data[1][col].value() && data[1][col].value() == data[2][col].value() {
                    return Some(data[0][col].value().unwrap());
                }
            }
        }

        if data[0][0].value() != CellValue::Empty {
            if data[0][0].value() == data[1][1].value() && data[1][1].value() == data[2][2].value() {
                return Some(data[0][0].value().unwrap());
            }
        }

        if data[0][2].value() != CellValue::Empty {
            if data[0][2].value() == data[1][1].value() && data[1][1].value() == data[2][0].value() {
                return Some(data[0][2].value().unwrap());
            }
        }

        return None;
    }

    pub fn update_game_status(&mut self) {
        let winner = self.check_winner();

        match winner {
            None => {
                if self.is_occupied() {
                    self.game_status = GameStatus::End(GameResult::Draw);
                } 
            },
            Some(player) => self.game_status = GameStatus::End(GameResult::Winner(player)),
        }
    }

}   



