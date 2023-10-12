use bevy::prelude::*;

use crate::fighter::components::{Fighter, FighterState};
use crate::{sprite, GameState};

pub mod components;
pub mod ken;
pub mod ryu;
pub mod systems;

pub struct FighterPlugin;

impl Plugin for FighterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (ken::spawn(), ryu::spawn()))
            .add_systems(
                Update,
                (
                    sprite::animate::<Fighter, FighterState>,
                    draw_anchor,
                    systems::change_state,
                    systems::movement,
                    systems::movement_constraint.after(systems::movement),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn draw_anchor(mut gizmos: Gizmos, fighter: Query<&Transform, With<Fighter>>) {
    for transform in fighter.iter() {
        gizmos.line_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            Vec2::new(transform.translation.x + 10., transform.translation.y),
            Color::RED,
        );
        gizmos.line_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            Vec2::new(transform.translation.x, transform.translation.y + 10.),
            Color::RED,
        );
    }
}
