pub mod audio;
pub mod button;
pub mod game;
pub mod image;
pub mod keys;
pub mod renderer;
pub mod sprites;

use crate::engine::sprites::SheetRect;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::{AudioBuffer, AudioContext, CanvasRenderingContext2d, HtmlImageElement};

pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f32,
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

pub struct Image {
    element: HtmlImageElement,
    bounding_box: Rect,
}

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
}

pub struct SpriteSheet {
    sheet: Sheet,
    image: HtmlImageElement,
}

#[derive(Deserialize, Clone)]
pub struct Sheet {
    pub frames: HashMap<String, Cell>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub frame: SheetRect,
    pub sprite_source_size: SheetRect,
}

#[derive(Default)]
pub struct Rect {
    pub position: Point,
    pub width: i16,
    pub height: i16,
}

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone)]
pub struct Audio {
    context: AudioContext,
}

#[derive(Clone)]
pub struct Sound {
    buffer: AudioBuffer,
}
