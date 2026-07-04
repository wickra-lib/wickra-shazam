//! Python bindings for `wickra-shazam`, exposed under the `wickra_shazam`
//! package.
//!
//! Thin glue over the shazam core's data-driven surface: build a [`Shazam`] from
//! a spec JSON, drive it with a command JSON and read back the response JSON.
//! The same command protocol crosses every binding, so a Python front-end drives
//! the exact same core as the native CLI.

// PyO3 protocol methods take `self` by value/ref regardless of use.
#![allow(clippy::needless_pass_by_value)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use shazam_core::Shazam;

/// A shazam instance driven by JSON commands.
///
/// `unsendable`: the shazam holds a stateful fingerprint index and streaming
/// evaluators, so a handle is bound to the thread that created it.
#[pyclass(name = "Shazam", unsendable)]
struct PyShazam {
    inner: Shazam,
}

#[pymethods]
impl PyShazam {
    /// Build a shazam from a spec JSON string.
    #[new]
    fn new(spec_json: &str) -> PyResult<Self> {
        Shazam::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// Apply a command JSON and return the resulting response JSON.
    fn command(&mut self, cmd_json: &str) -> PyResult<String> {
        self.inner
            .command_json(cmd_json)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// The library version.
    #[staticmethod]
    fn version() -> &'static str {
        Shazam::version()
    }
}

/// The native module (`wickra_shazam._wickra_shazam`).
#[pymodule]
fn _wickra_shazam(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyShazam>()?;
    Ok(())
}
