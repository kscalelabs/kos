use kscaleos::hello_world as kscaleos_hello_world;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult};
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[gen_stub_pyfunction]
#[pyfunction]
pub fn hello_world() -> PyResult<()> {
    kscaleos_hello_world();
    Ok(())
}

#[pymodule]
fn bindings(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?).unwrap();
    Ok(())
}

define_stub_info_gatherer!(stub_info);
