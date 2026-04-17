use std::sync::mpsc;
use crate::systems::structures::bridge_api_commands::BridgeCommands;

pub struct BridgeApi {
    pub spawn_sender: mpsc::Sender<BridgeCommands>,
}