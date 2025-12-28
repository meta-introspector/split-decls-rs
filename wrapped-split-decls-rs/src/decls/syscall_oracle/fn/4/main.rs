use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: main");
fn main () -> Result < () > { let cli = Cli :: parse () ; match cli . command { Commands :: Generate { output } => { generate_oracle_code (& output) } Commands :: Transform { dir , output , mock , safety , dao } => { transform_directory (& dir , & output , mock , & safety , dao) } Commands :: Config { output } => { generate_config (& output) } Commands :: Interactive { dir } => { run_interactive_mode (& dir) } Commands :: Analyze { dir , generate } => { analyze_syscalls (& dir , generate) } } }
}