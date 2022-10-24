use crate::browser;
use crate::engine::keys::{prepare_input, process_input};
use crate::engine::{draw_frame_rate, GameLoop, KeyState, Renderer};
use crate::game::Game;

use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;

type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut keyevent_receiver = prepare_input()?;

        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut keystate = KeyState::new();
        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            process_input(&mut keystate, &mut keyevent_receiver);
            let frame_time = perf - game_loop.last_frame;
            game_loop.accumulated_delta += frame_time as f32;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&keystate);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            game_loop.last_frame = perf;

            game.draw(&renderer);

            if cfg!(debug_assertions) {
                unsafe { draw_frame_rate(&renderer, frame_time) }
            }

            if let Err(err) = browser::request_animation_frame(f.borrow().as_ref().unwrap()) {
                panic!("Error while requesting animation frame {:#?}", err);
            }
        }));
        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}
