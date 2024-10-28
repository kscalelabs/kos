use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    println!("Generating stub info...");
    let stub = bindings::stub_info()?;
    println!("Generating stubs...");
    stub.generate()?;
    Ok(())
}
