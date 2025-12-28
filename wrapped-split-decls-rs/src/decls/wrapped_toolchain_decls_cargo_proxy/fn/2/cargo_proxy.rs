use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Looks up the binary in the cargo home directory if it exists."] fn cargo_proxy (executable_name : & str) -> Option < Utf8PathBuf > { let mut path = get_cargo_home () ? ; path . push ("bin") ; path . push (executable_name) ; probe_for_binary (path) }
}