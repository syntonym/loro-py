use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = loro_py::stub_info()?;
    println!("{:?}", stub);
    stub.generate()?;
    Ok(())
}
