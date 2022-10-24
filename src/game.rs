use crate::browser;
use crate::engine;
use crate::engine::{Audio, Image, KeyState, Point, Rect, Renderer, SpriteSheet};
use crate::game_segments::{Obstacle, RedHatBoy};
use crate::segments::weird_platform_and_stone;
use crate::segments::{platform_and_stone, stone_and_platform};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::mpsc::UnboundedReceiver;
use rand::{thread_rng, Rng};
use std::rc::Rc;
use web_sys::HtmlImageElement;

pub const HEIGHT: i16 = 600;
const TIMELINE_MINIMUM: i16 = 1000;
const OBSTACLE_BUFFER: i16 = 20;

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, keystate: &KeyState);
    fn draw(&self, render: &Renderer);
}

pub struct Walk {
    boy: RedHatBoy,
    backgrounds: [Image; 2],
    obstacle_sheet: Rc<SpriteSheet>,
    obstacles: Vec<Box<dyn Obstacle>>,
    stone: HtmlImageElement,
    timeline: i16,
}

pub struct WalkTheDog {
    machine: Option<WalkTheDogStateMachine>,
}

enum WalkTheDogStateMachine {
    Ready(WalkTheDogState<Ready>),
    Walking(WalkTheDogState<Walking>),
    GameOver(WalkTheDogState<GameOver>),
}

struct WalkTheDogState<T> {
    _state: T,
    walk: Walk,
}
struct Ready;
struct Walking {
    // score: UnboundedReceiver<()>,
}
struct GameOver {
    new_game_event: UnboundedReceiver<()>,
}

impl Walk {
    fn new(
        boy: RedHatBoy,
        stone: HtmlImageElement,
        backgrounds: [Image; 2],
        obstacle_sheet: Rc<SpriteSheet>,
    ) -> Self {
        let starting_obstacles = stone_and_platform(stone.clone(), obstacle_sheet.clone(), 0);
        let timeline = rightmost(&starting_obstacles);
        Walk {
            boy,
            backgrounds,
            obstacles: starting_obstacles,
            obstacle_sheet,
            stone,
            timeline,
        }
    }

    fn reset(walk: Self) -> Self {
        Walk::new(
            RedHatBoy::reset(walk.boy),
            walk.stone,
            walk.backgrounds,
            walk.obstacle_sheet,
        )
    }

    fn generate_next_segment(&mut self) {
        let mut rng = thread_rng();
        let next_segment = rng.gen_range(0..3);

        let mut next_obstacles = match next_segment {
            0 => stone_and_platform(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            1 => platform_and_stone(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            2 => weird_platform_and_stone(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            _ => vec![],
        };
        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
    }

    fn draw(&self, renderer: &Renderer) {
        self.backgrounds.iter().for_each(|background| {
            background.draw(renderer);
        });
        self.boy.draw(renderer);
        self.obstacles.iter().for_each(|obstacle| {
            obstacle.draw(renderer);
        });
    }

    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }

    fn knocked_out(&self) -> bool {
        self.boy.knocked_out()
    }
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog { machine: None }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self.machine {
            None => {
                let sheet = browser::fetch_json("rhb_trimmed.json").await?;
                let sheet = serde_wasm_bindgen::from_value(sheet).unwrap();

                let audio = Audio::new()?;
                let music = audio.load_sound("background_song.mp3").await?;
                audio.play_looping_sound(&music, 0.1)?;

                let jump_sound = audio.load_sound("SFX_Jump_23.mp3").await?;
                let die_sound = audio.load_sound("die.wav").await?;
                let rhb = RedHatBoy::new(
                    sheet,
                    engine::image::load_image("rhb_trimmed.png").await?,
                    audio,
                    jump_sound,
                    die_sound,
                );

                let background = engine::image::load_image("BG.png").await?;
                let stone = engine::image::load_image("Stone.png").await?;

                let tiles = browser::fetch_json("tiles.json").await?;
                let tiles = serde_wasm_bindgen::from_value(tiles).unwrap();
                let sprite_sheet = Rc::new(SpriteSheet::new(
                    tiles,
                    engine::image::load_image("tiles.png").await?,
                ));

                let machine = WalkTheDogStateMachine::new(Walk::new(
                    rhb,
                    stone,
                    [
                        Image::new(background.clone(), Point { x: 0, y: 0 }),
                        Image::new(
                            background.clone(),
                            Point {
                                x: background.width() as i16,
                                y: 0,
                            },
                        ),
                    ],
                    sprite_sheet,
                ));

                Ok(Box::new(WalkTheDog {
                    machine: Some(machine),
                }))
            }
            Some(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(keystate));
        }
        assert!(self.machine.is_some());
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect::new_from_x_y(0, 0, 600, HEIGHT));
        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }
}

fn rightmost(obstacle_list: &[Box<dyn Obstacle>]) -> i16 {
    obstacle_list
        .iter()
        .map(|obstacle| obstacle.right())
        .max_by(|x, y| x.cmp(y))
        .unwrap_or(0)
}

impl WalkTheDogStateMachine {
    fn new(walk: Walk) -> Self {
        WalkTheDogStateMachine::Ready(WalkTheDogState::new(walk))
    }

    fn update(self, keystate: &KeyState) -> Self {
        match self {
            WalkTheDogStateMachine::Ready(state) => state.update(keystate).into(),
            WalkTheDogStateMachine::Walking(state) => state.update(keystate).into(),
            WalkTheDogStateMachine::GameOver(state) => state.update().into(),
        }
    }

    fn draw(&self, renderer: &Renderer) {
        match self {
            WalkTheDogStateMachine::Ready(state) => state.draw(renderer),
            WalkTheDogStateMachine::Walking(state) => state.draw(renderer),
            WalkTheDogStateMachine::GameOver(state) => state.draw(renderer),
        }
    }
}

impl GameOver {
    fn new_game_pressed(&mut self) -> bool {
        matches!(self.new_game_event.try_next(), Ok(Some(())))
    }
}

impl<T> WalkTheDogState<T> {
    fn draw(&self, renderer: &Renderer) {
        self.walk.draw(renderer);
    }
}

enum ReadyEndState {
    Complete(WalkTheDogState<Walking>),
    Continue(WalkTheDogState<Ready>),
}

impl WalkTheDogState<Ready> {
    fn new(walk: Walk) -> WalkTheDogState<Ready> {
        WalkTheDogState {
            _state: Ready,
            walk,
        }
    }

    fn update(mut self, keystate: &KeyState) -> ReadyEndState {
        self.walk.boy.update();
        if keystate.is_pressed("ArrowRight") {
            ReadyEndState::Complete(self.start_running())
        } else {
            ReadyEndState::Continue(self)
        }
    }

    fn start_running(mut self) -> WalkTheDogState<Walking> {
        //let score = browser::draw_ui("<div class='score' id='score'><p>0</p></div>")
        //    .and_then(|_unit| browser::find_html_element_by_id("score"))
        //    .map(|element| engine::button::add_click_handler(element))
        //    .unwrap();

        self.run_right();
        WalkTheDogState {
            _state: Walking {},
            walk: self.walk,
        }
    }

    fn run_right(&mut self) {
        self.walk.boy.run_right();
    }
}

enum WalkingEndState {
    Complete(WalkTheDogState<GameOver>),
    Continue(WalkTheDogState<Walking>),
}

impl WalkTheDogState<Walking> {
    fn update(mut self, keystate: &KeyState) -> WalkingEndState {
        if keystate.is_pressed("Space") {
            self.walk.boy.jump();
        }
        if keystate.is_pressed("ArrowDown") {
            self.walk.boy.slide();
        }
        self.walk.boy.update();

        let walking_speed = self.walk.velocity();

        let [first_background, second_background] = &mut self.walk.backgrounds;
        first_background.move_horizontally(walking_speed);
        second_background.move_horizontally(walking_speed);
        if first_background.right() < 0 {
            first_background.set_x(second_background.right());
        }
        if second_background.right() < 0 {
            second_background.set_x(first_background.right());
        }

        self.walk.obstacles.retain(|obstacle| obstacle.right() > 0);
        self.walk.obstacles.iter_mut().for_each(|obstacle| {
            obstacle.move_horizontally(walking_speed);
            obstacle.check_intersection(&mut self.walk.boy);
        });

        if self.walk.timeline < TIMELINE_MINIMUM {
            self.walk.generate_next_segment();
        } else {
            self.walk.timeline += walking_speed;
        }

        if self.walk.knocked_out() {
            WalkingEndState::Complete(self.end_game())
        } else {
            WalkingEndState::Continue(self)
        }
    }

    fn end_game(self) -> WalkTheDogState<GameOver> {
        let button = browser::draw_ui("<button id='new_game'>New Game</button>")
            .and_then(|_unit| browser::find_html_element_by_id("new_game"))
            .map(engine::button::add_click_handler)
            .unwrap();

        WalkTheDogState {
            _state: GameOver {
                new_game_event: button,
            },
            walk: self.walk,
        }
    }
}

enum GameOverEndState {
    Complete(WalkTheDogState<Ready>),
    Continue(WalkTheDogState<GameOver>),
}

impl WalkTheDogState<GameOver> {
    fn update(mut self) -> GameOverEndState {
        if self._state.new_game_pressed() {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }

    fn new_game(self) -> WalkTheDogState<Ready> {
        if let Err(err) = browser::hide_ui() {
            error!("Error hiding the browser {:#?}", err);
        }
        WalkTheDogState {
            _state: Ready,
            walk: Walk::reset(self.walk),
        }
    }
}

impl From<ReadyEndState> for WalkTheDogStateMachine {
    fn from(state: ReadyEndState) -> Self {
        match state {
            ReadyEndState::Complete(walking) => walking.into(),
            ReadyEndState::Continue(ready) => ready.into(),
        }
    }
}

impl From<WalkingEndState> for WalkTheDogStateMachine {
    fn from(state: WalkingEndState) -> Self {
        match state {
            WalkingEndState::Complete(game_over) => game_over.into(),
            WalkingEndState::Continue(walking) => walking.into(),
        }
    }
}

impl From<GameOverEndState> for WalkTheDogStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Complete(ready) => ready.into(),
            GameOverEndState::Continue(game_over) => game_over.into(),
        }
    }
}

impl From<WalkTheDogState<Ready>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Ready>) -> Self {
        WalkTheDogStateMachine::Ready(state)
    }
}

impl From<WalkTheDogState<Walking>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Walking>) -> Self {
        WalkTheDogStateMachine::Walking(state)
    }
}

impl From<WalkTheDogState<GameOver>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<GameOver>) -> Self {
        WalkTheDogStateMachine::GameOver(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{Sheet, Sound};
    use futures::channel::mpsc::unbounded;
    use std::collections::HashMap;
    use web_sys::{AudioBuffer, AudioBufferOptions};

    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    #[wasm_bindgen_test]
    fn test_transition_from_game_over_to_new_game() {
        let (_, receiver) = unbounded();
        let image = HtmlImageElement::new().unwrap();
        let audio = Audio::new().unwrap();
        let options = AudioBufferOptions::new(1, 3000.0);
        let sound = Sound {
            buffer: AudioBuffer::new(&options).unwrap(),
        };
        let rhb = RedHatBoy::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
            audio,
            sound.clone(),
            sound,
        );
        let sprite_sheet = SpriteSheet::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
        );
        let walk = Walk {
            boy: rhb,
            backgrounds: [
                Image::new(image.clone(), Point { x: 0, y: 0 }),
                Image::new(image.clone(), Point { x: 0, y: 0 }),
            ],
            obstacles: vec![],
            obstacle_sheet: Rc::new(sprite_sheet),
            stone: image.clone(),
            timeline: 0,
        };

        let document = browser::document().unwrap();
        document
            .body()
            .unwrap()
            .insert_adjacent_html("afterbegin", "<div id='ui'></div>")
            .unwrap();
        browser::draw_ui("<p>This is the UI</p>").unwrap();

        let state = WalkTheDogState {
            _state: GameOver {
                new_game_event: receiver,
            },
            walk: walk,
        };

        state.new_game();
        let ui = browser::find_html_element_by_id("ui").unwrap();
        assert_eq!(ui.child_element_count(), 0);
    }
}
