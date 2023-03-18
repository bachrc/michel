#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lazy_static::lazy_static;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs, process};

use anyhow::{anyhow, Result};
use michel_core::{MichelConfig, MichelInstance};
use michel_index::MilliPersistence;
use tauri::CustomMenuItem;
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Wry};

lazy_static! {
    static ref PLUGINS_CONFIG_PATH: PathBuf =
        compute_plugin_config_path().expect("No config path found");
    static ref PLUGINS_FOLDER: PathBuf = compute_plugins_folder().expect("No plugins folder found");
}

const MICHEL_CONFIG_FOLDER: &str = "michel";
//const MICHEL_CONFIG_FILENAME: &str = "config.toml";

mod config;
mod search_bar;
mod settings;

const PLUGINS_CONFIG_FILENAME: &str = "plugins.toml";
const PLUGINS_FOLDER_NAME: &str = "plugins";

fn compute_plugins_folder() -> Result<PathBuf> {
    let path = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME"))
        .map(|path| {
            Path::new(&path)
                .join(MICHEL_CONFIG_FOLDER)
                .join(PLUGINS_FOLDER_NAME)
        })
        .or(Err(anyhow!("no place to find michel config")))?;

    if path.exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(path)
}

fn compute_plugin_config_path() -> Result<PathBuf> {
    let path = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME"))
        .map(|path| {
            Path::new(&path)
                .join(MICHEL_CONFIG_FOLDER)
                .join(PLUGINS_CONFIG_FILENAME)
        })
        .or(Err(anyhow!("no place to find michel config")))?;

    if path.exists() {
        File::create(&path)?;
    }

    Ok(path)
}

fn setup_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(settings)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn handle_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "quit" => process::exit(0),
            "show" => toggle_search_bar_visibility(app),
            "settings" => show_settings(app),
            _ => {}
        }
    }
}

fn show_settings(app: &AppHandle<Wry>) {
    app.get_window("settings")
        .expect("settings window to be present")
        .show()
        .unwrap()
}

fn toggle_search_bar_visibility(app: &AppHandle<Wry>) {
    let window = app.get_window("search-bar").unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
        window.center().unwrap();
        window.set_focus().unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let tray = setup_system_tray();

    let instance = MichelInstance::new(
        MilliPersistence::new()?,
        MichelConfig {
            name: "Michel".to_string(),
            plugins_path: PLUGINS_FOLDER.clone(),
        },
    )
    .await?;

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(handle_tray_event)
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            search_bar::register_search_shortcut(app.handle());

            Ok(())
        })
        .manage(instance)
        .invoke_handler(tauri::generate_handler![
            settings::get_plugins_list,
            settings::run_plugin_index
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    return Ok(());
}
