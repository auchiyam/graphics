use std::{collections::HashMap, sync::RwLock};

use crate::MyState;

use self::plugin::node::memory::{Address, Direction, Uuid};
use plugin::node::memory::Host;
use wasmtime::component::{bindgen, Resource, ResourceTable};

pub use self::plugin::node::memory::{HostManager, Manager};

bindgen!({
    world: "node",
    path: "../api/wit/node.wit",
    with: {
        "plugin:node/memory/manager": MemoryManager
    },
    additional_derives: [
        Hash,
        Eq,
        PartialEq,
        Clone,
    ]
});

#[derive(Default)]
pub struct MemoryManager {
    memory: RwLock<HashMap<Address, Vec<u8>>>,
    mapping: RwLock<HashMap<Address, Address>>,
}

#[derive(Default)]
pub struct MemoryHost {
    table: ResourceTable,
}

impl Host for MyState {}

impl HostManager for MyState {
    fn new(&mut self) -> Resource<Manager> {
        self.table.push(MemoryManager::default()).unwrap()
    }

    fn connect_port(
        &mut self,
        self_: Resource<Manager>,
        src: Address,
        dest: Address,
    ) -> Result<(), String> {
        if Direction::Input == src.dir || Direction::Output == dest.dir {
            return Err(
                "Cannot connect address of wrong direction. Source must always be output, Destination must always be input".into(),
            );
        }

        let memory = self.table.get(&self_).unwrap();
        let mut mapping_guard = memory.mapping.write().unwrap();
        mapping_guard.entry(dest).and_modify(|p| *p = src);

        Ok(())
    }

    fn read(&mut self, self_: Resource<Manager>, src: Address) -> Vec<u8> {
        let memory = self.table.get(&self_).unwrap();
        let memory_guard = memory.memory.read().unwrap();
        let mapping_guard = memory.mapping.read().unwrap();

        let real_addr = if let Some(mem) = mapping_guard.get(&src) {
            mem
        } else {
            tracing::warn!(
                id = src.id,
                port = src.port,
                direction = ?src.dir,
                "The address does not point to any inserts"
            );
            return Vec::new();
        };

        memory_guard.get(real_addr).cloned().unwrap_or_default()
    }

    fn write(&mut self, self_: Resource<Manager>, dest: Address, data: Vec<u8>) {
        let memory = self.table.get(&self_).unwrap();
        let mut memory_guard = memory.memory.write().unwrap();
        memory_guard.entry(dest).and_modify(|p| *p = data);
    }

    fn allocate_id(&mut self, _self_: Resource<Manager>) -> Uuid {
        let (left, right) = uuid::Uuid::new_v4().as_u64_pair();
        Uuid { left, right }
    }

    fn flush(&mut self, _self: Resource<Manager>) -> Result<(), String> {
        Ok(())
    }

    fn drop(&mut self, rep: Resource<Manager>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
