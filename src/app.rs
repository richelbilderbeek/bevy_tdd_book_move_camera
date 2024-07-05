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

#[cfg(test)]
fn add_player_with_sprite(mut commands: Commands) {
    commands.spawn((SpriteBundle { ..default() }, Player));
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
    for c in app.world.components().iter() {
        // The complete name will be '[crate_name]::Player'
        if c.name().contains("Player") {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
fn get_camera_scale(app: &mut App) -> f32 {
    let mut query = app.world.query::<&OrthographicProjection>();
    let projection = query.single(&app.world);
    projection.scale
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec3 {
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    transform.translation
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec3 {
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    transform.scale
}

#[cfg(test)]
fn get_all_components_names(app: &App) -> Vec<String> {
    use std::str::FromStr;

    let mut v: Vec<String> = Vec::new();
    for c in app.world.components().iter() {
        v.push(String::from_str(c.name()).unwrap());
    }
    v
}

#[cfg(test)]
fn has_camera(app: &App) -> bool {
    for c in app.world.components().iter() {
        if c.name() == "bevy_render::camera::camera::Camera" {
            return true;
        }
    }
    false
}

#[cfg(test)]
fn print_all_components_names(app: &App) {
    for c in app.world.components().iter() {
        println!("{}", c.name())
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
    fn test_get_all_components_names_for_empty_app() {
        let mut app = App::new();
        app.update();
        let v = get_all_components_names(&app);
        assert_eq!(v.len(), 7);
    }

    #[test]
    fn test_get_all_components_names_for_our_app() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        let v = get_all_components_names(&app);
        assert_eq!(v.len(), 24);
    }

    #[test]
    fn test_print_names_of_empty_app() {
        let mut app = App::new();
        app.update();
        print_all_components_names(&app);
        /*
        bevy_ecs::schedule::schedule::Schedules
        bevy_ecs::reflect::AppTypeRegistry
        bevy_app::main_schedule::MainScheduleOrder
        bevy_app::main_schedule::FixedMainScheduleOrder
        bevy_ecs::event::Events<bevy_app::app::AppExit>
        */
    }

    #[test]
    fn test_print_names_of_app_with_player() {
        let mut app = App::new();
        app.add_systems(Startup, add_player);
        app.update();
        print_all_components_names(&app);
        /*
        // First 5 of empty App, then
        bevy_ecs::schedule::stepping::Stepping
        bevy_tdd_book_add_player_sprite::app::Player
        bevy_ecs::event::EventUpdateSignal
        */
    }

    #[test]
    fn test_print_names_of_app_with_player_sprite() {
        let mut app = App::new();
        app.add_systems(Startup, add_player_with_sprite);
        app.update();
        print_all_components_names(&app);
        /*
        // First 5 of empty App, then:
        bevy_ecs::schedule::stepping::Stepping // From Player
        bevy_sprite::sprite::Sprite
        bevy_transform::components::transform::Transform
        bevy_transform::components::global_transform::GlobalTransform
        bevy_asset::handle::Handle<bevy_render::texture::image::Image>
        bevy_render::view::visibility::Visibility
        bevy_render::view::visibility::InheritedVisibility
        bevy_render::view::visibility::ViewVisibility
        bevy_tdd_book_add_player_sprite::app::Player // From Player
        bevy_ecs::event::EventUpdateSignal // From Player
        */
    }
}
