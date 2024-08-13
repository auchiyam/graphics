mod manager;
pub(crate) mod node;

use manager::PluginManager;
use node::Node;
use wasmtime::{
    component::{Component, Linker, ResourceTable},
    Engine, Store,
};

#[derive(Default)]
struct MyState {
    table: ResourceTable,
}

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new();
    manager.load_component("./adder.wasm")?;

    Ok(())
}
