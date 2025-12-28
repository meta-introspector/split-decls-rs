use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " See [`DebugStrictAdd`]."] pub trait DebugStrictSub { # [doc = " See [`DebugStrictAdd`]."] fn debug_strict_sub (self , other : Self) -> Self ; }
}