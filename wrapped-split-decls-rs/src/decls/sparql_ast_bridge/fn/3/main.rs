use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () > { let cli = Cli :: parse () ; match cli . command { Commands :: Generate { kb , top_n , output , complexity } => { generate_probes_from_sparql (& kb , top_n , & output , complexity) } Commands :: Apply { dir , probes , verbose } => { apply_sparql_probes (& dir , & probes , verbose) } Commands :: Config { output } => { generate_sparql_config (& output) } Commands :: Interactive { kb , dir } => { run_interactive_mode (& kb , & dir) } } }
}