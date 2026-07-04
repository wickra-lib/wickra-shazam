//! WebAssembly bindings for `wickra-shazam` (wasm-bindgen).
//!
//! The data-driven history-fingerprint match core, compiled to WebAssembly for
//! the browser: build a `Shazam` from a spec JSON, drive it with a command JSON
//! and read back the response JSON. The same command protocol crosses every
//! binding.
//!
//! The `parallel` feature of the core is disabled here: rayon's thread pool is
//! not available in a browser sandbox, so indexing and matching run
//! sequentially — which is byte-identical to the parallel path, the exact
//! cross-language golden check.

use wasm_bindgen::prelude::*;

use shazam_core::Shazam as CoreShazam;

/// A shazam instance driven by JSON commands.
#[wasm_bindgen]
pub struct Shazam {
    inner: CoreShazam,
}

#[wasm_bindgen]
impl Shazam {
    /// Build a shazam from a spec JSON string.
    #[wasm_bindgen(constructor)]
    pub fn new(spec_json: &str) -> Result<Shazam, JsError> {
        CoreShazam::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Apply a command JSON and return the resulting response JSON.
    pub fn command(&mut self, cmd_json: &str) -> Result<String, JsError> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        CoreShazam::version().to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    CoreShazam::version().to_string()
}
