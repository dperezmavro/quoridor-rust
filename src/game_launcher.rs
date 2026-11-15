use crate::cli::QuoridorCli;
use crate::core::game_rounds_state::GameRoundsState;
use crate::game_loop;
use crate::save_file::SaveFileManager;
use crate::ui::ui_handler::UIHandler;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GameLauncher<T>
where
    T: UIHandler,
{
    pub q_cli: QuoridorCli,
    pub ui_handler: T,
    pub sfm: SaveFileManager,
    pub game_rounds_state: Rc<RefCell<GameRoundsState>>,
}

impl<T: UIHandler> GameLauncher<T>
where
    T: UIHandler,
{
    pub fn new(
        q_cli: QuoridorCli,
        ui_handler: T,
        sfm: SaveFileManager,
    ) -> GameLauncher<T> {
        GameLauncher {
            q_cli,
            ui_handler,
            sfm,
            game_rounds_state: Rc::new(RefCell::new(GameRoundsState::new())),
        }
    }

    pub fn setup(&mut self) {
        let grs: GameRoundsState;

        if let Some(file) = self.q_cli.load_file.clone() {
            println!("[+] Loading game from {file}");
            let sf = self.sfm.load_file(file.clone());
            grs = sf.to_game_round_state();
        } else {
            grs = GameRoundsState::new();
        }

        self.game_rounds_state = Rc::new(RefCell::new(grs));
    }

    pub async fn run(&mut self) {
        println!("[*] Game start!");
        let scores = self.game_rounds_state.borrow().scores();
        let winner_id =
            game_loop(self.game_rounds_state.clone(), &self.ui_handler, scores)
                .await;
        self.game_rounds_state.borrow_mut().record_win(winner_id);
        self.game_rounds_state.borrow_mut().new_game();
    }
}

impl<T> Drop for crate::GameLauncher<T>
where
    T: UIHandler,
{
    fn drop(&mut self) {
        println!(
            "Game launcher dropped, saving file: {}",
            self.sfm.save_file(self.game_rounds_state.clone())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::Player;
    use crate::core::player_turn::PLAYER_2_ID;
    use macroquad::color::Color;
    use macroquad::prelude::{TextParams, Vec2};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::io::Write;
    use std::option::Option;
    use vfs::MemoryFS;

    struct MockUiHandler {
        click_coordinates: RefCell<VecDeque<(f32, f32)>>,
    }

    impl UIHandler for MockUiHandler {
        async fn new() -> Self {
            Self {
                click_coordinates: RefCell::new(VecDeque::with_capacity(10)),
            }
        }

        fn draw_game_base(
            &self,
            _remaining_walls: (usize, usize),
            _scores: (usize, usize),
        ) {
        }

        fn mouse_position(&self) -> (f32, f32) {
            println!("popping location");
            let r = self.click_coordinates.borrow_mut().pop_front().unwrap();
            println!("returning {:?}", r);

            r
        }

        fn is_mouse_button_pressed(&self) -> bool {
            true
        }

        async fn next_frame(&self) {}

        fn draw_player(&self, _p: &Player, _highlight: bool) {}

        fn draw_rectangle(
            &self,
            _x: f32,
            _y: f32,
            _w: f32,
            _h: f32,
            _color: Color,
        ) {
        }

        fn draw_circle(&self, _x: f32, _y: f32, _r: f32, _color: Color) {}

        fn draw_text_ex(
            &self,
            _text: impl AsRef<str>,
            _x: f32,
            _y: f32,
            _params: TextParams,
        ) {
        }

        fn get_text_center(
            &self,
            _text: impl AsRef<str>,
            _font: Option<&macroquad::text::Font>,
            _font_size: u16,
            _font_scale: f32,
            _rotation: f32,
        ) -> macroquad::math::Vec2 {
            Vec2 { x: 0.0, y: 0.0 }
        }
    }

    const TILE_COORD_0: f32 = 75.0;
    const TILE_COORD_1: f32 = 150.0;
    const TILE_COORD_2: f32 = 230.0;
    const TILE_COORD_3: f32 = 320.0;
    const TILE_COORD_4: f32 = 375.0;
    const TILE_COORD_5: f32 = 460.0;
    const TILE_COORD_6: f32 = 520.0;
    const TILE_COORD_7: f32 = 600.0;
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_game_setup() {
        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (TILE_COORD_4, TILE_COORD_1), // p1 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p2 - y = 7
            (TILE_COORD_4, TILE_COORD_2), // p1 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p2 - y = 6
            (TILE_COORD_4, TILE_COORD_3), // p1 - y = 3
            (TILE_COORD_4, TILE_COORD_5), // p2 - y = 5
            (TILE_COORD_4, TILE_COORD_4), // p1 - y = 4
            (TILE_COORD_4, TILE_COORD_3), // p2 - y = 3
            (TILE_COORD_4, TILE_COORD_5), // p1 - y = 5
            (TILE_COORD_4, TILE_COORD_2), // p2 - y = 2
            (TILE_COORD_4, TILE_COORD_6), // p1 - y = 6
            (TILE_COORD_4, TILE_COORD_1), // p2 - y = 1
            (TILE_COORD_4, TILE_COORD_7), // p1 - y = 7
            (TILE_COORD_4, TILE_COORD_0), // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;

        let q_cli = crate::cli::QuoridorCli {
            save_file: None,
            load_file: None,
        };

        let sfm = SaveFileManager::new();
        let mut gl = crate::GameLauncher::new(q_cli, mui, sfm);
        gl.setup();
        gl.run().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_game_setup_from_save_file() {
        let mut mui = MockUiHandler::new().await;
        let click_coordinates = RefCell::new(VecDeque::from([
            (TILE_COORD_3, TILE_COORD_0), // p2 - y = 0
        ]));
        mui.click_coordinates = click_coordinates;

        let save_file = "/save.json";

        let q_cli = QuoridorCli {
            save_file: None,
            load_file: Some(String::from(save_file)),
        };

        let sfm = SaveFileManager {
            fs: MemoryFS::new().into(),
        };

        let state = String::from(
            r#"{"p1_score":0,"p2_score":0,"total_rounds":0,"current_player_id":1,"players":[{"starting_position":"Top","name":"p1","position":{"x":4,"y":1},"remaining_walls":10},{"starting_position":"Bottom","name":"p2","position":{"x":3,"y":1},"remaining_walls":10}],"walls":[]}"#,
        );

        sfm.fs
            .join(save_file)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(state.as_bytes())
            .unwrap();

        let mut gl = GameLauncher::new(q_cli, mui, sfm);
        gl.setup();
        assert_eq!(4, gl.game_rounds_state.borrow().game.p1.position.x);
        assert_eq!(1, gl.game_rounds_state.borrow().game.p1.position.y);

        assert_eq!(3, gl.game_rounds_state.borrow().game.p2.position.x);
        assert_eq!(1, gl.game_rounds_state.borrow().game.p2.position.y);

        assert_eq!(
            PLAYER_2_ID,
            gl.game_rounds_state.borrow().game.get_player_turn()
        );

        gl.run().await;
    }
}
