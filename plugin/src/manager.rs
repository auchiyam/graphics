use std::{collections::HashMap, path::Path};

use wasmtime::{
    component::{Component, Instance, Linker, ResourceTable},
    Engine, Store,
};

use crate::{node::Node, MyState};

pub struct PluginManager {
    engine: Engine,
    store: Store<MyState>,
    plugins: HashMap<u128, Plugin>,
}

pub struct Plugin {
    component: Component,
    linker: Linker<MyState>,
    instance: Instance,
    node: Node,
}

impl PluginManager {
    pub fn new() -> Self {
        let engine = Engine::default();

        let state = MyState {
            table: ResourceTable::default(),
        };
        let store = Store::new(&engine, state);
        let plugins = HashMap::new();

        Self {
            engine,
            store,
            plugins,
        }
    }

    pub fn load_component(&mut self, path: impl AsRef<Path>) -> Result<(), anyhow::Error> {
        let component = Component::from_file(&self.engine, path).unwrap();
        let mut linker = Linker::new(&self.engine);
        Node::add_to_linker(&mut linker, |state: &mut MyState| state);
        let (node, instance) = Node::instantiate(&mut self.store, &component, &linker)?;

        let plugin = Plugin {
            component,
            linker,
            instance,
            node,
        };

        Ok(())
    }
}
