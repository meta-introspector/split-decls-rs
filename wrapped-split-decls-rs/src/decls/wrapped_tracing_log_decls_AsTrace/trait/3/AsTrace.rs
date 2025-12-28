use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Trait implemented for `log` types that can be converted to a `tracing`"] # [doc = " equivalent."] pub trait AsTrace : crate :: sealed :: Sealed { # [doc = " The `tracing` type that this type can be converted into."] type Trace ; # [doc = " Returns the `tracing` equivalent of `self`."] fn as_trace (& self) -> Self :: Trace ; }
}