mod client_module;
mod engine2_module;
mod offsets;

pub use client_module::cs2_dumper::schemas::client_dll as client;
pub use engine2_module::cs2_dumper::schemas::engine2_dll as engine;
pub use offsets::cs2_dumper::offsets::{client_dll, engine2_dll};
