use bevy::prelude::*;
use game::Breakout;

mod game;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Breakout)
        .run();
}
