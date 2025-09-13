use pyo3::prelude::*;

use test_shakmaty_pgn_to_uci::str_to_uci;

#[pyfunction]
fn str_to_uci2(pgn: &str) -> PyResult<String> {
    Ok(str_to_uci(pgn).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn san_to_uci(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(str_to_uci2, module)?)
}