use bevy::prelude::*;

use crate::game::exp::Paddle;

pub fn system(
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut trans) in query.iter_mut() {
        let mut dir = 0.0;

        if key.pressed(KeyCode::Left) {
            dir -= 1.0
        }
        if key.pressed(KeyCode::Right) {
            dir += 1.0
        }

        let place = &mut trans.translation;
        *place.x_mut() += time.delta_seconds * dir * paddle.speed;
        *place.x_mut() = place.x().min(380.0).max(-380.0);
    }
}
