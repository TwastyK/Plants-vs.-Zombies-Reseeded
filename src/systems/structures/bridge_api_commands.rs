use bevy::math::Vec2;

pub enum BridgeCommands {
    SpawnGrid {row: usize, col: usize, tile_size: Vec2, is_water: bool, is_roof: bool},
    SetWidget{x: f64, y: f64, z_index: f64, w: f64, h: f64, path: String},
    SunGeneration{sun: f32},
    TakePlant{id: f32},
    SetPlants{id: f32 ,row: usize, col: usize},
    SpawnEntity {x: f32, y: f32},
    SpawnZombie {row: usize, col: Option<usize>},
    MoveCamera {x: f32, y: f32},
    PlaySound(String),
}