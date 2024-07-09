use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let velocity = Vec2::new(0.2, 0.1);
    let mut app = create_app(velocity);
    app.add_plugins(DefaultPlugins);
    app.run();
}
