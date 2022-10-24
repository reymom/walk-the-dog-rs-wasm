use crate::engine::{Audio, Cell, Image, Point, Rect, Renderer, Sheet, Sound, SpriteSheet};
use crate::game_state::red_hat_boy_states::RedHatBoyState;
use crate::game_state::{Event, RedHatBoyStateMachine};
use std::rc::Rc;
use web_sys::HtmlImageElement;

pub const LOW_PLATFORM: i16 = 420;
pub const FIRST_PLATFORM: i16 = 370;

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

pub trait Obstacle {
    fn check_intersection(&self, boy: &mut RedHatBoy);
    fn draw(&self, renderer: &Renderer);
    fn move_horizontally(&mut self, x: i16);
    fn right(&self) -> i16;
}

pub struct Platform {
    sheet: Rc<SpriteSheet>,
    bounding_boxes: Vec<Rect>,
    sprites: Vec<Cell>,
    position: Point,
}

pub struct Barrier {
    image: Image,
}

impl RedHatBoy {
    pub fn new(
        sheet: Sheet,
        image: HtmlImageElement,
        audio: Audio,
        jump_sound: Sound,
        die_sound: Sound,
    ) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new(
                audio, jump_sound, die_sound,
            )),
            sprite_sheet: sheet,
            image,
        }
    }

    pub fn update(&mut self) {
        self.state_machine = self.state_machine.clone().update();
    }

    pub fn reset(boy: Self) -> Self {
        RedHatBoy::new(
            boy.sprite_sheet,
            boy.image,
            boy.state_machine.context().audio.clone(),
            boy.state_machine.context().jump_sound.clone(),
            boy.state_machine.context().die_sound.clone(),
        )
    }

    pub fn draw(&self, renderer: &Renderer) {
        let sprite = self.current_sprite().expect("Cell not found");

        renderer.draw_image(
            &self.image,
            &Rect::new_from_x_y(
                sprite.frame.x,
                sprite.frame.y,
                sprite.frame.w,
                sprite.frame.h,
            ),
            &self.destination_box(),
        );
    }

    fn destination_box(&self) -> Rect {
        let sprite = self.current_sprite().expect("Cell not found");
        Rect::new_from_x_y(
            self.state_machine.context().position.x + sprite.sprite_source_size.x as i16,
            self.state_machine.context().position.y + sprite.sprite_source_size.y as i16,
            sprite.frame.w,
            sprite.frame.h,
        )
    }

    pub fn bounding_box(&self) -> Rect {
        const X_OFFSET: i16 = 18;
        const Y_OFFSET: i16 = 14;
        const WIDTH_OFFSET: i16 = 28;
        let mut bounding_box = self.destination_box();
        bounding_box.add_x(X_OFFSET);
        bounding_box.width -= WIDTH_OFFSET;
        bounding_box.add_y(Y_OFFSET);
        bounding_box.height -= Y_OFFSET;
        bounding_box
    }

    fn current_sprite(&self) -> Option<&Cell> {
        self.sprite_sheet.frames.get(&self.frame_name())
    }

    fn frame_name(&self) -> String {
        format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        )
    }

    pub fn run_right(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Run);
    }

    pub fn slide(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Slide);
    }

    pub fn jump(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Jump);
    }

    pub fn land_on(&mut self, y: i16) {
        self.state_machine = self.state_machine.clone().transition(Event::Land(y));
    }

    pub fn knock_out(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::KnockOut);
    }

    pub fn knocked_out(&self) -> bool {
        self.state_machine.knocked_out()
    }

    pub fn pos_y(&self) -> i16 {
        self.state_machine.context().position.y
    }

    pub fn velocity_y(&self) -> i16 {
        self.state_machine.context().velocity.y
    }

    pub fn walking_speed(&self) -> i16 {
        self.state_machine.context().velocity.x
    }
}

impl Obstacle for Platform {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if let Some(box_to_land_on) = self
            .bounding_boxes()
            .iter()
            .find(|&bounding_box| boy.bounding_box().intersects(bounding_box))
        {
            if boy.velocity_y() > 0 && boy.pos_y() < self.position.y {
                boy.land_on(box_to_land_on.y());
            } else {
                boy.knock_out();
            }
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let mut x = 0;
        self.sprites.iter().for_each(|sprite| {
            self.sheet.draw(
                renderer,
                &Rect::new_from_x_y(
                    sprite.frame.x,
                    sprite.frame.y,
                    sprite.frame.w,
                    sprite.frame.h,
                ),
                // Just use position and the standard widths in the tileset
                &Rect::new_from_x_y(
                    self.position.x + x,
                    self.position.y,
                    sprite.frame.w,
                    sprite.frame.h,
                ),
            );
            x += sprite.frame.w;
        });
    }

    fn move_horizontally(&mut self, x: i16) {
        self.position.x += x;
        self.bounding_boxes.iter_mut().for_each(|bounding_box| {
            bounding_box.set_x(bounding_box.position.x + x);
        });
    }

    fn right(&self) -> i16 {
        self.bounding_boxes()
            .last()
            .unwrap_or(&Rect::default())
            .right()
    }
}

impl Platform {
    pub fn new(
        sheet: Rc<SpriteSheet>,
        sprite_names: &[&str],
        bounding_boxes: &[Rect],
        position: Point,
    ) -> Self {
        let sprites = sprite_names
            .iter()
            .filter_map(|name| sheet.cell(name).cloned())
            .collect();

        let bounding_boxes = bounding_boxes
            .iter()
            .map(|bounding_box| {
                Rect::new_from_x_y(
                    bounding_box.x() + position.x,
                    bounding_box.y() + position.y,
                    bounding_box.width,
                    bounding_box.height,
                )
            })
            .collect();

        Platform {
            sheet,
            position,
            sprites,
            bounding_boxes,
        }
    }

    fn bounding_boxes(&self) -> &Vec<Rect> {
        &self.bounding_boxes
    }
}

impl Obstacle for Barrier {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if boy.bounding_box().intersects(self.image.bounding_box()) {
            boy.knock_out();
        }
    }

    fn draw(&self, renderer: &Renderer) {
        self.image.draw(renderer);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.image.move_horizontally(x);
    }

    fn right(&self) -> i16 {
        self.image.right()
    }
}

impl Barrier {
    pub fn new(image: Image) -> Self {
        Barrier { image }
    }
}
