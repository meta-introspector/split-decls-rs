use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn toolchain_channel (db : & dyn RootQueryDb , krate : Crate) -> Option < ReleaseChannel > { krate . workspace_data (db) . toolchain . as_ref () . and_then (| v | ReleaseChannel :: from_str (& v . pre)) }
}