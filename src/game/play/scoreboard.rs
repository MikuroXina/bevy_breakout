use bevy::prelude::*;

use crate::game::exp::Scoreboard;

pub fn system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}
