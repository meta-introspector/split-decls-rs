use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: fix_windows_verbatim_for_gcc");
# [cfg (not (windows))] pub fn fix_windows_verbatim_for_gcc (p : & Path) -> PathBuf { p . to_path_buf () }
}