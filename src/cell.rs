use std::path::PathBuf;

use rusty_engine::prelude::*;

use super::player::Player;

// Sprite paths const
pub const CELL_BORDER_SPRITE_PATH : &str = "sprite/cell/cell.png";
pub const CELL_X_SPRITE_PATH : &str = "sprite/cell/x.png";
pub const CELL_O_SPRITE_PATH : &str = "sprite/cell/o.png";

// Cell related const
pub const CELL_SIZE : f32 = 130.0;
pub const CLICK_REGION_RADIUS : i32 = 60;
pub const CELL_GAP : f32 = 5.0;
pub const CELL_OFFSET: f32 = CELL_SIZE / 2.0;

pub const CELL_PER_ROW : i32 = 3;
pub const CELL_NUM : i32 = CELL_PER_ROW * CELL_PER_ROW;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellValue {
    Empty, 
    Occupied(Player),
}

impl CellValue {
    pub fn unwrap(&self) -> Player {
        match self {
            CellValue::Empty => panic!("Can not unwrap empty cell"),
            CellValue::Occupied(player) => *player, 
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    pub coordinate: Vec2,

    pub border_sprite_label: String, 
    pub border_sprite: PathBuf, 

    pub value_sprite_label: String, 
    pub value_sprite: PathBuf, 

    pub value: CellValue, 
}

impl Cell {
    pub fn value(&self) -> CellValue {
        self.value
    }

    pub fn new(border_sprite_label: String, border_sprite: PathBuf, coordinate: Vec2) -> Cell {
        Cell {
            border_sprite_label, 
            border_sprite, 
            coordinate, 
            value: CellValue::Empty, 
            value_sprite_label: String::new(), 
            value_sprite: PathBuf::new(), 
        }
    }

    pub fn clicked_on(&self, coordinate: Vec2) -> bool {
        let length = self.coordinate.distance(coordinate) as i32;

        length < CLICK_REGION_RADIUS
    }

    pub fn is_occupied(&self) -> bool {
        match self.value {
            CellValue::Empty => false, 
            _ => true, 
        }
    }


    /********************************************************************************
     * @brief: Try to occupy a cell with a player, if possible return true else false
     *
     ********************************************************************************/


    pub fn occupy(&mut self, player: Player) -> bool {
        if !self.is_occupied() {
            match player {
                Player::X => self.value_sprite = PathBuf::from(CELL_X_SPRITE_PATH),
                Player::O => self.value_sprite = PathBuf::from(CELL_O_SPRITE_PATH),
            }
            
            self.value = CellValue::Occupied(player);

            true 
        } else {
            false
        }
    }
}
