use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
trait IntoTokenStream { fn into_ts (self) -> TokenStream ; }
}