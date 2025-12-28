use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A lock which only uses synchronization if `might_be_dyn_thread_safe` is true."] # [doc = " It implements `DynSend` and `DynSync` instead of the typical `Send` and `Sync`."] pub struct Lock < T > { # [doc = " Indicates if synchronization is used via `mode_union.sync` if it's `Sync`, or if a"] # [doc = " not thread safe cell is used via `mode_union.no_sync` if it's `NoSync`."] # [doc = " This is set on initialization and never changed."] mode : Mode , mode_union : ModeUnion , data : UnsafeCell < T > , }
}