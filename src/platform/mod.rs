#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
#[allow(unused)]
pub use web::{run, start_audio_playback};

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused)]
pub use native::{run, start_audio_playback};
