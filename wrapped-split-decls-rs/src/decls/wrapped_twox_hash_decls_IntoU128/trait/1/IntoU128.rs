use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [allow (dead_code , reason = "Too lazy to cfg-gate these")] trait IntoU128 { fn into_u128 (self) -> u128 ; }
}