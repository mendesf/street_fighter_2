use bevy::prelude::*;

use crate::fighter::components::{ChangeStateEvent, Direction, Fighter, FighterState, Velocity};
use crate::sprite::Animations;

pub fn change_state(
    mut reader: EventReader<ChangeStateEvent>,
    mut fighter: Query<(
        &mut FighterState,
        &mut TextureAtlasSprite,
        &mut Direction,
        &mut Velocity,
        &Fighter,
        &Animations<FighterState>,
    )>,
) {
    for event in reader.iter() {
        for (mut current_state, mut sprite, mut direction, mut velocity, fighter, animations) in
            fighter.iter_mut()
        {
            info!("changing state {:?}", *event);

            if event.fighter != *fighter || *current_state == event.new_state {
                continue;
            }

            *current_state = event.new_state;
            if let Some(frames) = animations.frames.get(&current_state) {
                sprite.index = frames[0].index;
            }

            (*direction, velocity.0) = match *current_state {
                FighterState::Idle => (Direction::Zero, Vec2::new(0., 0.)),
                FighterState::WalkForward => (Direction::Right, Vec2::new(200., 0.)),
                FighterState::WalkBackward => (Direction::Left, Vec2::new(150., 0.)),
                FighterState::JumpUp => (Direction::Up, Vec2::new(0., 420.)),
                FighterState::JumpForward => (Direction::UpRight, Vec2::new(170., 420.)),
                FighterState::JumpBackward => (Direction::UpLeft, Vec2::new(200., 420.)),
            };

            if sprite.flip_x {
                *direction = direction.flip();
            }
        }
    }
}
