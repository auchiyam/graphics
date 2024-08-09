use wasmtime::component::bindgen;

bindgen!({
    world: "node",
    path: "../api/wit/node.wit"
});
