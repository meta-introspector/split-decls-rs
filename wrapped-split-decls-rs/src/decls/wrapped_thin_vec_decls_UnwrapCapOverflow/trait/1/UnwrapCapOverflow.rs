use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
trait UnwrapCapOverflow < T > { fn unwrap_cap_overflow (self) -> T ; }
}