use glam::{UVec2, Vec2};
pub use legion::*;
use raylib::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTransform {
    pub pos: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputControlled;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sprite {
    pub sample_pos: UVec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CColor {
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub hp: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wander;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChaseMouse;
