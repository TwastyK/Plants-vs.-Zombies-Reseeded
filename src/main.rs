use bevy::camera::ScalingMode;
use bevy::prelude::*;
use crate::systems::interaction_dispatcher::interact;
use crate::systems::level_dispatcher;

pub mod systems;
#[derive(Resource)]
pub struct LevelPath(pub String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PVZ Reseeded".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(LevelPath("assets/levels/menu.yaml".to_string()))
        .add_systems(Startup, (setup_camera, level_dispatcher::widget_dispatcher))
        .add_systems(Update, interact)
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 720.0, // Высота мира всегда 720 единиц
            },
            // Задаем стандартные плоскости отсечения для 2D
            ..OrthographicProjection::default_2d()
        }),
    ));
}