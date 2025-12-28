use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A folder for `#[cfg]`-based conditional compilation."] # [doc = ""] # [doc = " This is a \"folder\" in the sense of `rustc_ast::visit::Folder`."] # [doc = ""] # [doc = " `strip_unconfigured_items` is the main entry point."] pub struct StripUnconfigured < 'a > { pub sess : & 'a rustc_session :: Session , pub features : Option < & 'a rustc_feature :: Features > , # [doc = " If `true`, `#[cfg]` attributes are not stripped."] pub keep_mode : KeepCfg , pub config_tokens : bool , pub lint_node_id : NodeId , }