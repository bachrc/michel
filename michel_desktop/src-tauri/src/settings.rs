use anyhow::{anyhow, Result};
use michel_core::MichelInstance;
use michel_index::MilliPersistence;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DisplayedPlugin {
    identifier: String,
    name: String,
    description: String,
    can_index: bool,
    enabled: bool,
}

#[derive(Deserialize)]
pub struct RunPluginIndex {
    identifier: String,
}

#[tauri::command]
pub fn get_plugins_list(
    michel: tauri::State<MichelInstance<MilliPersistence>>,
) -> Vec<DisplayedPlugin> {
    println!("wallah");
    michel
        .plugins()
        .iter()
        .map(|plugin| DisplayedPlugin {
            identifier: plugin.identifier(),
            name: plugin.name(),
            description: plugin.description(),
            can_index: plugin.can_index(),
            enabled: plugin.enabled(),
        })
        .collect()
}

#[tauri::command]
pub async fn run_plugin_index(
    identifier: String,
    michel: tauri::State<'_, MichelInstance<MilliPersistence>>,
) -> Result<String, String> {
    let plugin = michel.plugin(identifier).ok_or("deso".to_string())?;
    let result = plugin.index().await;

    Ok(String::from("okbro"))
}
