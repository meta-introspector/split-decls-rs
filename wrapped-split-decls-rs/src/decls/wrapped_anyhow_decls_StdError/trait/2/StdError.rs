use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [cfg (all (not (feature = "std") , anyhow_no_core_error))] trait StdError : Debug + Display { fn source (& self) -> Option < & (dyn StdError + 'static) > { None } }
}