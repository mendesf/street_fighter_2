use std::collections::HashMap;

use bevy::prelude::*;

use crate::fighter::components::{Direction, Fighter, FighterState, Velocity};
use crate::sprite::{AnimationFrame, Animations, SpriteSheet};

pub fn spawn(
) -> impl FnMut(Commands, Res<AssetServer>, Query<&Window>, ResMut<Assets<TextureAtlas>>) {
    let frames = HashMap::from([
        (
            FighterState::Idle,
            vec![
                AnimationFrame { index: 0, time: 68 },
                AnimationFrame { index: 1, time: 68 },
                AnimationFrame { index: 2, time: 68 },
                AnimationFrame { index: 3, time: 68 },
            ],
        ),
        (
            FighterState::WalkForward,
            vec![
                AnimationFrame { index: 4, time: 65 },
                AnimationFrame { index: 5, time: 65 },
                AnimationFrame { index: 6, time: 65 },
                AnimationFrame { index: 7, time: 65 },
                AnimationFrame { index: 8, time: 65 },
                AnimationFrame { index: 9, time: 65 },
            ],
        ),
        (
            FighterState::WalkBackward,
            vec![
                AnimationFrame {
                    index: 10,
                    time: 65,
                },
                AnimationFrame {
                    index: 15,
                    time: 65,
                },
                AnimationFrame {
                    index: 16,
                    time: 65,
                },
                AnimationFrame {
                    index: 17,
                    time: 65,
                },
                AnimationFrame {
                    index: 18,
                    time: 65,
                },
                AnimationFrame {
                    index: 19,
                    time: 65,
                },
            ],
        ),
        (
            FighterState::JumpUp,
            vec![
                AnimationFrame {
                    index: 16,
                    time: 180,
                },
                AnimationFrame {
                    index: 17,
                    time: 100,
                },
                AnimationFrame {
                    index: 18,
                    time: 100,
                },
                AnimationFrame {
                    index: 19,
                    time: 100,
                },
                AnimationFrame {
                    index: 20,
                    time: 100,
                },
                AnimationFrame { index: 21, time: 0 },
            ],
        ),
        (
            FighterState::JumpForward,
            vec![
                AnimationFrame {
                    index: 22,
                    time: 200,
                },
                AnimationFrame {
                    index: 23,
                    time: 50,
                },
                AnimationFrame {
                    index: 24,
                    time: 50,
                },
                AnimationFrame {
                    index: 25,
                    time: 50,
                },
                AnimationFrame {
                    index: 26,
                    time: 50,
                },
                AnimationFrame {
                    index: 27,
                    time: 50,
                },
                AnimationFrame { index: 28, time: 0 },
            ],
        ),
        (
            FighterState::JumpBackward,
            vec![
                AnimationFrame {
                    index: 28,
                    time: 200,
                },
                AnimationFrame {
                    index: 27,
                    time: 50,
                },
                AnimationFrame {
                    index: 26,
                    time: 50,
                },
                AnimationFrame {
                    index: 25,
                    time: 50,
                },
                AnimationFrame {
                    index: 24,
                    time: 50,
                },
                AnimationFrame {
                    index: 23,
                    time: 50,
                },
                AnimationFrame { index: 22, time: 0 },
            ],
        ),
    ]);

    super::systems::spawn(
        SpriteSheet {
            path: "sprites/ryu.png".into(),
            size: Vec2::new(1748., 852.),
        },
        Animations {
            rects: vec![
                // Idle
                Rect::new(7., 14., 66., 104.),
                Rect::new(7., 14., 66., 104.),
                Rect::new(277., 11., 335., 103.),
                Rect::new(211., 10., 266., 103.),
                // Forward
                Rect::new(9., 136., 62., 219.),
                Rect::new(78., 131., 138., 219.),
                Rect::new(152., 128., 216., 220.),
                Rect::new(229., 130., 292., 220.),
                Rect::new(307., 128., 361., 219.),
                Rect::new(371., 128., 421., 217.),
                // Backward
                Rect::new(430., 124., 489., 214.),
                Rect::new(495., 124., 552., 214.),
                Rect::new(559., 124., 617., 214.),
                Rect::new(631., 125., 689., 216.),
                Rect::new(707., 126., 764., 215.),
                Rect::new(777., 128., 838., 215.),
                // JumpUp
                Rect::new(67., 244., 123., 348.),
                Rect::new(138., 233., 188., 322.),
                Rect::new(197., 233., 251., 310.),
                Rect::new(259., 240., 307., 310.),
                Rect::new(319., 234., 367., 323.),
                Rect::new(375., 244., 430., 353.),
                // Jump Forward/Backward
                Rect::new(67., 244., 123., 348.),
                Rect::new(442., 261., 503., 339.),
                Rect::new(507., 259., 611., 301.),
                Rect::new(617., 240., 670., 322.),
                Rect::new(676., 257., 798., 301.),
                Rect::new(804., 258., 875., 345.),
                Rect::new(883., 261., 937., 370.),
            ],
            frames,
        },
        FighterState::Idle,
        Velocity(Vec2::splat(0.)),
        Direction::Zero,
        false,
        Fighter::Ryu,
    )
}
