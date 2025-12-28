use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait ConfigTrait { fn load () -> Self ; fn get_rules (& self) -> & Vec < Rule > ; }
}