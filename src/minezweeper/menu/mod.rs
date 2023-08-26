mod buttons;
use crate::consts;
use buttons::Button;
use ggez::{graphics::{self, Canvas}, GameResult, Context};

use super::minezweeper::Level;

const LEVELS: [Level; 3] = [
    Level::Easy, Level::Medium, Level::Hard
];

pub struct Menu {
    buttons: [Button; 3]
}

pub trait ButtonSize {
    fn button_size(&self, button_width: f32, button_height: f32, horizontal_margin: f32, vertical_margin: f32) ->  graphics::Rect;
}

impl ButtonSize for Level {
    fn button_size(&self, button_width: f32, button_height: f32, horizontal_margin: f32, vertical_margin: f32) ->  graphics::Rect {
        match self {
            Self::Easy => {
                graphics::Rect::new(
                    horizontal_margin,
                    vertical_margin,
                    button_width,
                    button_height,
                )
            },
            Self::Medium => {
                graphics::Rect::new(
                    horizontal_margin,
                    2.0 * vertical_margin + button_height,
                    button_width,
                    button_height,
                )
            },
            Self::Hard => {
                graphics::Rect::new(
                    horizontal_margin,
                    3.0 * vertical_margin + 2.0 * button_height,
                    button_width,
                    button_height,
                )
            }
        }
    }
}

impl Menu {
    pub fn standard() -> Self {
        let (button_width, button_height) = consts::BUTTON_SIZE;
        let horizontal_margin = 0.5 * (consts::SCREEN_SIZE.0 - button_width);
        let vertical_margin = 0.25 * (consts::SCREEN_SIZE.1 - 3.0 * button_height);

        Menu {
            buttons: LEVELS.map(|level|
                Button::new(
                    level.level_info().name,
                    level.button_size(button_width, button_height, horizontal_margin, vertical_margin)
                )
            )
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for button in self.buttons.iter() {
            button.draw(ctx, canvas, graphics::DrawParam::default())?;
        }
        Ok(())
    }

    pub fn mouse_button_down_event(
        &mut self,
        x: f32,
        y: f32,
    ) {
        for button in self.buttons.iter_mut() {
            button.clicked = button.point_inside(x, y);
        }
    }

    pub fn mouse_button_up_event(
        &mut self,
        x: f32,
        y: f32,
    ) -> Option<&Level> {
        for (i, button) in self.buttons.iter().enumerate() {
            if button.point_inside(x, y) {
                return Some(&LEVELS[i]);
            }
        }
        None
    }

    pub fn mouse_motion_event(
        &mut self,
        x: f32,
        y: f32,
    ) {
        for button in self.buttons.iter_mut() {
            button.hovered = button.point_inside(x, y);
            if !button.hovered {
                button.clicked = false
            }
        }
    }
}