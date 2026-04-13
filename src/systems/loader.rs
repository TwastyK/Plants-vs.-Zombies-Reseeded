use std::fs::File;
use std::path::Path;

use crate::systems::structures::level_structure::{LevelConfig, StaticWidget};
pub fn load_config(path: &Path) -> Result<LevelConfig, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let config: LevelConfig = serde_yaml::from_reader(file)?;

    Ok(config)
}