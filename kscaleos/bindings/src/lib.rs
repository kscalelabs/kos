use kscaleos::hello_world as kscaleos_hello_world;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult};
use pyo3_stub_gen::define_stub_info_gatherer;

#[pyfunction]
pub fn hello_world() -> PyResult<()> {
    kscaleos_hello_world();
    Ok(())
}

#[pymodule]
fn bindings(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(hello_world))?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
