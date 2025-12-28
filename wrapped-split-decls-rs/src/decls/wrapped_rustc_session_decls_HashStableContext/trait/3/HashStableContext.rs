use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : rustc_ast :: HashStableContext + rustc_hir :: HashStableContext { }
}