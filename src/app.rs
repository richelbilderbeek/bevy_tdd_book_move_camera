use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovingCamera {
    velocity: Vec2,
}

pub fn create_app(velocity: Vec2) -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::input::InputPlugin);
    }
    let add_camera_fun = move |commands: Commands| add_moving_camera(commands, velocity);
    app.add_systems(Startup, (add_camera_fun, add_player));
    app.add_systems(Update, move_camera);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_moving_camera(mut commands: Commands, velocity: Vec2) {
    commands.spawn((
        Camera2dBundle::default(),
        MovingCamera { velocity: velocity },
    ));
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 1.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

#[cfg(test)]
fn get_camera_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &MovingCamera)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
fn count_n_moving_cameras(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&MovingCamera>();
    query.iter(app.world()).len()
}

fn move_camera(mut query: Query<(&mut Transform, &MovingCamera)>) {
    let (mut transform, moving_camera) = query.single_mut();
    transform.translation.x += moving_camera.velocity.x;
    transform.translation.y += moving_camera.velocity.y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        let velocity = Vec2::new(0.2, 0.1);
        create_app(velocity);
    }

    #[test]
    fn test_empty_app_has_no_moving_cameras() {
        let mut app = App::new();
        assert_eq!(count_n_moving_cameras(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_moving_camera() {
        let velocity = Vec2::new(0.0, 0.0);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(count_n_moving_cameras(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let velocity = Vec2::new(0.0, 0.0);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let velocity = Vec2::new(0.0, 0.0);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(64.0, 32.0));
    }

    #[test]
    fn test_camera_is_at_origin() {
        let velocity = Vec2::new(0.0, 0.0);
        let mut app = create_app(velocity);
        app.update();
        assert_eq!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_moving_camera_moves() {
        let velocity = Vec2::new(1.2, 3.4);
        let mut app = create_app(velocity);
        app.update();
        assert_ne!(get_camera_position(&mut app), Vec2::new(0.0, 0.0));
    }
}
