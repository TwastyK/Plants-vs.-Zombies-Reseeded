use std::path::Path;
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{BackgroundColor, Button, Commands, ImageNode, Node, Query, Res, Transform};
use bevy::sprite::{Anchor, Sprite};
use bevy::ui::{Interaction, Val};
use bevy::utils::default;
use bevy::window::Window;
use crate::LevelPath;
use crate::systems::loader;
use crate::systems::structures::menu_level::GameButton;

pub fn widget_dispatcher(mut commands: Commands, path_pointer: Res<LevelPath>, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let path = Path::new(&path_pointer.0);
    let Ok(window) = windows.single() else {
        return; // Если окна нет, просто выходим из функции
    };
    let width = window.resolution.width();
    let height = window.resolution.height();

    let x_offset = width / 2.0;
    let y_offset = height / 2.0;

    match loader::load_config(path) {
        Ok(config) => {
            for widget in config.static_widgets {
                let texture_handle = asset_server.load(&widget.im_path);

                let x_pos = widget.x as f32 - x_offset;
                let y_pos = y_offset - widget.y as f32;
                commands.spawn((
                    Sprite {
                        image: texture_handle,
                        custom_size: Some(Vec2::new(widget.w as f32, widget.h as f32)),
                        ..default()
                    },
                    Anchor::TOP_LEFT,
                    Transform::from_xyz(
                       x_pos,
                       y_pos,
                       0.0
                    ),
                    ));
            }
        }
        Err(e) => {
            println!("Failed to load config: {}", e);
        }
    }

    match loader::load_config(path) {
        Ok(config) => {
            for widget in config.dynamic_widgets {
                let texture_handle = asset_server.load(&widget.im_path);
                let texture_hov_handle = asset_server.load(&widget.im_hov_path);

                commands.spawn((
                    Node {
                        left: Val::Px(widget.x as f32),
                        top: Val::Px(widget.y as f32),
                        width: Val::Px(widget.w as f32),
                        height: Val::Px(widget.h as f32),
                        ..default()
                    },
                    ImageNode::new(texture_handle.clone()),
                    Button,
                    Interaction::None,
                    GameButton { im_path: texture_handle, im_hov_path: texture_hov_handle, id: widget.id.clone() },
                    bevy::ui::RelativeCursorPosition::default(),
                ));
            }
        }
        Err(e) => {
            println!("Failed to load config: {}", e);
        }
    }
}
