mod buffer;

pub use buffer::{Buffer, ToBuffer};

/// Node keeps track of the user defined logic
///
/// Node is the smallest object used in the scripts that holds user defined
/// logic that transforms arbitrary input buffer into arbitrary output buffer
/// For safety reasons, the node definition must be pure confined in .wasm file
pub struct Node {
    node_id: u128,
    dispatch_id: u128,
    
}

impl Node {
    pub fn generate_node() -> Node {
        todo!();
    }

    pub fn transform(&self, input: Buffer) -> Buffer {
        todo!();
    }
}
