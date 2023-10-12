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
                    index: 11,
                    time: 65,
                },
                AnimationFrame {
                    index: 12,
                    time: 65,
                },
                AnimationFrame {
                    index: 13,
                    time: 65,
                },
                AnimationFrame {
                    index: 14,
                    time: 65,
                },
                AnimationFrame {
                    index: 15,
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
            path: "sprites/ken.png".into(),
            size: Vec2::new(2097., 4177.),
        },
        Animations {
            rects: vec![
                // Idle
                Rect::new(346., 688., 406., 777.),
                Rect::new(2., 687., 61., 777.),
                Rect::new(72., 685., 130., 777.),
                Rect::new(142., 684., 197., 777.),
                // Forward
                Rect::new(8., 872., 61., 955.),
                Rect::new(70., 867., 130., 955.),
                Rect::new(140., 866., 204., 956.),
                Rect::new(215., 865., 278., 954.),
                Rect::new(288., 866., 342., 955.),
                Rect::new(357., 867., 407., 956.),
                // Backward
                Rect::new(417., 868., 478., 955.),
                Rect::new(487., 866., 546., 956.),
                Rect::new(558., 865., 615., 955.),
                Rect::new(629., 864., 687., 954.),
                Rect::new(702., 865., 760., 956.),
                Rect::new(773., 866., 830., 955.),
                // JumpUp
                Rect::new(724., 1036., 780., 1140.),
                Rect::new(792., 995., 842., 1084.),
                Rect::new(853., 967., 907., 1044.),
                Rect::new(911., 966., 959., 1036.),
                Rect::new(975., 977., 1023., 1063.),
                Rect::new(1031., 1008., 1086., 1111.),
                // Jump Forward/Backward
                Rect::new(1237., 1037., 1292., 1140.),
                Rect::new(1301., 990., 1362., 1068.),
                Rect::new(1363., 994., 1467., 1036.),
                Rect::new(1468., 957., 1521., 1039.),
                Rect::new(1541., 988., 1663., 1032.),
                Rect::new(1664., 976., 1735., 1063.),
                Rect::new(1748., 977., 1803., 1080.),
            ],
            frames,
        },
        FighterState::Idle,
        Velocity(Vec2::splat(0.)),
        Direction::Zero,
        true,
        Fighter::Ken,
    )
}
