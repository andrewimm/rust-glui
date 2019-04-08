pub mod geometry;
pub mod node;
pub mod program;
pub mod texture;
pub mod uniforms;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
pub mod gli;

#[cfg(target_arch = "wasm32")]
#[path = "webgl/mod.rs"]
pub mod gli;