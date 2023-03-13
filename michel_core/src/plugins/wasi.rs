use anyhow::Result;
use host::WasiCtx;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use wasi_cap_std_sync::dir::Dir;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::component::bindgen;
use wasmtime::component::{Component, Linker};
use wasmtime::{AsContextMut, Config, Engine, Store};

bindgen!({
    world: "plugin",
    path: "../wit",
    async: true
});

use crate::{CustomPluginConfig, FsAccess, PluginConfig, PluginHostConfig, PluginInfo};
use async_trait::async_trait;

pub struct MichelApiForPlugins;

impl types::Types for MichelApiForPlugins {}

#[async_trait]
impl michel_api::MichelApi for MichelApiForPlugins {
    async fn hi(&mut self, name: String) -> Result<String> {
        todo!()
    }

    async fn send_entry_for_input(
        &mut self,
        input: String,
        entries: Vec<types::Entry>,
    ) -> Result<()> {
        todo!()
    }

    async fn new_document_for_index(
        &mut self,
        index: String,
        document: types::Document,
    ) -> Result<()> {
        todo!()
    }

    async fn search_in_index(
        &mut self,
        index: String,
        query: String,
    ) -> Result<Vec<types::Document>> {
        todo!()
    }

    async fn init_index(&mut self, index: String) -> Result<()> {
        todo!()
    }
}

fn wasi_dir_from_path<P: AsRef<Path>>(path: P) -> Dir {
    let file = std::fs::File::open(path).unwrap();

    Dir::from_cap_std(wasi_cap_std_sync::Dir::from_std_file(file))
}

pub struct Ctx {
    wasi: WasiCtx,
    michel: MichelApiForPlugins,
}

impl Ctx {
    async fn new() -> Ctx {
        let mut wasi = WasiCtxBuilder::new()
            .inherit_stderr()
            .inherit_stdin()
            .build();

        let search_path = wasi_dir_from_path("/home/yohann/devs");

        wasi.push_preopened_dir(Box::new(search_path), "/home/yohann/devs")
            .unwrap();

        Ctx {
            wasi,
            michel: MichelApiForPlugins,
        }
    }
}

pub struct PluginInstance {
    pub bindings: Michel,
    pub store: Arc<Mutex<Store<Ctx>>>,
}

impl PluginInstance {
    pub async fn get_infos(&self) -> Result<types::PluginInfo> {
        let mut guard = self.store.lock().expect("getting store content");
        let store = guard.as_context_mut();

        self.bindings.plugin_api.call_info(store).await
    }

    pub async fn init<T: AsRef<Path>>(path: T) -> Result<PluginInstance> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);

        // Modules can be compiled through either the text or binary format
        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, path)?;
        let mut linker: Linker<Ctx> = Linker::new(&engine);
        host::add_to_linker(&mut linker, |ctx| &mut ctx.wasi)?;
        Michel::add_to_linker(&mut linker, |ctx| &mut ctx.michel)?;

        let mut store = Store::new(&engine, Ctx::new().await);
        let (bindings, _) = Michel::instantiate_async(&mut store, &component, &linker).await?;

        Ok(PluginInstance {
            bindings,
            store: Arc::new(Mutex::new(store)),
        })
    }
}

impl From<types::PluginInfo> for PluginInfo {
    fn from(value: types::PluginInfo) -> Self {
        PluginInfo {
            name: value.name,
            description: value.description,
            version: value.version,
            icon: value.icon,
            url: value.url,
        }
    }
}

impl From<types::PluginConfigResult> for PluginConfig {
    fn from(value: types::PluginConfigResult) -> Self {
        Self {
            host: PluginHostConfig {
                fs_access: value.fs_access.into_iter().map(FsAccess::from).collect(),
            },
            custom: CustomPluginConfig,
        }
    }
}

impl From<types::FsAccessResult> for FsAccess {
    fn from(value: types::FsAccessResult) -> Self {
        Self {
            host_path: PathBuf::from(value.host_path),
            guest_path: String::new(),
            enabled: false,
            notify: false,
        }
    }
}
