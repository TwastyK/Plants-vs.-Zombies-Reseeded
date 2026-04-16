use bevy::sprite::Sprite;
use bevy::math::Vec2;


pub struct Conveyr {
    pub seed: Seed
}

pub struct Seed {
    pub id: f32,
    pub image: Sprite
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
