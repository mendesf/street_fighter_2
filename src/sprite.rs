use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct SpriteSheet {
    pub path: String,
    pub size: Vec2,
}

#[derive(Component, Clone, Default)]
pub struct Animations<T>
where
    T: Component + std::hash::Hash,
{
    pub rects: Vec<Rect>,
    pub frames: HashMap<T, AnimationFrames>,
}

#[derive(Copy, Clone, Default)]
pub struct AnimationFrame {
    pub index: usize,
    pub time: u64,
}

type AnimationFrames = Vec<AnimationFrame>;

#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct AnimationTimer(pub Timer);

pub fn animate<T, U>(
    time: Res<Time>,
    mut query: Query<
        (
            &U,
            &Animations<U>,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<T>,
    >,
) where
    T: Component,
    U: Component + std::hash::Hash + Eq,
{
    for (current_state, animations, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let frames = animations.frames.get(current_state).unwrap();
            let inner_index = frames
                .iter()
                .enumerate()
                .find(|(_, frame)| frame.index == sprite.index)
                .map(|(index, _)| index)
                .unwrap();

            let frame = if frames[inner_index].time == 0 {
                frames[inner_index]
            } else if inner_index == frames.len() - 1 {
                frames[0]
            } else {
                frames[inner_index + 1]
            };

            sprite.index = frame.index;
            timer.set_duration(Duration::from_millis(frame.time));
        }
    }
}
