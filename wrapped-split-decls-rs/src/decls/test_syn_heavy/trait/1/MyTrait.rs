use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
trait MyTrait { fn method (& self) -> String ; }
}