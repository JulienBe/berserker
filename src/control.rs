use bevy::prelude::{Res, Input, KeyCode, Transform, Sprite, With, Vec2};
use crate::scale::Scale;
use bevy::ecs::Query;
use crate::{SubPixelPos, Player};

pub(crate) fn move_player(keys: Res<Input<KeyCode>>, scale: Res<Scale>, mut q: Query<(&mut Transform, &mut SubPixelPos, &Sprite), With<Player>>) {
    let mut dir = Vec2::new(0.0, 0.0);
    if keys.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }
    for (mut trans, mut spp, sprite) in q.iter_mut() {
        spp.x += dir.x;
        spp.y += dir.y;
        let pos = scale.translate_pos(&spp, &sprite);
        trans.translation.x = pos.0;
        trans.translation.y = pos.1;
    }
}
