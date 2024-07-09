use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app(
    initial_camera_scale: f32,
    initial_player_position: Vec2,
    initial_player_size: Vec2,
) -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::input::InputPlugin);
        //app.add_plugins(bevy::window::WindowPlugin::default());
        //app.add_plugins(bevy::scene::ScenePlugin);
    }

    let add_player_fn = move |/* no mut? */ commands: Commands| {
        add_player(commands, initial_player_position, initial_player_size);
    };
    app.add_systems(Startup, add_player_fn);
    let add_camera_fun = move |mut commands: Commands| {
        let mut bundle = Camera2dBundle::default();
        bundle.projection.scale = initial_camera_scale;
        commands.spawn(bundle);
    };
    app.add_systems(Startup, (add_camera_fun, add_text));
    app.add_systems(Update, respond_to_mouse_move);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_text(mut commands: Commands) {
    commands.spawn((Text2dBundle {
        text: Text::from_section(String::new(), TextStyle { ..default() }),
        ..default()
    },));
}

fn add_player(mut commands: Commands, initial_player_position: Vec2, initial_player_scale: Vec2) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec2::extend(initial_player_position, 0.0),
                scale: Vec2::extend(initial_player_scale, 1.0),
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

fn coordinat_to_str(coordinat: Vec2) -> String {
    format!("({}, {})", coordinat[0], coordinat[1])
}

fn rect_to_str(r: Rect) -> String {
    format!("({}, {})-({}, {})", r.min.x, r.min.y, r.max.x, r.max.y)
}

fn urect_to_str(r: URect) -> String {
    format!("({}, {})-({}, {})", r.min.x, r.min.y, r.max.x, r.max.y)
}

#[cfg(test)]
fn get_camera_scale(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<&OrthographicProjection>();
    let projection = query.single(app.world());
    projection.scale
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec2 {
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

/*
#[cfg(test)]
fn get_all_components_names(app: &App) -> Vec<String> {
    use std::str::FromStr;

    let mut v: Vec<String> = Vec::new();
    for c in app.world().components().iter() {
        v.push(String::from_str(c.name()).unwrap());
    }
    v
}
*/

#[cfg(test)]
fn has_camera(app: &App) -> bool {
    for c in app.world().components().iter() {
        if c.name() == "bevy_render::camera::camera::Camera" {
            return true;
        }
    }
    false
}

/*
#[cfg(test)]
fn is_position_visible_sleepy_tea(app: &mut App, position: Vec2) -> bool {
    let position_3d = Vec3::new(position.x, position.y, 0.0);
    let mut camera_query = app.world_mut().query::<(&Camera, &GlobalTransform)>();
    let (camera, camera_transform) = camera_query.single(app.world());
    let maybe_point = camera.world_to_viewport(camera_transform, position_3d);
    if maybe_point.is_none() {
        return false;
    }
    let _point = maybe_point.unwrap();
    true
}
*/

fn is_position_visible_in_ortographic_projection_area(
    position: Vec2,
    projection: &OrthographicProjection,
) -> bool {
    projection.area.contains(position)
}

#[cfg(test)]
fn is_position_visible(app: &mut App, position: Vec2) -> bool {
    let mut camera_query = app
        .world_mut()
        .query::<(&Camera, &OrthographicProjection)>();
    let (_, projection) = camera_query.single(app.world());
    is_position_visible_in_ortographic_projection_area(position, projection)
}

#[cfg(test)]
fn is_player_visible(app: &mut App) -> bool {
    let position = get_player_coordinat(app).xy();
    is_position_visible(app, position)
}

fn respond_to_mouse_move(
    mut text_query: Query<&mut Text>,
    mut mouse_motion_event: EventReader<bevy::input::mouse::MouseMotion>,
    player_query: Query<(&Transform, &Player)>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &OrthographicProjection)>,
) {
    for _event in mouse_motion_event.read() {
        let line_cursor_pos: String;
        let (camera, projection) = camera_query.single();
        let maybe_cursor_pos = window_query.single().cursor_position();
        if maybe_cursor_pos.is_some() {
            let cursor_pos = maybe_cursor_pos.unwrap();
            line_cursor_pos = format!("cursor_pos: {}", coordinat_to_str(cursor_pos));
        } else {
            line_cursor_pos = "cursor_pos: none".to_string();
        }
        let line_logical_viewport_rect: String;

        let maybe_logical_viewport_rect = camera.logical_viewport_rect();
        //let maybe_logical_viewport_rect = projection.logical_viewport_rect();
        if maybe_logical_viewport_rect.is_some() {
            let logical_viewport_rect = maybe_logical_viewport_rect.unwrap();
            line_logical_viewport_rect = format!(
                "logical_viewport_rect: {}",
                rect_to_str(logical_viewport_rect)
            );
        } else {
            line_logical_viewport_rect = "No logical_viewport_rect".to_string();
        }
        // physical denotes actual screen pixels
        let line_physical_viewport_rect: String;
        let maybe_physical_viewport_rect = camera.physical_viewport_rect();
        if maybe_physical_viewport_rect.is_some() {
            let physical_viewport_rect = maybe_physical_viewport_rect.unwrap();
            line_physical_viewport_rect = format!(
                "physical_viewport_rect: {}",
                urect_to_str(physical_viewport_rect)
            );
        } else {
            line_physical_viewport_rect = "No physical_viewport_rect".to_string();
        }
        // player
        
        let player_pos = player_query.single().0.translation.xy();
        let line_player_pos: String = format!("player_pos: {}, {}", player_pos.x, player_pos.y);

        // projection
        let projection_area = projection.area;
        let line_projection_area = format!("projection_area: {}", rect_to_str(projection_area));
        let line_is_in = format!(
            "is_player_visible: {}",
            is_position_visible_in_ortographic_projection_area(player_pos, projection)
        );
        text_query.single_mut().sections[0].value = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            line_cursor_pos,
            line_player_pos,
            line_logical_viewport_rect,
            line_physical_viewport_rect,
            line_projection_area,
            line_is_in
        );
    }
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
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let app = App::new();
        assert_eq!(count_n_players(&app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_is_at_custom_place() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        assert_eq!(get_player_coordinat(&mut app), initial_player_position);
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );

        app.update();
        assert_eq!(get_player_scale(&mut app), initial_player_size);
    }

    #[test]
    fn test_app_has_a_camera() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        assert!(has_camera(&app));
    }

    #[test]
    fn test_get_camera_scale() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        assert_eq!(get_camera_scale(&mut app), 1.0);
    }

    #[test]
    fn test_game_parameters_use_camera_scale() {
        let initial_camera_scale = 12.34;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        assert_eq!(get_camera_scale(&mut app), initial_camera_scale);
    }

    #[test]
    fn test_is_visible_position_visible() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        // By default, (0,0) is placed at the center of the screen,
        // hence that position is visible
        assert!(is_position_visible(&mut app, Vec2::new(0.0, 0.0)));
        assert!(is_player_visible(&mut app));
    }

    #[test]
    fn test_is_invisible_position_not_visible() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        // By default, (0,0) is placed at the center of the screen,
        // after which the mapping matches the pixels.
        // 10,000 pixels right and 10,000 pixes up is outside my of
        // computer screen
        assert!(!is_position_visible(&mut app, Vec2::new(10000.0, 100000.0)));
    }

    #[test]
    fn test_player_is_visible_at_start() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        println!("{}", is_player_visible(&mut app))
        //assert!(is_player_visible(&mut app));
    }

    #[test]
    fn test_player_is_not_visible_at_start() {
        let initial_camera_scale = 1.0;
        let initial_player_position = Vec2::new(100000.0, 100000000.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(
            initial_camera_scale,
            initial_player_position,
            initial_player_size,
        );
        app.update();
        assert!(!is_player_visible(&mut app));
    }
}
