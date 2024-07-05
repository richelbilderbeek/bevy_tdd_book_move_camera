use crate::app::*;
use crate::game_parameters::*;
use bevy::prelude::*;
mod app;
mod game_parameters;

fn main() {
    let mut params = create_default_game_parameters();
    params.initial_camera_scale = 0.2;
    let mut app = create_app(params);
    app.add_plugins(DefaultPlugins);
    app.run();
}
