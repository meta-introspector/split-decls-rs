use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ThreadData { registry_id : Cell < RegistryId > , index : Cell < usize > , }
}