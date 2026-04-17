use bevy::sprite::Sprite;
use bevy::math::Vec2;

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
pub struct WidgetConfig {
    pub static_widgets: Vec<StaticWidget>,
    pub dynamic_widgets: Vec<DynamicWidget>,
}

#[derive(Component)]
pub struct GameButton {
    pub im_path: Handle<Image>,
    pub im_hov_path: Handle<Image>,
    pub id: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub size: Vec2,
    is_water: bool,
    is_roof: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    pub row: usize,
    pub col: usize,
}

pub struct ZombieManager {
    pub dollars: f32,
    zombie_vec: Vec<String>
}

pub struct Conveyr {
    pub seed: Seed
}

pub struct Seed {
    pub id: f32,
    pub image: Sprite
}

pub struct TimeManager {
    pub dt: f32,

}
