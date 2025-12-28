use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () > { let cli = Cli :: parse () ; match cli . command { Commands :: Generate { output , sparql_data , all_impls } => { generate_decoupling_traits (& output , sparql_data . as_deref () , all_impls) } Commands :: Report => { show_syscall_report () } Commands :: Trait { category , output } => { generate_specific_trait (& category , & output) } Commands :: Interactive => { run_interactive_mode () } } }
}