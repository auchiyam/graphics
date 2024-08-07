//! Singleton struct

use std::{collections::HashMap, path::Path};

use uuid::Uuid;
use wasmtime::{Engine, Func, Instance, Linker, Module, Store, TypedFunc};

/// Dispatcher abstracts the interaction between the plugin wasm
pub struct Dispatcher {
    engine: Engine,
    wasm: HashMap<u128, Wasm>,
}

pub struct Wasm {
    plugin_id: u128,
    module: Module,
    store: Store<()>,
    functions: HashMap<u128, Func>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Func specified did not exist")]
    FuncNotFound,
    #[error("Wasmtime Error: {0}")]
    Wasmtime(#[from] wasmtime::Error),
}

impl Dispatcher {
    pub fn init() -> Dispatcher {
        Self {
            engine: Engine::default(),
            wasm: HashMap::new(),
        }
    }

    pub fn add_plugin(&mut self, wasm: &Path, functions: Vec<String>) -> Result<(), Error> {
        let pid = Uuid::new_v4().as_u128();

        let mut module = Module::from_file(&self.engine, wasm)?;
        let mut store = Store::new(&self.engine, ());
        let linker = Linker::new(&self.engine);
        let instance = linker.instantiate(&mut store, &module)?;
        let mut func_map = HashMap::new();

        for n in functions {
            let fid = Uuid::new_v4().as_u128();
            let f = instance
                .get_func(&mut store, &n)
                .ok_or_else(|| Error::FuncNotFound)?;

            func_map.insert(fid, f);
        }

        let wasm = Wasm {
            plugin_id: pid,
            module,
            store,
            functions: func_map,
        };

        Ok(())
    }
}
