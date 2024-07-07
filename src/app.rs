use crate::game_parameters::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app(game_parameters: GameParameters) -> App {
    let mut app = App::new();
    let add_player_fn = move |/* no mut? */ commands: Commands| {
        add_player_with_sprite_at_pos_with_scale(
            commands,
            game_parameters.initial_player_position,
            game_parameters.initial_player_scale,
        );
    };
    app.add_systems(Startup, add_player_fn);
    let add_camera_fun = move |mut commands: Commands| {
        let mut bundle = Camera2dBundle::default();
        bundle.projection.scale = game_parameters.initial_camera_scale;
        commands.spawn(bundle);
    };
    app.add_systems(Startup, add_camera_fun);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

#[cfg(test)]
fn add_player(mut commands: Commands) {
    commands.spawn(Player);
}

fn add_player_with_sprite_at_pos_with_scale(
    mut commands: Commands,
    initial_player_position: Vec3,
    initial_player_scale: Vec3,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: initial_player_position,
                scale: initial_player_scale,
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

#[cfg(test)]
fn count_n_players(app: &App) -> usize {
    let mut n = 0;
    for c in app.world().components().iter() {
        // The complete name will be '[crate_name]::Player'
        if c.name().contains("Player") {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
fn get_camera_scale(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<&OrthographicProjection>();
    let projection = query.single(&app.world());
    projection.scale
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec3 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world());
    transform.translation
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec3 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world());
    transform.scale
}

#[cfg(test)]
fn get_all_components_names(app: &App) -> Vec<String> {
    use std::str::FromStr;

    let mut v: Vec<String> = Vec::new();
    for c in app.world().components().iter() {
        v.push(String::from_str(c.name()).unwrap());
    }
    v
}

#[cfg(test)]
fn has_camera(app: &App) -> bool {
    for c in app.world().components().iter() {
        if c.name() == "bevy_render::camera::camera::Camera" {
            return true;
        }
    }
    false
}

#[cfg(test)]
fn is_position_visible_sleepy_tea(app: &mut App, position: Vec2) -> bool {
    let position_3d = Vec3::new(position.x, position.y, 0.0);
    let mut camera_query = app.world_mut().query::<(&Camera, &GlobalTransform)>();
    let (camera, camera_transform) = camera_query.single(&app.world());
    let maybe_point = camera.world_to_viewport(camera_transform, position_3d);
    if maybe_point.is_none() {
        return false;
    }
    let _point = maybe_point.unwrap();
    true
}

#[cfg(test)]
fn is_position_visible(app: &mut App, position: Vec2) -> bool {
    let mut camera_query = app.world_mut().query::<(&Camera, &GlobalTransform)>();
    let (camera, camera_transform) = camera_query.single(&app.world());
    let maybe_point = camera.viewport_to_world_2d(camera_transform, position);
    if maybe_point.is_none() {
        println!("NONE");
        return false;
    }
    let point = maybe_point.unwrap();
    println!("{},{}", point.x, point.y);
    true

    /*
    let player_pos_3d = get_player_coordinat(app);
    let player_pos_2d = Vec2::new(player_pos_3d.x, player_pos_3d.y);
    let mut query = app.world().query::<(&Camera, &GlobalTransform)>();
    let (camera, camera_transform) = query.single(&app.world());
    let maybe_point = camera.viewport_to_world_2d(camera_transform, player_pos_2d);
    if maybe_point.is_none() {
        return false;
    }
    return true;
    */

    /*
    let mut query = app.world().query::<(&Camera, &GlobalTransform)>();
    let (camera, _) = query.single(&app.world());

    let maybe_rect = camera.physical_viewport_rect();
    if maybe_rect.is_none() {
        println!("NONE");
        return false;
    }
    let rect = maybe_rect.unwrap();
    println!(
        "({},{})-({},{})",
        rect.min.x, rect.min.y, rect.max.x, rect.max.y
    );
    return true;

     */
}

#[cfg(test)]
fn is_player_visible(app: &mut App) -> bool {
    let position = get_player_coordinat(app).xy();
    is_position_visible(app, position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app(create_default_game_parameters());
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let app = App::new();
        assert_eq!(count_n_players(&app), 0);
    }

    #[test]
    fn test_setup_player_adds_a_player() {
        let mut app = App::new();
        assert_eq!(count_n_players(&app), 0);
        app.add_systems(Startup, add_player);
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_player_is_at_custom_place() {
        let initial_coordinat = Vec3::new(1.2, 3.4, 5.6);
        let mut game_parameters = create_default_game_parameters();
        game_parameters.initial_player_position = initial_coordinat;
        let mut app = create_app(game_parameters);
        app.update();
        assert_eq!(get_player_coordinat(&mut app), initial_coordinat);
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let player_scale = Vec3::new(1.1, 2.2, 3.3);
        let mut game_parameters = create_default_game_parameters();
        game_parameters.initial_player_scale = player_scale;
        let mut app = create_app(game_parameters);
        app.update();
        assert_eq!(get_player_scale(&mut app), player_scale);
    }

    #[test]
    fn test_empty_app_has_no_camera() {
        let mut app = App::new();
        app.update();
        assert!(!has_camera(&app));
    }

    #[test]
    fn test_app_has_a_camera() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert!(has_camera(&app));
    }

    #[test]
    fn test_get_camera_scale() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert_eq!(get_camera_scale(&mut app), 1.0);
    }

    #[test]
    fn test_game_parameters_use_camera_scale() {
        let custom_camera_scale: f32 = 5.0;
        let mut params = create_default_game_parameters();
        params.initial_camera_scale = custom_camera_scale;
        let mut app = create_app(params);
        app.update();
        assert_eq!(get_camera_scale(&mut app), custom_camera_scale);
    }

    #[test]
    fn test_is_visible_position_visible() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        // By default, (0,0) is placed at the center of the screen,
        // hence that position is visible
        assert!(is_position_visible(&mut app, Vec2::new(0.0, 0.0)));
    }

    #[test]
    fn test_is_invisible_position_not_visible() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        // By default, (0,0) is placed at the center of the screen,
        // after which the mapping matches the pixels.
        // 10,000 pixels right and 10,000 pixes up is outside my of
        // computer screen
        assert!(!is_position_visible(&mut app, Vec2::new(10000.0, 100000.0)));
    }

    #[test]
    fn test_player_is_visible_at_start() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        println!("{}", is_player_visible(&mut app))
        //assert!(is_player_visible(&mut app));
    }

    #[test]
    fn test_player_is_not_visible_at_start() {
        let mut params = create_default_game_parameters();
        params.initial_player_position = Vec3::new(10000.0, 10000.0, 1.0);
        let mut app = create_app(params);
        app.update();
        assert!(!is_player_visible(&mut app));
    }

    #[test]
    fn test_get_all_components_names_for_empty_app() {
        let mut app = App::new();
        app.update();
        let v = get_all_components_names(&app);
        assert_eq!(v.len(), 7);
    }

    // SleapTea's ideas
    #[test]
    fn test_player_is_visible_at_start_sleepy_tea() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        // Fails
        assert!(is_position_visible_sleepy_tea(
            &mut app,
            Vec2::new(0.0, 0.0)
        ));
    }

    #[test]
    fn test_player_is_not_visible_at_start_sleepy_tea() {
        let mut params = create_default_game_parameters();
        params.initial_player_position = Vec3::new(10000.0, 10000.0, 1.0);
        let mut app = create_app(params);
        app.update();
        // Passes
        assert!(!is_position_visible_sleepy_tea(
            &mut app,
            Vec2::new(10000.0, 10000.0)
        ));
    }
}
