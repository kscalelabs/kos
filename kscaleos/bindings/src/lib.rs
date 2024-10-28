use kscaleos::hello_world as kscaleos_hello_world;
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

#[pyfunction]
pub fn hello_world() {
    kscaleos_hello_world();
}

define_stub_info_gatherer!(stub_info);
