use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: run_ecosystem_scan_mode");
fn run_ecosystem_scan_mode (verbose : bool , dry_run : bool , base_path : & Path , recursive : bool , global_config : & SplitDeclsConfig ,) -> Result < () > { ecosystem_processor :: process_ecosystem (verbose , dry_run , base_path , recursive , global_config) }
}