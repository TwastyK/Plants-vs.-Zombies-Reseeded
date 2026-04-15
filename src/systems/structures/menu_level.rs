use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::Component;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StaticWidget {
    pub id: String,
    pub w: u32,
    pub h: u32,
    pub x: f64,
    pub y: f64,
    pub im_path: String,
}

#[derive(Debug, Deserialize)]
pub struct DynamicWidget {
    pub id: String,
    pub w: u32,
    pub h: u32,
    pub x: f64,
    pub y: f64,
    pub im_path: String,
    pub im_hov_path: String,
}

#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    pub static_widgets: Vec<StaticWidget>,
    pub dynamic_widgets: Vec<DynamicWidget>,
}

#[derive(Component)]
pub struct GameButton {
    pub im_path: Handle<Image>,
    pub im_hov_path: Handle<Image>,
    pub id: String,
}
