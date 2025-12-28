use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn format_generated_rust_files (output_dir : & Path , verbose : bool) -> Result < () > { if verbose { println ! ("DEBUG: Skipping rustfmt formatting in {} (disabled due to edition issues)" , output_dir . display ()) ; } Ok (()) }
}