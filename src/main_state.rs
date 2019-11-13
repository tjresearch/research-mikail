use legion::prelude::*;

use ggez::{
    input, GameResult, Context, graphics,
    input::{
        mouse::MouseButton, 
        keyboard::{KeyCode, KeyMods},
    },
    graphics::{DrawParam},
    event::{EventHandler},
};

use crate::{Point, Vector};

pub struct MainState {
    universe: Universe,
}

impl MainState {
    pub fn new(universe: Universe) -> Self{
        MainState{
            universe,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult{
        println!("hello world");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        ggez::graphics::present(ctx)
    }
}