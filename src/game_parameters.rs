use bevy::prelude::*;

pub struct GameParameters {
    pub initial_camera_scale: f32,
    pub initial_player_position: Vec3,
    pub initial_player_scale: Vec3,
}

pub fn create_default_game_parameters() -> GameParameters {
    GameParameters {
        initial_camera_scale: 1.0,
        initial_player_position: Vec3::new(0.0, 0.0, 0.0),
        initial_player_scale: Vec3::new(100.0, 20.0, 1.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_game_parameters() {
        create_default_game_parameters();
    }
    #[test]
    fn test_initial_player_position() {
        assert_eq!(
            create_default_game_parameters().initial_player_position,
            Vec3::new(0.0, 0.0, 0.0)
        );
    }
    #[test]
    fn test_initial_player_scale() {
        assert_eq!(
            create_default_game_parameters().initial_player_scale,
            Vec3::new(100.0, 20.0, 1.0)
        );
    }
    #[test]
    fn test_initial_camera_scale() {
        assert_eq!(create_default_game_parameters().initial_camera_scale, 1.0);
    }
}
