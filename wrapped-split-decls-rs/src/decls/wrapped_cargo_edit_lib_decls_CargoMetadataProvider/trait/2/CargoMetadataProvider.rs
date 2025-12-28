use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait CargoMetadataProvider : Send + Sync { fn get_metadata (& self , cargo_toml_path : & std :: path :: Path) -> Result < Box < dyn AnyMetadata > > ; }