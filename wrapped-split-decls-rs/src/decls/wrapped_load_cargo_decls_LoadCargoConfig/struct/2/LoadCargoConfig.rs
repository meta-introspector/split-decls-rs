use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct LoadCargoConfig { pub load_out_dirs_from_check : bool , pub with_proc_macro_server : ProcMacroServerChoice , pub prefill_caches : bool , }
}