mod plugins;

use anyhow::Result;
use plugins::wasi::PluginInstance;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) type Document = serde_json::Map<String, serde_json::Value>;

pub struct Index {
    pub name: String,
}

pub trait MichelPersistence {
    fn add_document(&self, index: Index, document: Document) -> Result<()>;
    fn search_document(
        &self,
        index: Index,
        query: String,
        limit: Option<u32>,
    ) -> Result<Vec<Document>>;
}

pub struct FsAccess {
    host_path: PathBuf,
    guest_path: String,
    enabled: bool,
    notify: bool,
}

pub struct PluginHostConfig {
    fs_access: Vec<FsAccess>,
}

pub struct CustomPluginConfig;

pub struct PluginConfig {
    host: PluginHostConfig,
    custom: CustomPluginConfig,
}

pub struct PluginInfo {
    name: String,
    description: String,
    version: String,
    icon: Option<String>,
    url: Option<String>,
}

pub struct Plugin {
    instance: Option<PluginInstance>,
    infos: PluginInfo,
    config: PluginConfig,
}

impl Plugin {
    pub async fn load_from_path<T: AsRef<Path>>(path: T) -> Result<Plugin> {
        let instance = PluginInstance::init(path).await?;

        let infos = instance.get_infos().await?;
        Ok(Plugin {
            instance: Some(instance),
            infos: PluginInfo::from(infos),
            config: PluginConfig {
                host: PluginHostConfig { fs_access: vec![] },
                custom: CustomPluginConfig,
            },
        })
    }

    pub fn name(&self) -> String {
        String::from(&self.infos.name)
    }
}

pub struct MichelConfig {
    pub name: String,
    pub plugins_path: PathBuf,
}

pub struct MichelInstance<T: MichelPersistence> {
    persistence: T,
    config: MichelConfig,
    plugins: Vec<Plugin>,
}

impl<T: MichelPersistence> MichelInstance<T> {
    pub async fn new(persistence: T, config: MichelConfig) -> Result<MichelInstance<T>> {
        let mut instance = MichelInstance {
            persistence,
            config,
            plugins: vec![],
        };

        instance.refresh_plugins().await?;

        Ok(instance)
    }

    async fn refresh_plugins(&mut self) -> Result<()> {
        println!(
            "Alors voilà le chemin : {}",
            self.config.plugins_path.to_string_lossy()
        );
        let paths = fs::read_dir(self.config.plugins_path.as_path()).unwrap();

        let mut plugins: Vec<Plugin> = Vec::new();

        for path in paths {
            let plugin_path = path.unwrap().path();
            println!("proutent {}", plugin_path.to_string_lossy());
            let state = Plugin::load_from_path(plugin_path).await?;
            plugins.push(state)
        }

        self.plugins = plugins;

        Ok(())
    }

    pub fn plugins(&self) -> &Vec<Plugin> {
        return &self.plugins;
    }
}
