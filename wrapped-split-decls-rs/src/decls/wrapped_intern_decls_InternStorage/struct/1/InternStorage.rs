use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct InternStorage < T : ? Sized > { map : OnceLock < InternMap < T > > , }
}