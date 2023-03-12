use rusty_engine::prelude::*;

use caro_game::board::*;
use caro_game::cell::*;
use caro_game::settings::*;
use caro_game::player::*;
use caro_game::game_state::*;

fn main() {
    let mut game = Game::new();
    game.window_settings(get_window_descriptor());
    let mut background = game.add_sprite("background", BACKGROUND_SPRITE_PATH);
    background.layer = BACKGROUND_LAYER;


    let mut title = game.add_text("title", "Caro Game");
    title.translation = Vec2::new(TITLE_TEXT_X, TITLE_TEXT_Y);

    let first_player = Player::X;

    let initial_game_state = GameState {
        board: Board::new(CELL_PER_ROW as usize), 
        current_player: first_player, 
    };

    let mut main_text = game.add_text(MAIN_TEXT_LABEL, format!("{}'s turn", first_player.get_text()));
    main_text.translation = Vec2::new(MAIN_TEXT_X, MAIN_TEXT_Y);

    game.audio_manager.play_music(BACKGROUND_MUSIC_PATH, BACKGROUND_VOLUMN);

    for inner in &initial_game_state.board.data {
        for cell in inner {
            let mut sprite = game.add_sprite(cell.border_sprite_label.clone(), cell.border_sprite.clone());
            sprite.layer = CELL_LAYER;
            sprite.translation = cell.coordinate.clone();
        }
    }

    game.add_logic(mouse_logic);
    game.run(initial_game_state);
}

fn mouse_logic(engine: &mut Engine, game_state: &mut GameState) {
    match game_state.board.game_status {
        GameStatus::InProgress => {
            if engine.mouse_state.just_pressed(MouseButton::Left) {
                if let Some(location) = engine.mouse_state.location() {
                    if game_state.board.handle_click(location, engine, game_state.current_player) {
                        game_state.current_player.rotate();
                        engine.texts.get_mut(MAIN_TEXT_LABEL).unwrap().value = format!("{}'s turn", game_state.current_player.get_text());
                        engine.audio_manager.play_sfx(SFX_PATH, SFX_VOLUMN);
                    }
                    
                }
            }
        },

        GameStatus::End(game_result) => {
            match game_result {
                GameResult::Draw => {
                    engine.texts.get_mut(MAIN_TEXT_LABEL).unwrap().value = "Draw".to_string();
                },
                GameResult::Winner(player) => {
                    engine.texts.get_mut(MAIN_TEXT_LABEL).unwrap().value = format!("{} has won the game", player.get_text());
                }
            }
        }
    }
    
    
}