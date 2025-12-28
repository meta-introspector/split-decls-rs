use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Returns a `rustlib` path for this particular target, relative to the provided sysroot."] # [doc = ""] # [doc = " For example: `target_sysroot_path(\"/usr\", \"x86_64-unknown-linux-gnu\")` =>"] # [doc = " `\"lib*/rustlib/x86_64-unknown-linux-gnu\"`."] pub fn relative_target_rustlib_path (sysroot : & Path , target_triple : & str) -> PathBuf { let libdir = find_relative_libdir (sysroot) ; Path :: new (libdir . as_ref ()) . join (RUST_LIB_DIR) . join (target_triple) }
}