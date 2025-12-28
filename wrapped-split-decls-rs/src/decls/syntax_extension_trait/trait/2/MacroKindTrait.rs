use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait MacroKindTrait < DRT : OpaqueDeriveResolution + 'static > : Send + Sync + 'static { fn get_name (& self) -> String ; }
}