use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct OnDrop < F : FnOnce () > (Option < F >) ;
}