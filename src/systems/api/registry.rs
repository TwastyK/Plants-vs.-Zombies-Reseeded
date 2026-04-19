use crate::systems::api::bridge_level_structure::WidgetConfig;
use rhai::{Engine, Scope};
use crate::systems::api::bridge_api_commands::BridgeCommands;

pub fn register_api(engine: &mut Engine) {
    engine.register_fn("load_yaml", |path: String| {
        BridgeCommands::LoadYaml {path: std::path::PathBuf::from(path)}
    })
}