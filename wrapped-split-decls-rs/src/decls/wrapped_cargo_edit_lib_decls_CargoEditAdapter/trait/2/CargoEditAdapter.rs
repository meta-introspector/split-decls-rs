use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait CargoEditAdapter : Send + Sync { fn generate_cargo_config (& self , git_adapter : & dyn GitAdapter , cargo_metadata_provider : & dyn CargoMetadataProvider ,) -> Result < String , anyhow :: Error > ; }