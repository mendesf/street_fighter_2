use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::config::{SCALE, STAGE_FLOOR};
use crate::fighter::components::{Direction, Fighter, FighterBundle, FighterState, Velocity};
use crate::sprite::{AnimationTimer, Animations, SpriteSheet};

pub fn spawn(
    sprite_sheet: SpriteSheet,
    animations: Animations<FighterState>,
    current_state: FighterState,
    velocity: Velocity,
    direction: Direction,
    flip_x: bool,
    fighter: Fighter,
) -> impl FnMut(Commands, Res<AssetServer>, Query<&Window>, ResMut<Assets<TextureAtlas>>) {
    move |mut commands: Commands,
          asset_server: Res<AssetServer>,
          window: Query<&Window>,
          mut texture_atlases: ResMut<Assets<TextureAtlas>>| {
        let window = window.single();
        let texture_handle = asset_server.load(sprite_sheet.path.clone());
        let mut texture_atlas = TextureAtlas::new_empty(texture_handle, sprite_sheet.size);

        for rect in &animations.rects {
            texture_atlas.add_texture(*rect);
        }

        let frames = animations.frames.get(&current_state).unwrap();
        let first_frame = frames[0];
        let mut sprite = TextureAtlasSprite::new(first_frame.index);
        sprite.flip_x = flip_x;
        sprite.anchor = Anchor::BottomCenter;

        let rect = &animations.rects[sprite.index];
        let sprite_size = rect.size();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn((FighterBundle {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite,
                transform: Transform::from_xyz(
                    (window.width() / 2. - sprite_size.x / 2. * SCALE)
                        * if flip_x { 1. } else { -1. },
                    STAGE_FLOOR,
                    0.,
                )
                .with_scale(Vec3::new(SCALE, SCALE, 0.)),
                ..default()
            },
            animations: animations.clone(),
            animation_timer: AnimationTimer(Timer::from_seconds(
                first_frame.time as f32 / 1000.,
                TimerMode::Repeating,
            )),
            current_state,
            direction,
            velocity,
            fighter,
        },));
    }
}
