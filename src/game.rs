use bevy::prelude::*;

use self::exp::{setup, Scoreboard};
use self::play::{ball_collision, ball_move, paddle_move, scoreboard};

mod exp;
mod play;

pub struct Breakout;

impl Plugin for Breakout {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Scoreboard { score: 0 })
            .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_startup_system(setup.system())
            .add_system(paddle_move::system.system())
            .add_system(ball_collision::system.system())
            .add_system(ball_move::system.system())
            .add_system(scoreboard::system.system());
    }
}
