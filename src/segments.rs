use crate::engine::{Image, Point, Rect, SpriteSheet};
use crate::game_segments::{Barrier, Obstacle, Platform, FIRST_PLATFORM, LOW_PLATFORM};
use std::rc::Rc;
use web_sys::HtmlImageElement;

const STONE_ON_GROUND: i16 = 546;
const FLOATING_PLATFORM_SPRITES: [&str; 3] = ["13.png", "14.png", "15.png"];
const FLOATING_PLATFORM_BOUNDING_BOXES: [Rect; 3] = [
    Rect {
        position: Point { x: 0, y: 0 },
        width: 60,
        height: 54,
    },
    Rect {
        position: Point { x: 60, y: 0 },
        width: 384 - (60 * 2),
        height: 93,
    },
    Rect {
        position: Point { x: 384 - 60, y: 0 },
        width: 60,
        height: 54,
    },
];
const WATERY_PLATFORM_SPRITES: [&str; 3] = ["1.png", "18.png", "3.png"];
const WATERY_PLATFORM_BOUNDING_BOXES: [Rect; 3] = [
    Rect {
        position: Point { x: 0, y: 0 },
        width: 60,
        height: 54,
    },
    Rect {
        position: Point { x: 60, y: 0 },
        width: 384 - (60 * 2),
        height: 93,
    },
    Rect {
        position: Point { x: 384 - 60, y: 0 },
        width: 60,
        height: 54,
    },
];

pub fn stone_and_platform(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 150;
    vec![
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + INITIAL_STONE_OFFSET,
                y: STONE_ON_GROUND,
            },
        ))),
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: LOW_PLATFORM,
            },
        )),
    ]
}

pub fn platform_and_stone(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 150;
    vec![
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: STONE_ON_GROUND,
            },
        ))),
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + INITIAL_STONE_OFFSET,
                y: LOW_PLATFORM,
            },
        )),
    ]
}

pub fn weird_platform_and_stone(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 150;
    vec![
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: STONE_ON_GROUND,
            },
        ))),
        Box::new(create_weird_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + INITIAL_STONE_OFFSET,
                y: LOW_PLATFORM,
            },
        )),
    ]
}

fn create_floating_platform(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Platform {
    Platform::new(
        sprite_sheet,
        &FLOATING_PLATFORM_SPRITES,
        &FLOATING_PLATFORM_BOUNDING_BOXES,
        position,
    )
}

fn create_weird_floating_platform(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Platform {
    Platform::new(
        sprite_sheet,
        &WATERY_PLATFORM_SPRITES,
        &WATERY_PLATFORM_BOUNDING_BOXES,
        position,
    )
}
