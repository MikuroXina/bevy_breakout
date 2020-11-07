use bevy::prelude::*;

use crate::game::exp::Ball;

pub fn system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    let delta_seconds = time.delta_seconds.min(0.2);
    for (ball, mut trans) in query.iter_mut() {
        trans.translation += ball.velocity * delta_seconds;
    }
}
