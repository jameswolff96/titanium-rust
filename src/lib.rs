use std::error::Error;

pub mod generator;
pub mod parser;

pub fn compile_source(source: &str) -> Result<String, Box<dyn Error>> {
    //Parse
    let ast: parser::Program = syn::parse_str(source)?;

    // Generate
    let output_code = generator::generate(ast).map_err(|e| -> Box<dyn Error> { e.into() })?;
    
    Ok(output_code)
}
