use bevy::prelude::*;

use crate::config::{GRAVITY, SCALE, STAGE_FLOOR};
use crate::fighter::components::{ChangeStateEvent, Direction, Fighter, FighterState, Velocity};
use crate::sprite::Animations;

pub fn movement(
    time: Res<Time>,
    mut fighter: Query<(&mut Transform, &Direction, &mut Velocity), With<Fighter>>,
) {
    for (mut transform, direction, mut velocity) in fighter.iter_mut() {
        let displacement = velocity.0 * SCALE * time.delta_seconds() * direction.sign();
        transform.translation.x += displacement.x;
        transform.translation.y += displacement.y;
        velocity.0.y -= GRAVITY * time.delta_seconds();
    }
}

pub fn movement_constraint(
    mut fighter: Query<(
        &mut Transform,
        &TextureAtlasSprite,
        &Animations<FighterState>,
        &Fighter,
        &Direction,
        &mut Velocity,
    )>,
    mut writer: EventWriter<ChangeStateEvent>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();

    for (mut transform, sprite, animations, fighter, direction, mut velocity) in fighter.iter_mut()
    {
        let window_offset =
            window.width() / 2. - animations.rects[sprite.index].size().x / 2. * SCALE;
        let mut send_idle_state_event = || {
            writer.send(ChangeStateEvent {
                fighter: *fighter,
                new_state: FighterState::Idle,
            });
        };
        if transform.translation.y < STAGE_FLOOR {
            transform.translation.y = STAGE_FLOOR;
            send_idle_state_event();
        }

        let sign = direction.sign();

        let touching_left_border = transform.translation.x < -window_offset && sign.x == -1.;
        let touching_right_border = transform.translation.x > window_offset && sign.x == 1.;

        if touching_left_border || touching_right_border {
            transform.translation.x = window_offset * sign.x;
            velocity.0.x = 0.;
            if transform.translation.y == STAGE_FLOOR {
                send_idle_state_event();
            }
        }
    }
}
