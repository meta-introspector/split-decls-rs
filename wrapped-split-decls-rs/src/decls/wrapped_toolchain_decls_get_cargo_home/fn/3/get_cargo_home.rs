use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: get_cargo_home");
fn get_cargo_home () -> Option < Utf8PathBuf > { if let Some (path) = env :: var_os ("CARGO_HOME") { return Utf8PathBuf :: try_from (PathBuf :: from (path)) . ok () ; } if let Some (mut path) = home :: home_dir () { path . push (".cargo") ; return Utf8PathBuf :: try_from (path) . ok () ; } None }
}