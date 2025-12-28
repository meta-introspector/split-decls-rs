use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct ThreadData { registry_id : Cell < RegistryId > , index : Cell < usize > , }