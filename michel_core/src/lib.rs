pub mod persistence;
mod plugins;

use crate::persistence::MichelPersistence;
use anyhow::{anyhow, Result};
use plugins::wasi::PluginInstance;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use wasmtime::AsContextMut;

pub struct FsAccess {
    host_path: PathBuf,
    guest_path: String,
    enabled: bool,
    notify: bool,
}

pub struct PluginHostConfig {
    enabled: bool,
    fs_access: Vec<FsAccess>,
}

pub struct CustomPluginConfig;

pub struct PluginConfig {
    host: PluginHostConfig,
    custom: CustomPluginConfig,
}

pub struct PluginInfo {
    identifier: String,
    name: String,
    description: String,
    version: String,
    icon: Option<String>,
    url: Option<String>,
}

pub struct Plugin<P: MichelPersistence> {
    instance: PluginInstance<P>,
    infos: PluginInfo,
    config: PluginConfig,
}

impl<P: MichelPersistence> Plugin<P> {
    pub async fn load_from_path<T: AsRef<Path>>(
        path: T,
        persistence: Arc<Mutex<P>>,
    ) -> Result<Plugin<P>> {
        let instance = PluginInstance::init(path, persistence.clone()).await?;

        let infos = instance.get_infos().await?;
        Ok(Plugin {
            instance,
            infos: PluginInfo::from(infos),
            config: PluginConfig {
                host: PluginHostConfig {
                    fs_access: vec![],
                    enabled: true,
                },
                custom: CustomPluginConfig,
            },
        })
    }

    pub async fn index(&self) -> Result<()> {
        let mut guard = self.instance.store.lock().await;
        let store = guard.as_context_mut();
        return match self
            .instance
            .bindings
            .plugin_api()
            .call_index(store)
            .await?
        {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("run index")),
        };
    }

    pub fn identifier(&self) -> String {
        String::from(&self.infos.identifier)
    }
    pub fn name(&self) -> String {
        String::from(&self.infos.name)
    }
    pub fn description(&self) -> String {
        String::from(&self.infos.description)
    }
    pub fn can_index(&self) -> bool {
        true
    }
    pub fn enabled(&self) -> bool {
        self.config.host.enabled
    }
}

pub struct MichelConfig {
    pub name: String,
    pub plugins_path: PathBuf,
}

pub struct MichelInstance<P: MichelPersistence> {
    persistence: Arc<Mutex<P>>,
    config: MichelConfig,
    plugins: Vec<Plugin<P>>,
}

impl<P: MichelPersistence> MichelInstance<P> {
    pub async fn new(persistence: P, config: MichelConfig) -> Result<MichelInstance<P>> {
        let mut instance = MichelInstance {
            persistence: Arc::new(Mutex::new(persistence)),
            config,
            plugins: vec![],
        };

        instance.refresh_plugins().await?;

        Ok(instance)
    }

    async fn refresh_plugins(&mut self) -> Result<()> {
        let paths = fs::read_dir(self.config.plugins_path.as_path()).unwrap();

        let mut plugins: Vec<Plugin<P>> = Vec::new();

        for path in paths {
            let plugin_path = path?.path();

            let state = Plugin::load_from_path(plugin_path, self.persistence.clone()).await?;
            plugins.push(state)
        }

        self.plugins = plugins;

        Ok(())
    }

    pub fn plugins(&self) -> &Vec<Plugin<P>> {
        return &self.plugins;
    }

    pub fn plugin(&self, identifier: String) -> Option<&Plugin<P>> {
        self.plugins
            .iter()
            .find(|plugin| plugin.infos.identifier.eq(&identifier))
    }
}
