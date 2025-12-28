use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () > { let cli = Cli :: parse () ; match cli . command { Commands :: Reflect { dir , probes , output , verbose } => { reflect_directory (& dir , probes . as_deref () , output . as_deref () , verbose) } Commands :: GenProbes { output } => { generate_example_probes (& output) } Commands :: ListTypes => { list_probe_types () ; Ok (()) } Commands :: Analyze { dir , generate } => { analyze_directory (& dir , generate) } } }
}