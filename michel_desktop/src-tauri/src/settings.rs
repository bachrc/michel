use michel_core::MichelInstance;
use michel_index::MilliPersistence;
use serde::Serialize;

#[derive(Serialize)]
pub struct DisplayedPlugin {
    name: String,
}

#[tauri::command]
pub fn get_plugins_list(
    michel: tauri::State<MichelInstance<MilliPersistence>>,
) -> Vec<DisplayedPlugin> {
    michel
        .plugins()
        .iter()
        .map(|plugin| DisplayedPlugin {
            name: plugin.name(),
        })
        .collect()
}
