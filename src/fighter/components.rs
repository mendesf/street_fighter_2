use bevy::prelude::*;
use strum::EnumIter;

use crate::sprite::{AnimationTimer, Animations};

#[derive(Component, Default, Copy, Clone, PartialEq, Debug)]
pub enum Fighter {
    #[default]
    Ken,
    Ryu,
}

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum FighterState {
    #[default]
    Idle,
    WalkForward,
    WalkBackward,
    JumpUp,
    JumpForward,
    JumpBackward,
}

#[derive(Event, Debug)]
pub struct ChangeStateEvent {
    pub fighter: Fighter,
    pub new_state: FighterState,
}

#[derive(Event, Debug)]
pub struct JumpEvent {
    pub fighter: Fighter,
    pub velocity: Velocity,
}

#[derive(Component, Copy, Clone, Default, PartialEq, Debug, EnumIter)]
pub enum Direction {
    Up,
    Right,
    UpRight,
    #[default]
    Zero,
    UpLeft,
    Left,
}

impl Direction {
    pub fn sign(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0., 1.),
            Direction::Right => Vec2::new(1., 0.),
            Direction::UpRight => Vec2::new(1., 1.),
            Direction::Zero => Vec2::new(0., 0.),
            Direction::Left => Vec2::new(-1., 0.),
            Direction::UpLeft => Vec2::new(-1., 1.),
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::UpRight,
            Direction::UpRight => Direction::UpLeft,
            _ => *self,
        }
    }
}

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Velocity(pub Vec2);

// impl Velocity {
//     pub fn new(x: f32, y: f32) -> Self {
//         Self {
//             max: Vec2::new(x, y),
//             current: Vec2::new(x, y),
//         }
//     }
//
//     pub fn max(&self) -> Vec2 {
//         self.max
//     }
// }

#[derive(Bundle, Default)]
pub struct FighterBundle {
    pub sprite_sheet: SpriteSheetBundle,
    pub animations: Animations<FighterState>,
    pub current_state: FighterState,
    pub animation_timer: AnimationTimer,
    pub direction: Direction,
    pub velocity: Velocity,
    pub fighter: Fighter,
}
