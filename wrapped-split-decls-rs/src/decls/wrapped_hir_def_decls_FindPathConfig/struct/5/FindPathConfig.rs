use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A wrapper around three booleans"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Copy)] pub struct FindPathConfig { # [doc = " If true, prefer to unconditionally use imports of the `core` and `alloc` crate"] # [doc = " over the std."] pub prefer_no_std : bool , # [doc = " If true, prefer import paths containing a prelude module."] pub prefer_prelude : bool , # [doc = " If true, prefer abs path (starting with `::`) where it is available."] pub prefer_absolute : bool , # [doc = " If true, paths containing `#[unstable]` segments may be returned, but only if if there is no"] # [doc = " stable path. This does not check, whether the item itself that is being imported is `#[unstable]`."] pub allow_unstable : bool , }
}