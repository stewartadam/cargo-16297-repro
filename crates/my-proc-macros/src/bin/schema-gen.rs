use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Details on the real use case for this file layout:
    //
    // Schemars doesn't support automatically exporting all JsonSchema-derived
    // structs, so I have a proc macro (defined in lib.rs) to introspect the
    // codebase and generate the necessary code to export each JsonSchema struct
    //
    // This binary is created in the proc-macro crate as a small helper to be
    // able to run that macro and output the JSON file.

    // <logic omitted>
    println!("JSON schemas generated!");
    Ok(())
}
