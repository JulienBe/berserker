use bevy::prelude::{Res, Events, ResMut, With, Sprite};
use bevy::window::WindowResized;
use crate::scale::Scale;
use bevy::ecs::Query;
use bevy::transform::prelude::Transform;

pub(crate) fn resize_notificator(resize_event: Res<Events<WindowResized>>, mut scale: ResMut<Scale>, mut q: Query<&mut Transform, With<Sprite>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        scale.update(e.width, e.height)
    }
    for mut trans in q.iter_mut() {
        trans.scale.x = scale.g;
        trans.scale.y = scale.g;
        trans.translation.x = 0.0;
        trans.translation.y = 0.0;
    }
}
