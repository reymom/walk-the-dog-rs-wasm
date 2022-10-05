use crate::engine::{Cell, Renderer};
use crate::engine::{Point, Rect, Sheet, SpriteSheet};
use serde::Deserialize;
use web_sys::HtmlImageElement;

impl SpriteSheet {
    pub fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
        SpriteSheet { sheet, image }
    }

    pub fn cell(&self, name: &str) -> Option<&Cell> {
        self.sheet.frames.get(name)
    }

    pub fn draw(&self, renderer: &Renderer, source: &Rect, destination: &Rect) {
        renderer.draw_image(&self.image, source, destination);
    }
}

#[derive(Deserialize, Clone)]
pub struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

impl Rect {
    pub fn new(position: Point, width: i16, height: i16) -> Self {
        Rect {
            position,
            width,
            height,
        }
    }

    pub fn new_from_x_y(x: i16, y: i16, width: i16, height: i16) -> Self {
        Rect::new(Point { x, y }, width, height)
    }

    pub fn x(&self) -> i16 {
        self.position.x
    }

    pub fn set_x(&mut self, x: i16) {
        self.position.x = x
    }

    pub fn add_x(&mut self, x: i16) {
        self.position.x += x
    }

    pub fn y(&self) -> i16 {
        self.position.y
    }

    pub fn add_y(&mut self, y: i16) {
        self.position.y += y
    }

    pub fn intersects(&self, rect: &Rect) -> bool {
        self.x() < rect.right()
            && self.right() > rect.x()
            && self.y() < rect.bottom()
            && self.bottom() > rect.y()
    }

    pub fn right(&self) -> i16 {
        self.x() + self.width
    }

    pub fn bottom(&self) -> i16 {
        self.y() + self.height
    }
}
