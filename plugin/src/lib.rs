mod manager;
mod node;

use node::Node;
use wasmtime::{
    component::{Component, Linker},
    Engine, Store,
};

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, "./plugin.wasm")?;

    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let (binding, instance) = Node::instantiate(&mut store, &component, &linker)?;
    let metadata = binding
        .plugin_node_metadata()
        .call_get_metadata(&mut store)?;
    let state = binding.plugin_node_runner().state();
    let state_res = state.call_constructor(&mut store)?;
    let result = state
        .call_add_input(&mut store, state_res, &1u8.to_be_bytes())
        .err()
        .take();
    state.call_run(&mut store, state_res)?;

    Ok(())
}
