use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn generate_wrapped_main (binary_name : & str) -> Result < String > { let wrapped_crate_name = format ! ("wrapped_{}" , binary_name . replace ("-" , "_")) ; Ok (format ! (r#"// Generated wrapped main.rs for {}
// This calls the main function from the wrapped library

use anyhow::Result;

fn main() -> Result<()> {{
    // Import and call the wrapped main function
    {}::main()
}}
"# , binary_name , wrapped_crate_name)) }
}